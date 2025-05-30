#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Str(String),
    Ident(String),
    Input(Vec<Expr>), 
    Call(String, Vec<Expr>),
    Binary(Box<Expr>, String, Box<Expr>),
    Logical(String, Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Array(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Object(Vec<(String, Box<Expr>)>),
    Access(Box<Expr>, String),
    PostfixIncrement(String),
    PostfixDecrement(String),
    MethodCall {
        target: Box<Expr>,      // a 或更长链
        name: String,           // func
        args: Vec<Expr>,        // 附加实参（不含 target）
    },
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
    PropAssign(Box<Expr>, Expr),
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