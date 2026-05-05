use std::collections::HashSet;

use crate::ast::*;
use crate::error::OogaError;

/// Context passed down during semantic analysis.
struct Context {
    /// Variables visible in the current scope.
    declared: HashSet<String>,
    /// Whether we are currently inside a function body.
    in_function: bool,
    /// Whether we are currently inside a loop body.
    in_loop: bool,
    /// Names of all top-level functions (populated in first pass).
    functions: HashSet<String>,
}

impl Context {
    fn new() -> Self {
        Context {
            declared: HashSet::new(),
            in_function: false,
            in_loop: false,
            functions: HashSet::new(),
        }
    }

    /// Built-in function names that are always available.
    fn is_builtin(name: &str) -> bool {
        matches!(
            name,
            "NUMBR" | "WORDY" | "BIGNESS" | "FLOORY" | "ROUNDY" | "ROOTY"
        )
    }

    fn is_known_callable(&self, name: &str) -> bool {
        Self::is_builtin(name) || self.functions.contains(name)
    }

    fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name) || Self::is_builtin(name)
    }
}

/// Run semantic analysis on a program, returning a list of errors.
/// We collect all errors rather than stopping at the first.
pub fn analyse(program: &Program) -> Vec<OogaError> {
    let mut errors = Vec::new();
    let mut ctx = Context::new();

    // First pass: collect all top-level function names so mutual recursion works.
    for stmt in &program.statements {
        if let Statement::FuncDef { name, .. } = stmt {
            ctx.functions.insert(name.clone());
        }
    }

    for stmt in &program.statements {
        check_statement(stmt, &mut ctx, &mut errors);
    }

    errors
}

fn check_statement(stmt: &Statement, ctx: &mut Context, errors: &mut Vec<OogaError>) {
    match stmt {
        Statement::VarDecl { name, initializer, span } => {
            if let Some(init) = initializer {
                check_expr(init, ctx, errors);
            }
            ctx.declared.insert(name.clone());
            let _ = span;
        }

        Statement::Assign { name, value, span } => {
            if !ctx.is_declared(name) {
                errors.push(OogaError::semantic(
                    span.clone(),
                    format!(
                        "NAME \"{}\" NOT KNOWN. CAVE CREATURE NOT DECLARE IT WITH OOGA FIRST.",
                        name
                    ),
                ));
            }
            check_expr(value, ctx, errors);
        }

        Statement::Say { value, .. } => {
            check_expr(value, ctx, errors);
        }

        Statement::Hear { name, span } => {
            if !ctx.is_declared(name) {
                errors.push(OogaError::semantic(
                    span.clone(),
                    format!(
                        "HEAR NEEDS DECLARED VARIABLE BUT \"{}\" NOT KNOWN. USE OOGA FIRST.",
                        name
                    ),
                ));
            }
        }

        Statement::If {
            condition,
            then_block,
            else_ifs,
            else_block,
            ..
        } => {
            check_expr(condition, ctx, errors);
            check_block(then_block, ctx, errors);
            for (cond, block) in else_ifs {
                check_expr(cond, ctx, errors);
                check_block(block, ctx, errors);
            }
            if let Some(block) = else_block {
                check_block(block, ctx, errors);
            }
        }

        Statement::While { condition, body, .. } => {
            check_expr(condition, ctx, errors);
            let prev = ctx.in_loop;
            ctx.in_loop = true;
            check_block(body, ctx, errors);
            ctx.in_loop = prev;
        }

        Statement::Loop { body, .. } => {
            let prev = ctx.in_loop;
            ctx.in_loop = true;
            check_block(body, ctx, errors);
            ctx.in_loop = prev;
        }

        Statement::Break { span } => {
            if !ctx.in_loop {
                errors.push(OogaError::semantic(
                    span.clone(),
                    "STOP OUTSIDE LOOP. CAVE BRAIN CONFUSED. STOP ONLY WORK INSIDE UGGA BLOCK.",
                ));
            }
        }

        Statement::Continue { span } => {
            if !ctx.in_loop {
                errors.push(OogaError::semantic(
                    span.clone(),
                    "SKIP OUTSIDE LOOP. CAVE BRAIN CONFUSED. SKIP ONLY WORK INSIDE UGGA BLOCK.",
                ));
            }
        }

        Statement::FuncDef { name, params, body, span } => {
            // Register function name in outer scope
            ctx.functions.insert(name.clone());
            // Build inner context for function body
            let mut inner = Context {
                declared: params.iter().cloned().collect(),
                in_function: true,
                in_loop: false,
                functions: ctx.functions.clone(),
            };
            // Check for duplicate parameter names
            let mut seen_params = HashSet::new();
            for p in params {
                if !seen_params.insert(p) {
                    errors.push(OogaError::semantic(
                        span.clone(),
                        format!(
                            "FUNCTION \"{}\" HAS DUPLICATE PARAMETER \"{}\". CAVE NO LIKE TWINS.",
                            name, p
                        ),
                    ));
                }
            }
            check_block(body, &mut inner, errors);
        }

        Statement::Return { value, span } => {
            if !ctx.in_function {
                errors.push(OogaError::semantic(
                    span.clone(),
                    "GIVEBACK OUTSIDE FUNCTION. ONLY USE GIVEBACK INSIDE MAGIC BLOCK.",
                ));
            }
            if let Some(val) = value {
                check_expr(val, ctx, errors);
            }
        }

        Statement::ExprStmt { expr, .. } => {
            check_expr(expr, ctx, errors);
        }
    }
}

