use std::collections::HashMap;
use ast;
use ast::Variable;

#[derive(Debug)]
enum LValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    LoxFunction,
    NativeFunction,
}

#[derive(Debug)]
pub enum RuntimeError {
    NotYetImplemented,
    NoSuchVariable(String),
}

pub struct Interpreter {
    globals: HashMap<String, f64>,
    scope: Vec<[f64; 16]>,
}

type StatementResult = Result<Option<f64>, RuntimeError>;
type ExpressionResult = Result<f64, RuntimeError>;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            globals: HashMap::new(),
            scope: Vec::new(),
        }
    }

    pub fn run(&mut self, program: ast::Program) -> StatementResult {
        let mut last = None;
        for stmt in program.statements() {
            last = self.execute(stmt)?;
        }
        Ok(last)
    }
    pub fn execute(&mut self, stmt: &ast::Statement) -> StatementResult {
        match *stmt {
            ast::Statement::Print(ref expr) => {
                println!("{:?}", self.evaluate(expr)?);
                Ok(None)
            }
            ast::Statement::Expression(ref expr) => self.evaluate(expr).map(Some),
            ast::Statement::Block(ref body) => {
                self.scope.push([0.0; 16]);
                for stmt in body {
                    self.execute(stmt)?;
                }
                self.scope.pop();
                Ok(None)
            }
            ast::Statement::If(ref cond, ref if_true, ref if_false) => {
                // comparing floats? genius
                if self.evaluate(cond)? != 0.0 {
                    self.execute(if_true)?;
                } else if let Some(ref if_false) = **if_false {
                    self.execute(&if_false)?;
                }
                Ok(None)
            }
            _ => Err(RuntimeError::NotYetImplemented),
        }
    }
    pub fn evaluate(&mut self, expr: &ast::Expression) -> ExpressionResult {
        match *expr {
            ast::Expression::Binop(ast::Binop::Assignment, ref lhs, ref rhs) => {
                if let ast::Expression::Variable(ref name) = **lhs {
                    let rhs = self.evaluate(rhs)?;
                    match *name {
                        Variable::Global(ref name) => {
                            self.globals.insert(name.clone(), rhs);
                            return Ok(rhs);
                        }
                        Variable::Scoped(pos, idx) => {
                            let pos = self.scope.len() - pos as usize;
                            self.scope[pos - 1][idx as usize] = rhs;
                            return Ok(rhs);
                        }
                        _ => println!("err: {:?}", name),
                    }
                }
                return Err(RuntimeError::NotYetImplemented);
            }
            ast::Expression::Binop(ref op, ref lhs, ref rhs) => {
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;
                match *op {
                    ast::Binop::Add => Ok(lhs + rhs),
                    ast::Binop::Sub => Ok(lhs - rhs),
                    ast::Binop::Mul => Ok(lhs * rhs),
                    ast::Binop::Div => Ok(lhs / rhs),
                    _ => Err(RuntimeError::NotYetImplemented),
                }
            }
            ast::Expression::Literal(n) => Ok(n),
            ast::Expression::Variable(ref name) => match *name {
                Variable::Global(ref name) => self.globals
                    .get(name)
                    .map(|n| *n)
                    .ok_or(RuntimeError::NoSuchVariable(name.clone())),
                Variable::Scoped(pos, idx) => {
                    let pos = self.scope.len() - pos as usize;
                    Ok(self.scope[pos - 1][idx as usize])
                }
                _ => Err(RuntimeError::NotYetImplemented),
            },
            _ => Err(RuntimeError::NotYetImplemented),
        }
    }
}
