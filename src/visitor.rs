use ast::{Binop, Expression, Program, Statement, Variable};

pub trait Visitor {
    fn visit_program(&mut self, prog: Program) -> Program {
        Program::new(
            prog.statements()
                .into_iter()
                .map(|st| self.visit_statement(st))
                .collect(),
        )
    }
    fn visit_statement(&mut self, st: &Statement) -> Statement {
        match *st {
            Statement::Block(ref body) => self.visit_block(body),
            Statement::Expression(ref expr) => self.visit_expression(expr),
            Statement::Fun(ref name, ref bindings, ref body) => {
                self.visit_fun(name, bindings, body)
            }
            Statement::If(ref expr, ref body) => self.visit_if(expr, body),
            Statement::Print(ref expr) => self.visit_print(expr),
            Statement::Return(ref expr) => self.visit_return(expr),
            Statement::Var(ref name, ref val) => self.visit_var(name, val),
            Statement::While(ref expr, ref body) => self.visit_while(expr, body),
        }
    }
    fn visit_block(&mut self, body: &Vec<Statement>) -> Statement {
        let body = body.iter().map(|st| self.visit_statement(st)).collect();
        Statement::Block(body)
    }
    fn visit_expression(&mut self, expr: &Expression) -> Statement {
        let expr = self.visit_expr(expr);
        Statement::Expression(expr)
    }
    fn visit_fun(
        &mut self,
        name: &String,
        bindings: &Vec<String>,
        body: &Vec<Statement>,
    ) -> Statement {
        let body = body.iter().map(|st| self.visit_statement(st)).collect();
        Statement::Fun(name.clone(), bindings.clone(), body)
    }
    fn visit_if(&mut self, expr: &Expression, body: &Statement) -> Statement {
        let expr = self.visit_expr(expr);
        let body = self.visit_statement(body);
        Statement::If(expr, Box::new(body))
    }
    fn visit_print(&mut self, expr: &Expression) -> Statement {
        let expr = self.visit_expr(expr);
        Statement::Print(expr)
    }
    fn visit_return(&mut self, expr: &Option<Expression>) -> Statement {
        let expr = match *expr {
            Some(ref expr) => Some(self.visit_expr(expr)),
            None => None,
        };
        Statement::Return(expr)
    }
    fn visit_var(&mut self, name: &Variable, val: &Option<Expression>) -> Statement {
        let val = match *val {
            Some(ref val) => Some(self.visit_expr(val)),
            None => None,
        };
        Statement::Var(name.clone(), val)
    }
    fn visit_while(&mut self, expr: &Expression, body: &Statement) -> Statement {
        let expr = self.visit_expr(expr);
        let body = self.visit_statement(body);
        Statement::While(expr, Box::new(body))
    }
    fn visit_expr(&mut self, expr: &Expression) -> Expression {
        match *expr {
            Expression::Binop(ref op, ref lhs, ref rhs) => self.visit_binop(op, lhs, rhs),
            Expression::Literal(n) => self.visit_literal(n),
            Expression::Variable(ref name) => self.visit_variable(name),
            Expression::Call(ref lhs, ref args) => self.visit_call(lhs, args),
        }
    }
    fn visit_binop(&mut self, op: &Binop, lhs: &Expression, rhs: &Expression) -> Expression {
        let lhs = self.visit_expr(lhs);
        let rhs = self.visit_expr(rhs);
        Expression::Binop(*op, Box::new(lhs), Box::new(rhs))
    }
    fn visit_literal(&mut self, val: f64) -> Expression {
        Expression::Literal(val)
    }
    fn visit_variable(&mut self, name: &Variable) -> Expression {
        Expression::Variable(name.clone())
    }
    fn visit_call(&mut self, lhs: &Expression, args: &Vec<Expression>) -> Expression {
        let lhs = self.visit_expr(lhs);
        let args = args.iter().map(|a| self.visit_expr(a)).collect();
        Expression::Call(Box::new(lhs), args)
    }
}
