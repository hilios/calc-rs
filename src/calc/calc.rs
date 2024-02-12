use std::fmt::{Debug, Display, Formatter};

use itertools::join;

use super::expr::Expr;
use super::token::Token;

#[derive(Debug)]
struct Calc {
    memory: Vec<Expr>
}

impl Display for Calc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*join(self.memory.clone(), " "))
    }
}

impl Calc {

    pub fn postfix(input: &str) -> Result<Calc, String> {
        let memory = Vec::with_capacity(100);
        let mut calc = Calc { memory };
        for token in input.split_ascii_whitespace() {
            let token = Token::new(token);
            calc.parse_token(token)?;
        }
        Ok(calc)
    }


    pub fn infix(input: &str) -> Result<Calc, String> {
        let output = Token::shunting_yard(input);
        let memory = Vec::with_capacity(100);
        let mut calc = Calc { memory };
        for token in output {
            calc.parse_token(token)?;
        }
        Ok(calc)
    }

    fn parse_token(&mut self, token: Token) -> Result<(), String> {
        match token {
            Token::Plus => {
                let y = self.memory.pop().ok_or("Missing operands")?;
                let x = self.memory.pop().ok_or("Missing operand")?;
                let e = Expr::Add(Box::from(x), Box::from(y));
                self.memory.push(e);
            },
            Token::Minus => {
                let y = self.memory.pop().ok_or("Missing operands")?;
                let x = self.memory.pop().ok_or("Missing operand")?;
                let e = Expr::Subtract(Box::from(x), Box::from(y));
                self.memory.push(e);
            },
            Token::Slash => {
                let y = self.memory.pop().ok_or("Missing operands")?;
                let x = self.memory.pop().ok_or("Missing operand")?;
                let e = Expr::Divide(Box::from(x), Box::from(y));
                self.memory.push(e);
            },
            Token::Star => {
                let y = self.memory.pop().ok_or("Missing operands")?;
                let x = self.memory.pop().ok_or("Missing operand")?;
                let e = Expr::Multiply(Box::from(x), Box::from(y));
                self.memory.push(e);
            },
            Token::Caret => {
                let y = self.memory.pop().ok_or("Missing operands")?;
                let x = self.memory.pop().ok_or("Missing operand")?;
                let e = Expr::Power(Box::from(x), Box::from(y));
                self.memory.push(e);
            },
            Token::Sqrt => {
                let x = self.memory.pop().ok_or("Missing operand")?;
                let e = Expr::Sqrt(Box::from(x));
                self.memory.push(e);

            },
            Token::Undo => {
                let x = self.memory.pop().ok_or("Nothing to undo")?;
                for expr in x.undo() {
                    self.memory.push(expr.clone());
                }
            }
            Token::Number(n) => {
                let e = Expr::Number(n);
                self.memory.push(e);
            }
            Token::Unknown(t) => {
                return Err(format!("Unknown token: {}", t))
            }
            _ => {}
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case("2",      2.0)]
    #[case("2 2 +",  4.0)]
    #[case("2 2 -",  0.0)]
    #[case("2 2 *",  4.0)]
    #[case("2 2 /",  1.0)]
    #[case("2 3 ^",  8.0)]
    #[case("4 sqrt", 2.0)]
    fn should_parse_postfix(#[case] input: &str, #[case] output: f64) {
        let mut calc= Calc::postfix(input).unwrap();
        let expr = calc.memory.pop().unwrap();
        assert_eq!(expr.eval(), output);
    }

    #[rstest]
    #[case("2",      2.0)]
    #[case("2 + 2",  4.0)]
    #[case("2 - 2",  0.0)]
    #[case("2 * 2",  4.0)]
    #[case("2 / 2",  1.0)]
    #[case("sqrt 4", 2.0)]
    #[case("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3", 3.0001220703125)]
    fn should_parse_infix(#[case] input: &str, #[case] output: f64) {
        let infix = Calc::infix(input).unwrap();
        let expr = infix.memory.get(0).unwrap();
        assert_eq!(expr.eval(), output)
    }

    #[rstest]
    #[case("2",     "")]
    #[case("2 2 +", "2 2")]
    #[case("2 2 -", "2 2")]
    #[case("2 2 *", "2 2")]
    #[case("2 2 /", "2 2")]
    #[case("4 sqrt", "4")]
    fn should_undo(#[case] input: &str, #[case] output: &str) {
        let undo = format!("{} undo", input);
        let result = Calc::postfix(undo.as_str()).unwrap();
        assert_eq!(result.to_string(), output);
    }

    #[rstest]
    #[case("_",     "Unknown token: _")]
    #[case("+",     "Missing operands")]
    #[case("1 +",   "Missing operand")]
    #[case("-",     "Missing operands")]
    #[case("2 -",   "Missing operand")]
    #[case("*",     "Missing operands")]
    #[case("3 *",   "Missing operand")]
    #[case("/",     "Missing operands")]
    #[case("4 /",   "Missing operand")]
    #[case("^",     "Missing operands")]
    #[case("4 ^",   "Missing operand")]
    #[case("sqrt",  "Missing operand")]
    #[case("undo",  "Nothing to undo")]
    fn should_error(#[case] input: &str, #[case] error: &str) {
        let result = Calc::postfix(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }
}