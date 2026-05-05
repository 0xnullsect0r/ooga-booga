use crate::ast::*;
use crate::error::{OogaError, OogaResult, Span};
use crate::lexer::{Spanned, Token};

pub struct Parser {
    tokens: Vec<Spanned>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Spanned>) -> Self {
        Parser { tokens, pos: 0 }
    }

    // ── Token navigation ──────────────────────────────────────────────────

    fn peek(&self) -> &Token {
        &self.tokens[self.pos].token
    }

    fn peek_span(&self) -> Span {
        self.tokens[self.pos].span.clone()
    }

    fn peek2(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1).map(|s| &s.token)
    }

    fn advance(&mut self) -> &Spanned {
        let s = &self.tokens[self.pos];
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        s
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    /// Skip any leading newlines (blank lines between statements).
    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline) {
            self.advance();
        }
    }

    /// Consume a mandatory Newline (statement terminator).
    fn expect_newline(&mut self) -> OogaResult<()> {
        match self.peek() {
            Token::Newline | Token::Eof => {
                if matches!(self.peek(), Token::Newline) {
                    self.advance();
                }
                Ok(())
            }
            _ => Err(OogaError::parse(
                self.peek_span(),
                format!(
                    "CAVE EXPECTED LINE BREAK BUT GOT '{:?}'. ONE THING PER ROCK SLAB.",
                    self.peek()
                ),
            )),
        }
    }

    /// Consume a token of exact type, error if mismatch.
    fn expect(&mut self, expected: &Token) -> OogaResult<Span> {
        if self.peek() == expected {
            let span = self.peek_span();
            self.advance();
            Ok(span)
        } else {
            Err(OogaError::parse(
                self.peek_span(),
                format!(
                    "CAVE EXPECTED {:?} BUT FOUND {:?}. GRAMMAR ROCK BROKEN.",
                    expected,
                    self.peek()
                ),
            ))
        }
    }

    // ── Top-level ─────────────────────────────────────────────────────────

    pub fn parse_program(&mut self) -> OogaResult<Program> {
        let mut statements = Vec::new();
        self.skip_newlines();
        while !self.is_at_end() {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.skip_newlines();
        }
        Ok(Program { statements })
    }

    // ── Statements ────────────────────────────────────────────────────────

    fn parse_statement(&mut self) -> OogaResult<Statement> {
        match self.peek().clone() {
            Token::Ooga => self.parse_var_decl(),
            Token::Say => self.parse_say(),
            Token::Hear => self.parse_hear(),
            Token::Iff => self.parse_if(),
            Token::Ugga => self.parse_loop(),
            Token::Stop => {
                let span = self.peek_span();
                self.advance();
                self.expect_newline()?;
                Ok(Statement::Break { span })
            }
            Token::Skip => {
                let span = self.peek_span();
                self.advance();
                self.expect_newline()?;
                Ok(Statement::Continue { span })
            }
            Token::Magic => self.parse_func_def(),
            Token::Giveback => self.parse_return(),
            Token::Ident(_) => self.parse_ident_stmt(),
            other => Err(OogaError::parse(
                self.peek_span(),
                format!(
                    "CAVE NO UNDERSTAND STATEMENT STARTING WITH {:?}. UGH.",
                    other
                ),
            )),
        }
    }

    fn parse_var_decl(&mut self) -> OogaResult<Statement> {
        let span = self.peek_span();
        self.advance(); // OOGA
        let name = self.expect_ident("OOGA NEEDS NAME AFTER IT. LIKE: OOGA x")?;
        let initializer = if matches!(self.peek(), Token::Be) {
            self.advance(); // BE
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect_newline()?;
        Ok(Statement::VarDecl {
            name,
            initializer,
            span,
        })
    }

    fn parse_say(&mut self) -> OogaResult<Statement> {
        let span = self.peek_span();
        self.advance(); // SAY
        let value = self.parse_expr()?;
        self.expect_newline()?;
        Ok(Statement::Say { value, span })
    }

    fn parse_hear(&mut self) -> OogaResult<Statement> {
        let span = self.peek_span();
        self.advance(); // HEAR
        let name = self.expect_ident("HEAR NEEDS VARIABLE NAME. LIKE: HEAR x")?;
        self.expect_newline()?;
        Ok(Statement::Hear { name, span })
    }

    fn parse_if(&mut self) -> OogaResult<Statement> {
        let span = self.peek_span();
        self.advance(); // IFF
        let condition = self.parse_expr()?;
        self.expect_newline()?;
        let then_block = self.parse_block()?;

        let mut else_ifs = Vec::new();
        let mut else_block = None;

        // NOPE IFF ... (else-if) or NOPE (else)
        while matches!(self.peek(), Token::Nope) {
            let nope_span = self.peek_span();
            self.advance(); // NOPE
            if matches!(self.peek(), Token::Iff) {
                self.advance(); // IFF
                let cond = self.parse_expr()?;
                self.expect_newline()?;
                let block = self.parse_block()?;
                else_ifs.push((cond, block));
            } else {
                // Plain NOPE = else
                self.expect_newline()?;
                let block = self.parse_block()?;
                else_block = Some(block);
                // Avoid unused warning
                let _ = nope_span;
                break;
            }
        }

        self.expect(&Token::Ugha)?;
        self.expect_newline()?;
        Ok(Statement::If {
            condition,
            then_block,
            else_ifs,
            else_block,
            span,
        })
    }

    fn parse_loop(&mut self) -> OogaResult<Statement> {
        let span = self.peek_span();
        self.advance(); // UGGA
        match self.peek().clone() {
            Token::While => {
                self.advance(); // WHILE
                let condition = self.parse_expr()?;
                self.expect_newline()?;
                let body = self.parse_block()?;
                self.expect(&Token::Ugha)?;
                self.expect_newline()?;
                Ok(Statement::While {
                    condition,
                    body,
                    span,
                })
            }
            Token::Do => {
                self.advance(); // DO
                self.expect_newline()?;
                let body = self.parse_block()?;
                self.expect(&Token::Ugha)?;
                self.expect_newline()?;
                Ok(Statement::Loop { body, span })
            }
            other => Err(OogaError::parse(
                self.peek_span(),
                format!(
                    "UGGA NEEDS WHILE OR DO AFTER IT. FOUND {:?} INSTEAD. CAVE LOGIC FAIL.",
                    other
                ),
            )),
        }
    }

    fn parse_func_def(&mut self) -> OogaResult<Statement> {
        let span = self.peek_span();
        self.advance(); // MAGIC
        let name = self.expect_ident("MAGIC NEEDS FUNCTION NAME. LIKE: MAGIC fib(n)")?;
        self.expect(&Token::LParen)?;
        let params = self.parse_param_list()?;
        self.expect(&Token::RParen)?;
        self.expect_newline()?;
        let body = self.parse_block()?;
        self.expect(&Token::Ugha)?;
        self.expect_newline()?;
        Ok(Statement::FuncDef {
            name,
            params,
            body,
            span,
        })
    }

    fn parse_param_list(&mut self) -> OogaResult<Vec<String>> {
        let mut params = Vec::new();
        if matches!(self.peek(), Token::RParen) {
            return Ok(params);
        }
        params.push(self.expect_ident("EXPECTED PARAMETER NAME IN FUNCTION DEFINITION")?);
        while matches!(self.peek(), Token::Comma) {
            self.advance(); // ,
            params.push(self.expect_ident("EXPECTED PARAMETER NAME AFTER COMMA")?);
        }
        Ok(params)
    }

    fn parse_return(&mut self) -> OogaResult<Statement> {
        let span = self.peek_span();
        self.advance(); // GIVEBACK
        let value = if matches!(self.peek(), Token::Newline | Token::Eof) {
            None
        } else {
            Some(self.parse_expr()?)
        };
        self.expect_newline()?;
        Ok(Statement::Return { value, span })
    }

    /// Parse an identifier-led statement: either assignment or function call.
    fn parse_ident_stmt(&mut self) -> OogaResult<Statement> {
        let span = self.peek_span();
        let name = match self.advance().token.clone() {
            Token::Ident(n) => n,
            _ => unreachable!(),
        };

        // name GETS expr  →  assignment
        if matches!(self.peek(), Token::Gets) {
            self.advance(); // GETS
            let value = self.parse_expr()?;
            self.expect_newline()?;
            return Ok(Statement::Assign { name, value, span });
        }

        // name(args)  →  function call as statement
        if matches!(self.peek(), Token::LParen) {
            self.advance(); // (
            let args = self.parse_arg_list()?;
            self.expect(&Token::RParen)?;
            self.expect_newline()?;
            return Ok(Statement::ExprStmt {
                expr: Expr::FuncCall { name, args, span: span.clone() },
                span,
            });
        }

        Err(OogaError::parse(
            span,
            format!(
                "CAVE EXPECTED GETS OR ( AFTER NAME '{}'. WHAT IS THIS? UGH.",
                name
            ),
        ))
    }

    /// Parse a block of statements terminated by UGHA or NOPE (not consumed).
    fn parse_block(&mut self) -> OogaResult<Vec<Statement>> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !matches!(self.peek(), Token::Ugha | Token::Nope | Token::Eof) {
            stmts.push(self.parse_statement()?);
            self.skip_newlines();
        }
        Ok(stmts)
    }

    fn expect_ident(&mut self, hint: &str) -> OogaResult<String> {
        match self.peek().clone() {
            Token::Ident(name) => {
                self.advance();
                Ok(name)
            }
            _ => Err(OogaError::parse(
                self.peek_span(),
                format!("EXPECTED IDENTIFIER. {}", hint),
            )),
        }
    }

    // ── Expression parsing (recursive descent, precedence climbing) ────────

    fn parse_expr(&mut self) -> OogaResult<Expr> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> OogaResult<Expr> {
        let mut left = self.parse_and()?;
        while matches!(self.peek(), Token::Or) {
            let span = self.peek_span();
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinOp {
                op: BinOp::Or,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> OogaResult<Expr> {
        let mut left = self.parse_not()?;
        while matches!(self.peek(), Token::And) {
            let span = self.peek_span();
            self.advance();
            let right = self.parse_not()?;
            left = Expr::BinOp {
                op: BinOp::And,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_not(&mut self) -> OogaResult<Expr> {
        if matches!(self.peek(), Token::Not) {
            let span = self.peek_span();
            self.advance();
            let operand = self.parse_not()?;
            return Ok(Expr::UnaryOp {
                op: UnaryOp::Not,
                operand: Box::new(operand),
                span,
            });
        }
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> OogaResult<Expr> {
        let left = self.parse_additive()?;
        let span = self.peek_span();
        let op = match self.peek() {
            Token::Is => BinOp::Is,
            Token::Isnt => BinOp::Isnt,
            Token::Biggr => {
                // peek ahead: BIGGR IS → >=
                if matches!(self.peek2(), Some(Token::Is)) {
                    self.advance(); // BIGGR
                    self.advance(); // IS
                    let right = self.parse_additive()?;
                    return Ok(Expr::BinOp {
                        op: BinOp::BiggrIs,
                        left: Box::new(left),
                        right: Box::new(right),
                        span,
                    });
                }
                BinOp::Biggr
            }
            Token::Smallr => {
                // peek ahead: SMALLR IS → <=
                if matches!(self.peek2(), Some(Token::Is)) {
                    self.advance(); // SMALLR
                    self.advance(); // IS
                    let right = self.parse_additive()?;
                    return Ok(Expr::BinOp {
                        op: BinOp::SmallrIs,
                        left: Box::new(left),
                        right: Box::new(right),
                        span,
                    });
                }
                BinOp::Smallr
            }
            _ => return Ok(left),
        };
        self.advance(); // consume operator
        let right = self.parse_additive()?;
        Ok(Expr::BinOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
            span,
        })
    }

    fn parse_additive(&mut self) -> OogaResult<Expr> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let span = self.peek_span();
            let op = match self.peek() {
                Token::Plus => BinOp::Plus,
                Token::Minus => BinOp::Minus,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> OogaResult<Expr> {
        let mut left = self.parse_unary()?;
        loop {
            let span = self.peek_span();
            let op = match self.peek() {
                Token::Times => BinOp::Times,
                Token::Divvy => BinOp::Divvy,
                Token::Mod => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> OogaResult<Expr> {
        if matches!(self.peek(), Token::Minus) {
            let span = self.peek_span();
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expr::UnaryOp {
                op: UnaryOp::Neg,
                operand: Box::new(operand),
                span,
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> OogaResult<Expr> {
        let span = self.peek_span();
        match self.peek().clone() {
            Token::Int(v) => {
                self.advance();
                Ok(Expr::Literal {
                    value: Literal::Int(v),
                    span,
                })
            }
            Token::Float(v) => {
                self.advance();
                Ok(Expr::Literal {
                    value: Literal::Float(v),
                    span,
                })
            }
            Token::Str(v) => {
                self.advance();
                Ok(Expr::Literal {
                    value: Literal::Str(v),
                    span,
                })
            }
            Token::Yeah => {
                self.advance();
                Ok(Expr::Literal {
                    value: Literal::Bool(true),
                    span,
                })
            }
            Token::Nah => {
                self.advance();
                Ok(Expr::Literal {
                    value: Literal::Bool(false),
                    span,
                })
            }
            Token::Void => {
                self.advance();
                Ok(Expr::Literal {
                    value: Literal::Void,
                    span,
                })
            }
            Token::LParen => {
                self.advance(); // (
                let inner = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(inner)
            }
            Token::Ident(name) => {
                self.advance();
                // Is this a function call?
                if matches!(self.peek(), Token::LParen) {
                    self.advance(); // (
                    let args = self.parse_arg_list()?;
                    self.expect(&Token::RParen)?;
                    Ok(Expr::FuncCall { name, args, span })
                } else {
                    Ok(Expr::Ident { name, span })
                }
            }
            other => Err(OogaError::parse(
                span,
                format!(
                    "EXPECTED EXPRESSION BUT GOT {:?}. CAVE BRAIN HURT.",
                    other
                ),
            )),
        }
    }

    fn parse_arg_list(&mut self) -> OogaResult<Vec<Expr>> {
        let mut args = Vec::new();
        if matches!(self.peek(), Token::RParen) {
            return Ok(args);
        }
        args.push(self.parse_expr()?);
        while matches!(self.peek(), Token::Comma) {
            self.advance(); // ,
            args.push(self.parse_expr()?);
        }
        Ok(args)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(src: &str) -> Program {
        let tokens = Lexer::new(src).tokenise().expect("lex failed");
        Parser::new(tokens)
            .parse_program()
            .expect("parse failed")
    }

    #[test]
    fn test_var_decl_no_init() {
        let prog = parse("OOGA x");
        assert_eq!(prog.statements.len(), 1);
        matches!(
            &prog.statements[0],
            Statement::VarDecl { name, initializer: None, .. } if name == "x"
        );
    }

    #[test]
    fn test_var_decl_with_init() {
        let prog = parse("OOGA count BE 0");
        match &prog.statements[0] {
            Statement::VarDecl { name, initializer: Some(_), .. } => {
                assert_eq!(name, "count");
            }
            _ => panic!("expected VarDecl"),
        }
    }

    #[test]
    fn test_assignment() {
        let prog = parse("x GETS 42");
        matches!(&prog.statements[0], Statement::Assign { name, .. } if name == "x");
    }

    #[test]
    fn test_say() {
        let prog = parse(r#"SAY "hello""#);
        assert!(matches!(&prog.statements[0], Statement::Say { .. }));
    }

    #[test]
    fn test_if_else() {
        let prog = parse("IFF x IS 1\nSAY \"yes\"\nNOPE\nSAY \"no\"\nUGHA");
        assert!(matches!(&prog.statements[0], Statement::If { .. }));
    }

    #[test]
    fn test_while_loop() {
        let prog = parse("UGGA WHILE x SMALLR 10\nx GETS x PLUS 1\nUGHA");
        assert!(matches!(&prog.statements[0], Statement::While { .. }));
    }

    #[test]
    fn test_infinite_loop() {
        let prog = parse("UGGA DO\nSTOP\nUGHA");
        assert!(matches!(&prog.statements[0], Statement::Loop { .. }));
    }

    #[test]
    fn test_func_def() {
        let prog = parse("MAGIC add(a, b)\nGIVEBACK a PLUS b\nUGHA");
        assert!(matches!(&prog.statements[0], Statement::FuncDef { .. }));
    }

    #[test]
    fn test_func_call_stmt() {
        let prog = parse("foo(1, 2)");
        assert!(matches!(&prog.statements[0], Statement::ExprStmt { .. }));
    }

    #[test]
    fn test_expr_precedence() {
        // 2 PLUS 3 TIMES 4  should parse as  2 + (3 * 4)
        let prog = parse("SAY 2 PLUS 3 TIMES 4");
        match &prog.statements[0] {
            Statement::Say { value, .. } => match value {
                Expr::BinOp { op: BinOp::Plus, right, .. } => {
                    assert!(matches!(**right, Expr::BinOp { op: BinOp::Times, .. }));
                }
                _ => panic!("expected Plus at top level"),
            },
            _ => panic!("expected Say"),
        }
    }

    #[test]
    fn test_biggr_is_compound() {
        let prog = parse("SAY x BIGGR IS 5");
        match &prog.statements[0] {
            Statement::Say { value, .. } => {
                assert!(matches!(value, Expr::BinOp { op: BinOp::BiggrIs, .. }));
            }
            _ => panic!("expected Say"),
        }
    }
}
