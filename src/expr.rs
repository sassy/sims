#[derive(Debug,Clone, PartialEq)]
pub enum Expr {
    Int(i32),
    Symbol(String),
    Boolean(bool),
    List(Vec<Expr>),
    Cons(Box<(Expr, Expr)>),
}

impl Expr {
    pub fn expr_str(self) -> String {
        match self {
            Expr::Int(n) => n.to_string(),
            Expr::Symbol(s) => s,
            Expr::Boolean(b) => if b { "#t".to_owned() } else {"#f".to_owned()},
            Expr::List(_) => "not implemented".to_string(),
            Expr::Cons(_) => "not implemented".to_string(),
        }
    }
}