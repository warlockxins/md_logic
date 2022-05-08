use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;

mod operand;
use operand::{Operand, Operator};

pub struct Tokenizer<'a> {
    i: Peekable<Enumerate<Chars<'a>>>,
    operands: Vec<Operand>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expression: &'a str) -> Self {
        return Tokenizer {
            i: expression.chars().enumerate().peekable(),
            operands: Vec::with_capacity(expression.len()),
        };
    }

    pub fn parse(&mut self) -> Result<(), String> {
        loop {
            self.consume_spaces();

            if self.i.peek() == None {
                break;
            }
            let n = self.next_operand()?;
            self.operands.push(n);
        }

        Ok(())
    }

    fn consume_spaces(&mut self) {
        loop {
            match self.i.peek() {
                Some(&(_, '\t')) | Some(&(_, ' ')) | Some(&(_, '\n')) => {
                    self.i.next();
                }
                _ => break,
            }
        }
    }

    fn next_operand(&mut self) -> Result<Operand, String> {
        let mut root: Option<Operand> = None;

        while let Some(&(_index, c)) = self.i.peek() {
            if c == '(' {
                self.i.next();
                return Ok(Operand::OpenParen);
            } else if c == ')' {
                self.i.next();
                return Ok(Operand::CloseParen);
            } else if c == '"' {
                return self.consume_string();
            } else if check_if_operand(&c) {
                return self.consume_variable();
            } else if check_if_operator(&c) {
                return self.consume_operator();
            } else if check_if_digit(&c) {
                return self.consume_number();
            } else {
                return Err(format!("unknown symbol at index {}, {:?}", _index, c));
            }
        }

        Err("Reached end - unprocessed statements found".to_string())
    }
    pub fn to_postfix(&self) -> Result<Vec<&Operand>, String> {
        let mut stack: Vec<&Operand> = Vec::with_capacity(50);
        let mut postfix: Vec<&Operand> = Vec::with_capacity(self.operands.len());

        for o in &self.operands {
            match o {
                Operand::Number(_) | Operand::String(_) | Operand::Variable(_) => {
                    postfix.push(o);
                }
                Operand::OpenParen => {
                    stack.push(o);
                }
                Operand::CloseParen => {
                    let mut found = false;
                    while let Some(s_item) = stack.pop() {
                        match s_item {
                            Operand::OpenParen => {
                                found = true;
                                break;
                            }
                            _ => {
                                postfix.push(s_item);
                            }
                        }
                    }

                    if found == false {
                        return Err("no matching opening paren".to_string());
                    }
                }
                Operand::OperatorToken(t) => {
                    if stack.len() == 0 {
                        stack.push(o);
                    } else {
                        loop {
                            if let Some(Operand::OpenParen) = stack.last() {
                                stack.push(o);
                                break;
                            } else if let Some(Operand::OperatorToken(so)) = stack.last() {
                                if precedence(so) >= precedence(t) {
                                    if let Some(poped_stack_item) = stack.pop() {
                                        postfix.push(poped_stack_item);
                                    } else {
                                        return Err("stack underflow".to_string());
                                    }
                                } else {
                                    stack.push(o);
                                    break;
                                }
                            } else {
                                stack.push(o);
                                break;
                            }
                        }
                    }
                }
                _ => {
                    println!("-rest of prefix {:?}", o);
                }
            }
        }

        while let Some(s_item) = stack.pop() {
            if s_item == &Operand::OpenParen {
                return Err("no matching closing paren".to_string());
            }

            postfix.push(s_item);
        }

        return Ok(postfix);
    }

    fn consume_string(&mut self) -> Result<Operand, String> {
        let mut tmp: String = String::with_capacity(50); // buffer to hold temp strings
                                                         // Already registered quote symbol " to start this function
        self.i.next();
        let mut is_closed = false;

        while let Some((_index, c)) = self.i.next() {
            if c == '"' {
                is_closed = true;
                break;
            } else {
                tmp.push(c);
            }
        }

        if is_closed {
            return Ok(Operand::String(tmp));
        } else {
            return Err("A string has no closing \"".to_string());
        }
    }

    fn consume_operator(&mut self) -> Result<Operand, String> {
        let mut tmp: String = String::with_capacity(50); // buffer to hold operator

        while let Some(&(_index, c)) = self.i.peek() {
            if check_if_operator(&c) {
                tmp.push(c);
                self.i.next();
            } else {
                break;
            }
        }

        let o = match tmp.as_str() {
            "+" => Some(Operator::Plus),
            "-" => Some(Operator::Substract),
            "*" => Some(Operator::Multiply),
            "/" => Some(Operator::Division),
            "=" => Some(Operator::E),
            "!=" => Some(Operator::NE),
            "<" => Some(Operator::L),
            "<=" => Some(Operator::LE),
            ">" => Some(Operator::G),
            ">=" => Some(Operator::GE),
            _ => None,
        };

        if let Some(token) = o {
            return Ok(Operand::OperatorToken(token));
        } else {
            return Err("unsupported operator".to_string());
        }
    }

    fn consume_variable(&mut self) -> Result<Operand, String> {
        let mut tmp: String = String::with_capacity(50); // buffer to hold temp variables

        while let Some(&(_index, c)) = self.i.peek() {
            if check_if_operand(&c) {
                tmp.push(c);
                self.i.next();
            } else {
                break;
            }
        }

        return Ok(Operand::Variable(tmp));
    }

    fn consume_number(&mut self) -> Result<Operand, String> {
        let mut tmp: String = String::with_capacity(50); // buffer to hold temp variables

        let mut has_dot = false;

        while let Some(&(_index, c)) = self.i.peek() {
            if tmp.len() == 0 && check_if_digit(&c) {
                tmp.push(c);
                self.i.next();
            } else if &c == &'.' {
                if has_dot == false {
                    has_dot = true;
                    tmp.push('.');
                    self.i.next();
                } else {
                    return Err(format!("number: multiple '.' at {}", _index));
                }
            } else if check_if_digit(&c) {
                tmp.push(c);
                self.i.next();
            } else {
                break;
            }
        }

        let res_number = tmp.parse::<f32>().unwrap();

        return Ok(Operand::Number(res_number));
    }
}