fn check_block(stmts: &[Statement], ctx: &mut Context, errors: &mut Vec<OogaError>) {
    // Snapshot declared set; variables declared inside the block are visible
    // for the rest of the block (simple forward-declaration semantics).
    for stmt in stmts {
        check_statement(stmt, ctx, errors);
    }
}

fn check_expr(expr: &Expr, ctx: &mut Context, errors: &mut Vec<OogaError>) {
    match expr {
        Expr::Literal { .. } => {}

        Expr::Ident { name, span } => {
            if !ctx.is_declared(name) {
                errors.push(OogaError::semantic(
                    span.clone(),
                    format!(
                        "NAME \"{}\" NOT KNOWN. CAVE CREATURE NOT DECLARE IT WITH OOGA FIRST.",
                        name
                    ),
                ));
            }
        }

        Expr::BinOp { left, right, .. } => {
            check_expr(left, ctx, errors);
            check_expr(right, ctx, errors);
        }

        Expr::UnaryOp { operand, .. } => {
            check_expr(operand, ctx, errors);
        }

        Expr::FuncCall { name, args, span } => {
            if !ctx.is_known_callable(name) {
                errors.push(OogaError::semantic(
                    span.clone(),
                    format!(
                        "FUNCTION \"{}\" NOT KNOWN. DEFINE WITH MAGIC FIRST.",
                        name
                    ),
                ));
            }
            for arg in args {
                check_expr(arg, ctx, errors);
            }
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn analyse_src(src: &str) -> Vec<OogaError> {
        let tokens = Lexer::new(src).tokenise().expect("lex");
        let program = Parser::new(tokens).parse_program().expect("parse");
        analyse(&program)
    }

    #[test]
    fn test_no_errors_on_valid_program() {
        let errors = analyse_src("OOGA x BE 1\nSAY x");
        assert!(errors.is_empty(), "expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_undefined_variable() {
        let errors = analyse_src("SAY y");
        assert!(!errors.is_empty(), "expected undefined-var error");
        assert!(format!("{}", errors[0]).contains("y"));
    }

    #[test]
    fn test_return_outside_function() {
        let errors = analyse_src("GIVEBACK 1");
        assert!(!errors.is_empty());
        assert!(format!("{}", errors[0]).contains("GIVEBACK"));
    }

    #[test]
    fn test_break_outside_loop() {
        let errors = analyse_src("STOP");
        assert!(!errors.is_empty());
        assert!(format!("{}", errors[0]).contains("STOP"));
    }

    #[test]
    fn test_func_params_in_scope() {
        let errors = analyse_src("MAGIC double(n)\nGIVEBACK n TIMES 2\nUGHA");
        assert!(errors.is_empty(), "params should be in scope: {:?}", errors);
    }

    #[test]
    fn test_unknown_function_call() {
        let errors = analyse_src("OOGA x BE ghost(1)");
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_builtin_functions_allowed() {
        let errors = analyse_src("OOGA x BE 3.7\nOOGA n BE FLOORY(x)");
        assert!(errors.is_empty(), "builtins should be allowed: {:?}", errors);
    }

    #[test]
    fn test_undeclared_assign_target() {
        let errors = analyse_src("z GETS 99");
        assert!(!errors.is_empty());
    }
}
