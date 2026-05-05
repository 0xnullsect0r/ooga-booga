use crate::error::{OogaError, OogaResult, Span};

/// Every syntactic token the Ooga Booga lexer can produce.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // ── Literals ──────────────────────────────────────────────────────────
    Int(i64),
    Float(f64),
    Str(String),
    Ident(String),

    // ── Keywords ──────────────────────────────────────────────────────────
    Ooga,     // variable declaration
    Be,       // initialiser in OOGA x BE expr
    Gets,     // assignment
    Say,      // print
    Hear,     // input
    Iff,      // if
    Nope,     // else / else-if
    Ugha,     // end block
    Ugga,     // loop prefix
    While,    // UGGA WHILE
    Do,       // UGGA DO (infinite loop)
    Stop,     // break
    Skip,     // continue
    Magic,    // function definition
    Giveback, // return

    // ── Operators (word form) ──────────────────────────────────────────────
    Plus,
    Minus,
    Times,
    Divvy,
    Mod,
    Is,
    Isnt,
    Biggr,
    Smallr,
    And,
    Or,
    Not,

    // ── Boolean / null literals ───────────────────────────────────────────
    Yeah, // true
    Nah,  // false
    Void, // null / nothing

    // ── Punctuation ───────────────────────────────────────────────────────
    LParen,
    RParen,
    Comma,
    Newline,

    // ── Sentinel ──────────────────────────────────────────────────────────
    Eof,
}

/// A token together with the source position where it starts.
#[derive(Debug, Clone)]
pub struct Spanned {
    pub token: Token,
    pub span: Span,
}

impl Spanned {
    fn new(token: Token, line: usize, col: usize) -> Self {
        Spanned {
            token,
            span: Span::new(line, col),
        }
    }
}

// ── Lexer ─────────────────────────────────────────────────────────────────────

