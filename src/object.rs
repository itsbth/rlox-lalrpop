// (#scope (0 is current, 1 is parent, etc), #idx)
struct Variable(u8, u8);
struct Function {
    arity: i8,
    body: Vec<Statement>,
}

pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    NativeFunction,
    LoxFunction(&Function),
}
