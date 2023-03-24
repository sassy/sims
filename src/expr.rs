#[derive(Debug,Clone, PartialEq)]
pub enum Expr {
    Int(i32),
    Symbol(String),
    List(Vec<Expr>),
    Cons(Box<(Expr, Expr)>),
}

impl Expr {
    pub fn expr_str(self) -> String {
        match self {
            Expr::Int(n) => n.to_string(),
            Expr::Symbol(s) => s,
            Expr::List(_) => "not implemented".to_string(),
            Expr::Cons(_) => "not implemented".to_string(),
        }
    }
}