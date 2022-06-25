use super::operand::{Operand, Operator};
use super::tokenizer::Tokenizer;

pub fn interpret(postfix: &Vec<Operand>) -> Vec<Operand> {
    let mut stack: Vec<Operand> = Vec::with_capacity(postfix.len());

    for p in postfix {
        if let Operand::OperatorToken(o) = p {
            let r = stack.pop().unwrap();
            let l = stack.pop().unwrap();

            match o {
                Operator::Plus => stack.push(r + l),

                Operator::Substract => stack.push(l - r),

                Operator::G => stack.push(Operand::Boolean(l > r)),

                Operator::GE => stack.push(Operand::Boolean(l >= r)),

                Operator::L => stack.push(Operand::Boolean(l < r)),

                Operator::LE => {
                    println!("--->{:?} {:?}", l, r);
                    stack.push(Operand::Boolean(l <= r));
                }

                Operator::E => stack.push(Operand::Boolean(l == r)),

                Operator::NE => stack.push(Operand::Boolean(l != r)),

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

    fn postfix_for(formula: &str) -> Result<Vec<Operand>, String> {
        let mut tokenizer = Tokenizer::new(&formula);
        tokenizer.parse()?;
        tokenizer.to_postfix()
    }

    #[test]
    fn interpreter_succeeds_adding() -> Result<(), String> {
        let postfix = postfix_for("2+1");
        let formula_result = interpret(&postfix?);
        assert_eq!(formula_result, [Operand::Number(3.0)]);
        Ok(())
    }

    #[test]
    fn interpreter_succeeds_substracting() -> Result<(), String> {
        let postfix = postfix_for("2-1");
        let formula_result = interpret(&postfix?);
        assert_eq!(formula_result, [Operand::Number(1.0)]);
        Ok(())
    }

    #[test]
    fn interpreter_succeeds_comparing_numbers_greater() -> Result<(), String> {
        let postfix = postfix_for("2>1");
        let formula_result = interpret(&postfix?);
        assert_eq!(formula_result, [Operand::Boolean(true)]);
        Ok(())
    }

    #[test]
    fn interpreter_succeeds_comparing_numbers_less() -> Result<(), String> {
        let postfix = postfix_for("20<1");
        let formula_result = interpret(&postfix?);
        assert_eq!(formula_result, [Operand::Boolean(false)]);
        Ok(())
    }

    #[test]
    fn interpreter_succeeds_comparing_non_eq_numbers() -> Result<(), String> {
        let postfix = postfix_for("20!=20");
        let formula_result = interpret(&postfix?);
        assert_eq!(formula_result, [Operand::Boolean(false)]);
        Ok(())
    }

    #[test]
    fn interpreter_succeeds_comparing_numbers_less_eq() -> Result<(), String> {
        let postfix = postfix_for("20<=20");
        let formula_result = interpret(&postfix?);
        assert_eq!(formula_result, [Operand::Boolean(true)]);
        Ok(())
    }

    #[test]
    fn interpreter_succeeds_comparing_numbers_ne() -> Result<(), String> {
        let postfix = postfix_for("20!=20");
        let formula_result = interpret(&postfix?);
        assert_eq!(formula_result, [Operand::Boolean(false)]);
        Ok(())
    }
}
