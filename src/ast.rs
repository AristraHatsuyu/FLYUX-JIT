#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Str(String),
    Ident(String),
    Call(String, Vec<Expr>),
    Binary(Box<Expr>, String, Box<Expr>),
    Logical(String, Box<Expr>, Box<Expr>),
    Array(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Object(Vec<(String, Box<Expr>)>),
    Access(Box<Expr>, String),
    PostfixIncrement(String),
    PostfixDecrement(String),
}

#[derive(Debug, Clone)]
pub enum LoopKind {
    Times(Expr),                         // L>[10]
    ForEach(String, Expr),               // L>data:item
    While(Expr),                         // L>(condition)
    For(Box<Stmt>, Expr, Box<Stmt>),     // L>(init; cond; step)
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ConstDecl(String, Option<String>, Expr),
    VarDecl(String, Option<String>, Expr),
    Loop(LoopKind, Vec<Stmt>),
    MultiIf(Vec<(Option<Expr>, Vec<Stmt>)>),
    Assign(String, Expr),
    Increment(String),
    Decrement(String),
    #[allow(dead_code)]
    Expr(Expr),
    Return(Expr),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Option<String>)>,
    pub body: Vec<Stmt>,
}