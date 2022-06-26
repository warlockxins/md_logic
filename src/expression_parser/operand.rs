use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
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
            (Operand::String(s1), Operand::String(s2)) => Operand::String(format!("{}{}", s1, s2)),
            (_, _1) => Operand::None,
        }
    }
}

impl Sub for Operand {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Operand::Number(n1), Operand::Number(n2)) => Operand::Number(n1 - n2),
            (_, _1) => Operand::None,
        }
    }
}

impl Mul for Operand {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Operand::Number(n1), Operand::Number(n2)) => Operand::Number(n1 * n2),
            (_, _1) => Operand::None,
        }
    }
}

impl Div for Operand {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Operand::Number(n1), Operand::Number(n2)) => {
                if n2 == 0.0 {
                    return Operand::None;
                }
                return Operand::Number(n1 / n2);
            }
            (_, _1) => Operand::None,
        }
    }
}

impl std::cmp::Ord for Operand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Operand::Number(n1), Operand::Number(n2)) => {
                if n1 > n2 {
                    return std::cmp::Ordering::Greater;
                }
                if n1 < n2 {
                    return std::cmp::Ordering::Less;
                }

                return std::cmp::Ordering::Equal;
            }
            (_, _1) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Operand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Operand {}