pub struct Lexer<'src> {
    src: &'src [u8],
    pos: usize,
    line: usize,
    col: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Lexer {
            src: src.as_bytes(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn tokenise(mut self) -> OogaResult<Vec<Spanned>> {
        let mut tokens: Vec<Spanned> = Vec::new();
        let mut last_was_newline = true; // suppress leading blank lines

        loop {
            self.skip_spaces();

            if self.pos >= self.src.len() {
                // Ensure programs always end with a newline token so the
                // parser can use it as a statement terminator.
                if !last_was_newline {
                    tokens.push(Spanned::new(Token::Newline, self.line, self.col));
                }
                tokens.push(Spanned::new(Token::Eof, self.line, self.col));
                break;
            }

            let ch = self.src[self.pos];

            // ── Newline ────────────────────────────────────────────────────
            if ch == b'\n' || ch == b'\r' {
                let (l, c) = (self.line, self.col);
                self.advance();
                if ch == b'\r' && self.pos < self.src.len() && self.src[self.pos] == b'\n' {
                    self.advance(); // CRLF
                }
                if !last_was_newline {
                    tokens.push(Spanned::new(Token::Newline, l, c));
                    last_was_newline = true;
                }
                continue;
            }

            // ── Comment: OOF to end-of-line ────────────────────────────────
            if self.src[self.pos..].starts_with(b"OOF") {
                // Check it's not part of a longer identifier
                let after = self.pos + 3;
                if after >= self.src.len() || !is_ident_char(self.src[after]) {
                    while self.pos < self.src.len() && self.src[self.pos] != b'\n' {
                        self.advance();
                    }
                    continue;
                }
            }

            // ── String literal ────────────────────────────────────────────
            if ch == b'"' {
                let tok = self.lex_string()?;
                last_was_newline = false;
                tokens.push(tok);
                continue;
            }

            // ── Number ────────────────────────────────────────────────────
            if ch.is_ascii_digit() {
                let tok = self.lex_number()?;
                last_was_newline = false;
                tokens.push(tok);
                continue;
            }

            // ── Punctuation ───────────────────────────────────────────────
            if ch == b'(' {
                tokens.push(Spanned::new(Token::LParen, self.line, self.col));
                self.advance();
                last_was_newline = false;
                continue;
            }
            if ch == b')' {
                tokens.push(Spanned::new(Token::RParen, self.line, self.col));
                self.advance();
                last_was_newline = false;
                continue;
            }
            if ch == b',' {
                tokens.push(Spanned::new(Token::Comma, self.line, self.col));
                self.advance();
                last_was_newline = false;
                continue;
            }

            // ── Identifiers / keywords ────────────────────────────────────
            if ch.is_ascii_alphabetic() || ch == b'_' {
                let tok = self.lex_word();
                last_was_newline = false;
                tokens.push(tok);
                continue;
            }

            // ── Unknown character ─────────────────────────────────────────
            let bad = ch as char;
            return Err(OogaError::lex(
                bad.to_string(),
                Span::new(self.line, self.col),
                "CAVE DRAWING NOT RECOGNISED. MAYBE YOU DROP CHISEL?",
            ));
        }

        Ok(tokens)
    }

    // ── Helpers ───────────────────────────────────────────────────────────

    fn advance(&mut self) {
        if self.pos < self.src.len() {
            if self.src[self.pos] == b'\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            self.pos += 1;
        }
    }

    fn skip_spaces(&mut self) {
        while self.pos < self.src.len()
            && (self.src[self.pos] == b' ' || self.src[self.pos] == b'\t')
        {
            self.advance();
        }
    }

    fn lex_string(&mut self) -> OogaResult<Spanned> {
        let (start_line, start_col) = (self.line, self.col);
        self.advance(); // consume opening "
        let mut value = String::new();
        loop {
            if self.pos >= self.src.len() {
                return Err(OogaError::lex(
                    "unterminated string",
                    Span::new(start_line, start_col),
                    "CAVE SPEAK NEVER END. PUT CLOSING \" ROCK.",
                ));
            }
            let ch = self.src[self.pos];
            if ch == b'"' {
                self.advance(); // consume closing "
                break;
            }
            if ch == b'\\' {
                self.advance();
                if self.pos >= self.src.len() {
                    return Err(OogaError::lex(
                        "\\",
                        Span::new(self.line, self.col),
                        "ESCAPE AT END OF CAVE WALL. ADD MORE CHISELING.",
                    ));
                }
                match self.src[self.pos] {
                    b'n' => value.push('\n'),
                    b't' => value.push('\t'),
                    b'"' => value.push('"'),
                    b'\\' => value.push('\\'),
                    other => {
                        value.push('\\');
                        value.push(other as char);
                    }
                }
                self.advance();
            } else {
                value.push(ch as char);
                self.advance();
            }
        }
        Ok(Spanned::new(Token::Str(value), start_line, start_col))
    }

    fn lex_number(&mut self) -> OogaResult<Spanned> {
        let (start_line, start_col) = (self.line, self.col);
        let start = self.pos;
        while self.pos < self.src.len() && self.src[self.pos].is_ascii_digit() {
            self.advance();
        }
        let is_float = self.pos < self.src.len()
            && self.src[self.pos] == b'.'
            && self.pos + 1 < self.src.len()
            && self.src[self.pos + 1].is_ascii_digit();

        if is_float {
            self.advance(); // consume '.'
            while self.pos < self.src.len() && self.src[self.pos].is_ascii_digit() {
                self.advance();
            }
            let text = std::str::from_utf8(&self.src[start..self.pos]).unwrap();
            let v: f64 = text.parse().map_err(|_| {
                OogaError::lex(
                    text,
                    Span::new(start_line, start_col),
                    "BIG NUMBER HURT CAVE BRAIN.",
                )
            })?;
            Ok(Spanned::new(Token::Float(v), start_line, start_col))
        } else {
            let text = std::str::from_utf8(&self.src[start..self.pos]).unwrap();
            let v: i64 = text.parse().map_err(|_| {
                OogaError::lex(
                    text,
                    Span::new(start_line, start_col),
                    "NUMBER TOO BIG FOR CAVE MATH.",
                )
            })?;
            Ok(Spanned::new(Token::Int(v), start_line, start_col))
        }
    }

    fn lex_word(&mut self) -> Spanned {
        let (start_line, start_col) = (self.line, self.col);
        let start = self.pos;
        while self.pos < self.src.len() && is_ident_char(self.src[self.pos]) {
            self.advance();
        }
        let word = std::str::from_utf8(&self.src[start..self.pos]).unwrap();
        let token = keyword_or_ident(word);
        Spanned::new(token, start_line, start_col)
    }
}

fn is_ident_char(ch: u8) -> bool {
    ch.is_ascii_alphanumeric() || ch == b'_'
}

fn keyword_or_ident(word: &str) -> Token {
    match word {
        "OOGA" => Token::Ooga,
        "BE" => Token::Be,
        "GETS" => Token::Gets,
        "SAY" => Token::Say,
        "HEAR" => Token::Hear,
        "IFF" => Token::Iff,
        "NOPE" => Token::Nope,
        "UGHA" => Token::Ugha,
        "UGGA" => Token::Ugga,
        "WHILE" => Token::While,
        "DO" => Token::Do,
        "STOP" => Token::Stop,
        "SKIP" => Token::Skip,
        "MAGIC" => Token::Magic,
        "GIVEBACK" => Token::Giveback,
        "PLUS" => Token::Plus,
        "MINUS" => Token::Minus,
        "TIMES" => Token::Times,
        "DIVVY" => Token::Divvy,
        "MOD" => Token::Mod,
        "IS" => Token::Is,
        "ISNT" => Token::Isnt,
        "BIGGR" => Token::Biggr,
        "SMALLR" => Token::Smallr,
        "AND" => Token::And,
        "OR" => Token::Or,
        "NOT" => Token::Not,
        "YEAH" => Token::Yeah,
        "NAH" => Token::Nah,
        "VOID" => Token::Void,
        other => Token::Ident(other.to_string()),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(src: &str) -> Vec<Token> {
        Lexer::new(src)
            .tokenise()
            .unwrap()
            .into_iter()
            .map(|s| s.token)
            .collect()
    }

    #[test]
    fn test_keywords() {
        let tokens = lex("OOGA BE GETS SAY HEAR");
        assert_eq!(
            tokens,
            vec![
                Token::Ooga,
                Token::Be,
                Token::Gets,
                Token::Say,
                Token::Hear,
                Token::Newline,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_loop_keywords() {
        let tokens = lex("UGGA WHILE UGGA DO STOP SKIP");
        assert_eq!(tokens[0], Token::Ugga);
        assert_eq!(tokens[1], Token::While);
        assert_eq!(tokens[2], Token::Ugga);
        assert_eq!(tokens[3], Token::Do);
        assert_eq!(tokens[4], Token::Stop);
        assert_eq!(tokens[5], Token::Skip);
    }

    #[test]
    fn test_integer_literal() {
        let tokens = lex("42");
        assert_eq!(tokens[0], Token::Int(42));
    }

    #[test]
    fn test_float_literal() {
        let tokens = lex("3.14");
        assert_eq!(tokens[0], Token::Float(3.14));
    }

    #[test]
    fn test_string_literal() {
        let tokens = lex(r#""hello cave""#);
        assert_eq!(tokens[0], Token::Str("hello cave".into()));
    }

    #[test]
    fn test_string_escape() {
        let tokens = lex(r#""line1\nline2""#);
        assert_eq!(tokens[0], Token::Str("line1\nline2".into()));
    }

    #[test]
    fn test_ident() {
        let tokens = lex("my_var");
        assert_eq!(tokens[0], Token::Ident("my_var".into()));
    }

    #[test]
    fn test_boolean_literals() {
        let tokens = lex("YEAH NAH VOID");
        assert_eq!(tokens[0], Token::Yeah);
        assert_eq!(tokens[1], Token::Nah);
        assert_eq!(tokens[2], Token::Void);
    }

    #[test]
    fn test_comment_ignored() {
        let tokens = lex("OOF this is ignored\nSAY");
        // The comment line produces no tokens; SAY is the first real token.
        assert_eq!(tokens[0], Token::Say);
    }

    #[test]
    fn test_operators() {
        let tokens = lex("PLUS MINUS TIMES DIVVY MOD IS ISNT BIGGR SMALLR AND OR NOT");
        assert_eq!(tokens[0], Token::Plus);
        assert_eq!(tokens[1], Token::Minus);
        assert_eq!(tokens[2], Token::Times);
        assert_eq!(tokens[3], Token::Divvy);
        assert_eq!(tokens[4], Token::Mod);
        assert_eq!(tokens[5], Token::Is);
        assert_eq!(tokens[6], Token::Isnt);
        assert_eq!(tokens[7], Token::Biggr);
        assert_eq!(tokens[8], Token::Smallr);
        assert_eq!(tokens[9], Token::And);
        assert_eq!(tokens[10], Token::Or);
        assert_eq!(tokens[11], Token::Not);
    }

    #[test]
    fn test_punctuation() {
        let tokens = lex("( ) ,");
        assert_eq!(tokens[0], Token::LParen);
        assert_eq!(tokens[1], Token::RParen);
        assert_eq!(tokens[2], Token::Comma);
    }

    #[test]
    fn test_multiline_newlines_collapsed() {
        let tokens = lex("SAY\n\n\nSAY");
        // Multiple blank lines should produce a single Newline between SAYs
        let say_count = tokens.iter().filter(|t| **t == Token::Say).count();
        assert_eq!(say_count, 2);
    }
}
