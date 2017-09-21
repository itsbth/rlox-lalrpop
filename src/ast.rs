#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(f64),
    Binop(Binop, Box<Expression>, Box<Expression>),
    Variable(String),
    Call(Box<Expression>, Vec<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Print(Expression),
    Expression(Expression),
    Var(String, Option<Expression>),
    Fun(String, Vec<String>, Vec<Statement>),
    Return(Option<Expression>),
    Block(Vec<Statement>),
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Program {
        Program{ statements  }
    }
}
