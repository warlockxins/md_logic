#[derive(Debug, PartialEq)]
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

#[derive(PartialEq, Debug)]
pub enum Operand {
    //Operator(Operator, Box<Operand>, Box<Operand>),
    Number(f32),
    String(String),
    Variable(String),
    OperatorToken(Operator),
    OpenParen,
    CloseParen,
}

pub type BinaryOperation = (Operator, Operand, Operand);
