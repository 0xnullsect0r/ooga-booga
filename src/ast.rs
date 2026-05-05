use crate::error::Span;

/// The root of the AST: an ordered list of top-level statements.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Every kind of statement the language supports.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Statement {
    /// OOGA name  /  OOGA name BE expr
    VarDecl {
        name: String,
        initializer: Option<Expr>,
        span: Span,
    },
    /// name GETS expr
    Assign {
        name: String,
        value: Expr,
        span: Span,
    },
    /// SAY expr
    Say {
        value: Expr,
        span: Span,
    },
    /// HEAR name
    Hear {
        name: String,
        span: Span,
    },
    /// IFF expr ... [NOPE IFF expr ...] [NOPE ...] UGHA
    If {
        condition: Expr,
        then_block: Vec<Statement>,
        else_ifs: Vec<(Expr, Vec<Statement>)>,
        else_block: Option<Vec<Statement>>,
        span: Span,
    },
    /// UGGA WHILE expr ... UGHA
    While {
        condition: Expr,
        body: Vec<Statement>,
        span: Span,
    },
    /// UGGA DO ... UGHA
    Loop {
        body: Vec<Statement>,
        span: Span,
    },
    /// STOP
    Break {
        span: Span,
    },
    /// SKIP
    Continue {
        span: Span,
    },
    /// MAGIC name(params) ... UGHA
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
        span: Span,
    },
    /// GIVEBACK [expr]
    Return {
        value: Option<Expr>,
        span: Span,
    },
    /// A bare expression used as a statement (function calls).
    ExprStmt {
        expr: Expr,
        span: Span,
    },
}

#[allow(dead_code)]
impl Statement {
    pub fn span(&self) -> &Span {
        match self {
            Statement::VarDecl { span, .. } => span,
            Statement::Assign { span, .. } => span,
            Statement::Say { span, .. } => span,
            Statement::Hear { span, .. } => span,
            Statement::If { span, .. } => span,
            Statement::While { span, .. } => span,
            Statement::Loop { span, .. } => span,
            Statement::Break { span } => span,
            Statement::Continue { span } => span,
            Statement::FuncDef { span, .. } => span,
            Statement::Return { span, .. } => span,
            Statement::ExprStmt { span, .. } => span,
        }
    }
}

/// Every kind of expression the language supports.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Literal {
        value: Literal,
        span: Span,
    },
    Ident {
        name: String,
        span: Span,
    },
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expr>,
        span: Span,
    },
    FuncCall {
        name: String,
        args: Vec<Expr>,
        span: Span,
    },
}

#[allow(dead_code)]
impl Expr {
    pub fn span(&self) -> &Span {
        match self {
            Expr::Literal { span, .. } => span,
            Expr::Ident { span, .. } => span,
            Expr::BinOp { span, .. } => span,
            Expr::UnaryOp { span, .. } => span,
            Expr::FuncCall { span, .. } => span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    // Arithmetic
    Plus,
    Minus,
    Times,
    Divvy,
    Mod,
    // Comparison
    Is,
    Isnt,
    Biggr,
    Smallr,
    BiggrIs,
    SmallrIs,
    // Logical
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}
