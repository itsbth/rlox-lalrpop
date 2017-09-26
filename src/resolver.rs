use std::collections::{HashMap, HashSet};
use visitor::Visitor;
use ast::{Binop, Expression, Program, Statement, Variable};

#[derive(Default, Debug)]
struct Scope {
    variables: HashMap<String, u8>,
}

impl Scope {
    pub fn get(&self, name: &String) -> Option<u8> {
        self.variables.get(&name.clone()).map(|n| *n)
    }
    pub fn add(&mut self, name: &String) -> u8 {
        let idx = self.variables.len() as u8;
        self.variables.insert(name.clone(), idx);
        idx
    }
}

pub struct Resolver {
    globals: HashSet<String>,
    stack: Vec<Scope>,
    closure: Vec<Scope>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            globals: HashSet::new(),
            stack: Vec::new(),
            closure: Vec::new(),
        }
    }
    pub fn run(&mut self, prog: Program) -> Program {
        self.visit_program(prog)
    }
    fn enter_scope(&mut self) {
        self.stack.push(Default::default())
    }
    fn leave_scope(&mut self) {
        self.stack.pop();
    }
    fn current_scope(&mut self) -> &mut Scope {
        let top = { self.stack.len() - 1 };
        &mut self.stack[top]
    }
    fn is_root(&self) -> bool {
        self.stack.len() == 0
    }
    fn define_var(&mut self, name: &String) -> Variable {
        if self.is_root() {
            self.globals.insert(name.clone());
            Variable::Global(name.clone())
        } else {
            let idx = self.current_scope().add(name);
            Variable::Scoped(0, idx)
        }
    }
    fn resolve_var(&self, name: &String) -> Variable {
        if let Some((pos, idx)) = self.lookup_scope(name) {
            Variable::Scoped(pos, idx)
        } else {
            Variable::Global(name.clone())
        }
    }
    fn lookup_scope(&self, name: &String) -> Option<(u8, u8)> {
        for (pos, scope) in self.stack.iter().rev().enumerate() {
            if let Some(idx) = scope.get(name) {
                return Some((pos as u8, idx));
            }
        }
        return None;
    }
}

impl Visitor for Resolver {
    fn visit_var(&mut self, name: &Variable, val: &Option<Expression>) -> Statement {
        let val = match *val {
            Some(ref expr) => Some(self.visit_expr(expr)),
            None => None,
        };
        let name = match *name {
            Variable::Unresolved(ref name) => self.define_var(name),
            ref other => other.clone(),
        };
        // Statement::Var(name.clone(), val)
        Statement::Expression(Expression::Binop(
            Binop::Assignment,
            Box::new(Expression::Variable(name)),
            Box::new(val.unwrap_or(Expression::Literal(0.0))),
        ))
    }
    fn visit_variable(&mut self, name: &Variable) -> Expression {
        let name = match *name {
            Variable::Unresolved(ref name) => self.resolve_var(name),
            ref other => other.clone(),
        };
        Expression::Variable(name)
    }

    fn visit_block(&mut self, body: &Vec<Statement>) -> Statement {
        self.enter_scope();
        let body = body.iter().map(|st| self.visit_statement(st)).collect();
        self.leave_scope();
        Statement::Block(body)
    }
}
