use std::ops::Add;
use std::ops::Sub;

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
    None,
    Boolean(bool),
    Number(f32),
    String(String),
    Variable(String),
    OperatorToken(Operator),
    OpenParen,
    CloseParen,
}

impl Add for Operand {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Operand::Number(n1), Operand::Number(n2)) => Operand::Number(n1 + n2),
            (_, _1) => Operand::Number(0.0),
        }
    }
}

impl Sub for Operand {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Operand::Number(n1), Operand::Number(n2)) => Operand::Number(n1 - n2),
            (_, _1) => Operand::Number(0.0),
        }
    }
}
