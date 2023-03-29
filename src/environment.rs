use crate::expr::Expr;
pub struct Env {
    bindings: Vec<(String, Expr)>
}

impl Env {
    pub fn new() -> Env{
        Env {bindings: vec![]}
    }

    pub fn set(&mut self, name: String, value: Expr) {
        let pos  = self.bindings.iter().position(|(n, _)| n == &name);
        match pos {
            Some(index) => self.bindings[index] = (name, value),
            None => self.bindings.push((name, value)),
        }
    }

   pub fn get(&self, name: &str) -> Expr {
        let pos = self.bindings.iter().position(|(n, _)| n == &name);
        match pos {
            Some(index) => self.bindings[index].1.clone(),
            None => panic!("undefined variable: {}", name),
        }
    }
}