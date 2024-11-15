use crate::parse::Binary;
use crate::parse::Expr;
use crate::parse::Lit;
use crate::parse::Unary;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Interpreter {
    pub fn eval_expr(&mut self, expr: &Expr) -> Lit {
        match expr {
            Expr::Lit(lit) => lit.clone(),
            Expr::Unary(op, e) => {
                let v = self.eval_expr(e);
                match op {
                    Unary::Negate => match v {
                        Lit::Number(n) => Lit::Number(-n),
                        _ => panic!("Tried to negate non-number"),
                    },
                    Unary::Not => match v {
                        Lit::Bool(b) => Lit::Bool(!b),
                        _ => panic!("Tried to not non-bool"),
                    },
                }
            }
            Expr::Binary(op, lhs, rhs) => {
                let vLeft = self.eval_expr(lhs);
                let nLeft = match vLeft {
                    Lit::Number(n) => n,
                    _ => panic!("Tried to negate non-number"),
                };
                let vRight = self.eval_expr(rhs);
                let nRight = match vRight {
                    Lit::Number(n) => n,
                    _ => panic!("Tried to negate non-number"),
                };
                match op {
                    Binary::Add => Lit::Number(nLeft + nRight),
                    Binary::Sub => Lit::Number(nLeft - nRight),
                    Binary::Mul => Lit::Number(nLeft * nRight),
                    Binary::Div => Lit::Number(nLeft / nRight),
                }
            }
        }
    }
}
