use std::collections::HashMap;

use crate::parse::Binary;
use crate::parse::Expr;
use crate::parse::Lit;
use crate::parse::Stmt;
use crate::parse::Unary;

pub struct Interpreter {
    envs: Vec<HashMap<String, Value>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { envs: Vec::new() }
    }
}

impl Interpreter {
    pub fn eval_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.eval_expr(expr);
                match value {
                    Value::String(string) => println!("{}", string),
                    Value::Bool(bool) => println!("{}", bool),
                    Value::Nil => println!("nil"),
                    Value::Number(num) => println!("{}", num),
                }
            }
            Stmt::Seq(stmts) => {
                self.envs.push(HashMap::new());
                for stmt in stmts {
                    self.eval_stmt(stmt);
                }
                self.envs.pop();
            }
            Stmt::Assign { lhs, rhs } => {
                let value = self.eval_expr(rhs);
                self.envs
                    .last_mut()
                    .expect("internal compiler error: missing environment")
                    .insert(lhs.clone(), value);
            }
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Lit(lit) => match lit {
                Lit::Var(var) => self
                    .envs
                    .iter()
                    .rev()
                    .find_map(|env| env.get(var))
                    .unwrap_or_else(|| panic!("variable not defined: {}", var))
                    .clone(),
                Lit::Number(number) => Value::Number(*number),
                Lit::String(string) => Value::String(string.clone()),
                Lit::Bool(bool) => Value::Bool(*bool),
                Lit::Nil => Value::Nil,
            },
            Expr::Unary(op, e) => {
                let v = self.eval_expr(e);
                match op {
                    Unary::Negate => match v {
                        Value::Number(n) => Value::Number(-n),
                        _ => panic!("Tried to negate non-number"),
                    },
                    Unary::Not => match v {
                        Value::Bool(b) => Value::Bool(!b),
                        _ => panic!("Tried to not non-bool"),
                    },
                }
            }
            Expr::Binary(op, lhs, rhs) => {
                let v_left = self.eval_expr(lhs);
                let n_left = match v_left {
                    Value::Number(n) => n,
                    _ => panic!("Tried to negate non-number"),
                };
                let v_right = self.eval_expr(rhs);
                let n_right = match v_right {
                    Value::Number(n) => n,
                    _ => panic!("Tried to negate non-number"),
                };
                match op {
                    Binary::Add => Value::Number(n_left + n_right),
                    Binary::Sub => Value::Number(n_left - n_right),
                    Binary::Mul => Value::Number(n_left * n_right),
                    Binary::Div => Value::Number(n_left / n_right),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}
