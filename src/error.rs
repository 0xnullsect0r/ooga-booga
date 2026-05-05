use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

impl Span {
    pub fn new(line: usize, col: usize) -> Self {
        Span { line, col }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, col {}", self.line, self.col)
    }
}

#[derive(Error, Debug, Clone)]
pub enum OogaError {
    #[error("UGH! CAVE BRAIN NO READ FILE: {0}")]
    #[allow(dead_code)]
    IoError(String),

    #[error("UGH! ME NO UNDERSTAND '{token}' at {span}. {hint}")]
    LexError {
        token: String,
        span: Span,
        hint: String,
    },

    #[error("BONK! BAD GRAMMAR at {span}: {message}")]
    ParseError { span: Span, message: String },

    #[error("OW! CAVE THINKER CONFUSED at {span}: {message}")]
    SemanticError { span: Span, message: String },
}

impl OogaError {
    pub fn lex(token: impl Into<String>, span: Span, hint: impl Into<String>) -> Self {
        OogaError::LexError {
            token: token.into(),
            span,
            hint: hint.into(),
        }
    }

    pub fn parse(span: Span, message: impl Into<String>) -> Self {
        OogaError::ParseError {
            span,
            message: message.into(),
        }
    }

    pub fn semantic(span: Span, message: impl Into<String>) -> Self {
        OogaError::SemanticError {
            span,
            message: message.into(),
        }
    }
}

pub type OogaResult<T> = Result<T, OogaError>;