fn check_if_operand(c: &char) -> bool {
    (c >= &'a' && c <= &'z') || (c >= &'A' && c <= &'Z')
}

fn check_if_operator(c: &char) -> bool {
    ['+', '-', '/', '*', '<', '=', '>', '!'].contains(c)
}

fn check_if_digit(c: &char) -> bool {
    c >= &'0' && c <= &'9'
}

/*
            3
          ____
        __|__|___
        |2 level|      * /
    ____|_______|____
    |    1 Level    |  + -
 ______________________
|  0 other ... =, >, <  |

*/

fn precedence(c: &Operator) -> i32 {
    match c {
        Operator::Plus | Operator::Substract => 1,
        Operator::Division | Operator::Multiply => 2,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_expression() -> Result<(), String> {
        let formula = "100.00<=
 ((aA+(b*c))-d*2 )";

        let mut parser = Tokenizer::new(&formula);
        let res = parser.parse();
        assert!(res.is_ok());
        assert!(parser.operands.len() == 17);

        let postfix = parser.to_postfix()?;
        assert!(postfix.len() == 11);
        assert_eq!(
            postfix,
            vec![
                &Operand::Number(100.0),
                &Operand::Variable("aA".to_string()),
                &Operand::Variable("b".to_string()),
                &Operand::Variable("c".to_string()),
                &Operand::OperatorToken(Operator::Multiply),
                &Operand::OperatorToken(Operator::Plus),
                &Operand::Variable("d".to_string()),
                &Operand::Number(2.0),
                &Operand::OperatorToken(Operator::Multiply),
                &Operand::OperatorToken(Operator::Substract),
                &Operand::OperatorToken(Operator::LE),
            ]
        );
        Ok(())
    }
    #[test]
    fn parses_no_paren_expression() -> Result<(), String> {
        let formula = "aA+b *c-d*2 ";

        let mut parser = Tokenizer::new(&formula);
        let res = parser.parse();
        assert!(res.is_ok());
        assert!(parser.operands.len() == 9);

        let postfix = parser.to_postfix()?;
        assert!(postfix.len() == 9);
        assert_eq!(
            postfix,
            vec![
                &Operand::Variable("aA".to_string()),
                &Operand::Variable("b".to_string()),
                &Operand::Variable("c".to_string()),
                &Operand::OperatorToken(Operator::Multiply),
                &Operand::OperatorToken(Operator::Plus),
                &Operand::Variable("d".to_string()),
                &Operand::Number(2.0),
                &Operand::OperatorToken(Operator::Multiply),
                &Operand::OperatorToken(Operator::Substract),
            ]
        );
        Ok(())
    }

    #[test]
    fn fails_expression_unknown_symbol() {
        let formula = "100.00<)^";
        let mut parser = Tokenizer::new(&formula);
        let res = parser.parse();
        assert!(!res.is_ok());
    }

    #[test]
    fn fails_expression_no_open_paren() -> Result<(), String> {
        let formula = "100.00)";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;
        let postfix = parser.to_postfix();
        assert!(!postfix.is_ok());
        Ok(())
    }

    #[test]
    fn fails_expression_no_closing_paren() -> Result<(), String> {
        let formula = "100.00(";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;

        let postfix = parser.to_postfix();
        assert!(!postfix.is_ok());
        Ok(())
    }

    #[test]
    fn succeeds_single_element() -> Result<(), String> {
        let formula = "\"hello\"";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;

        let postfix = parser.to_postfix();
        assert!(postfix.is_ok());

        assert_eq!(postfix?, vec![&Operand::String("hello".to_string())]);
        Ok(())
    }
}
