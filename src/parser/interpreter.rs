use super::operand::{Operand, Operator};
use super::tokenizer::Tokenizer;

pub fn interpret(postfix: &Vec<Operand>) -> Vec<Operand> {
    let mut stack: Vec<Operand> = Vec::with_capacity(postfix.len());

    for p in postfix {
        if let Operand::OperatorToken(o) = p {
            let r = stack.pop().unwrap();
            let l = stack.pop().unwrap();

            match o {
                Operator::Plus => {
                    stack.push(r + l);
                }
                Operator::Substract => {
                    stack.push(r - l);
                }
                _ => {}
            }
        } else {
            stack.push(p.clone().to_owned());
        }
    }

    stack
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn interpreter_succeeds_adding() -> Result<(), String> {
        let formula = "2+1";
        let mut tokenizer = Tokenizer::new(&formula);
        tokenizer.parse()?;
        let postfix = tokenizer.to_postfix();

        let formula_result = interpret(&postfix?);
        assert_eq!(formula_result, [Operand::Number(3.0)]);
        Ok(())
    }
}
