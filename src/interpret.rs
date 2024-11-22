use crate::parse::Binary;
use crate::parse::Expr;
use crate::parse::Lit;
use crate::parse::Stmt;
use crate::parse::Unary;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Interpreter {
    pub fn eval_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.eval_expr(expr);
                match value {
                    Lit::String(string) => println!("{}", string),
                    Lit::Bool(bool) => println!("{}", bool),
                    Lit::Nil => println!("nil"),
                    Lit::Number(num) => println!("{}", num),
                }
            }
            Stmt::Seq(stmts) => {
                for stmt in stmts {
                    self.eval_stmt(stmt);
                }
            }
        }
    }

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
                let v_left = self.eval_expr(lhs);
                let n_left = match v_left {
                    Lit::Number(n) => n,
                    _ => panic!("Tried to negate non-number"),
                };
                let v_right = self.eval_expr(rhs);
                let n_right = match v_right {
                    Lit::Number(n) => n,
                    _ => panic!("Tried to negate non-number"),
                };
                match op {
                    Binary::Add => Lit::Number(n_left + n_right),
                    Binary::Sub => Lit::Number(n_left - n_right),
                    Binary::Mul => Lit::Number(n_left * n_right),
                    Binary::Div => Lit::Number(n_left / n_right),
                }
            }
        }
    }
}
