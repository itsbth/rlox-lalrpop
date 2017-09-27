#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Assignment,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Variable {
    Unresolved(String),
    Scoped(u8, u8),
    Closure(u8),
    Global(String),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(f64),
    Binop(Binop, Box<Expression>, Box<Expression>),
    Variable(Variable),
    Call(Box<Expression>, Vec<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Print(Expression),
    Expression(Expression),
    Var(Variable, Option<Expression>),
    Fun(String, Vec<String>, Vec<Statement>),
    Return(Option<Expression>),
    Block(Vec<Statement>),
    While(Expression, Box<Statement>),
    If(Expression, Box<Statement>, Box<Option<Statement>>),
}

#[derive(Debug, Clone)]
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Program {
        Program { statements }
    }
    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}
