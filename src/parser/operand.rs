#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Substract,
    Multiply,
    Division,
    L,
    G,
    LE,
    GE,
    E,
    NE,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operand {
    //Operator(Operator, Box<Operand>, Box<Operand>),
    Number(f32),
    String(String),
    Variable(String),
    OperatorToken(Operator),
    OpenParen,
    CloseParen,
}
