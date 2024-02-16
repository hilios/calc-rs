pub mod expr;
pub mod token;

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

use crate::calc::expr::Expr;
use crate::calc::token::Token;
use itertools::join;

#[derive(Debug, Serialize, Deserialize)]
pub struct Calc {
    memory: Vec<Expr>,
}

pub enum Format<'a> {
    Infix(&'a str),
    Postfix(&'a str),
}

impl Calc {
    pub fn empty() -> Calc {
        Calc {
            memory: Vec::with_capacity(100),
        }
    }

    pub fn postfix(input: &str) -> Result<Calc, String> {
        let mut calc = Calc::empty();
        calc.input(Format::Postfix(input))?;
        Ok(calc)
    }

    pub fn infix(input: &str) -> Result<Calc, String> {
        let mut calc = Calc::empty();
        calc.input(Format::Infix(input))?;
        Ok(calc)
    }

    pub fn input(&mut self, input: Format) -> Result<(), String> {
        match input {
            Format::Infix(input) => {
                let tokens = Token::shunting_yard(input);
                for token in tokens {
                    self.parse_token(token)?;
                }
            }
            Format::Postfix(input) => {
                for token in input.split_ascii_whitespace() {
                    let token = Token::new(token);
                    self.parse_token(token)?;
                }
            }
        }
        Ok(())
    }

    pub fn eval(&self) -> Vec<f64> {
        self.memory.iter().map(|e| e.eval()).collect()
    }

    fn parse_token(&mut self, token: Token) -> Result<(), String> {
        match token {
            Token::Plus => {
                let y = self.memory.pop().ok_or("Missing operands")?;
                let x = self.memory.pop().ok_or("Missing operand")?;
                let e = Expr::Add(Box::from(x), Box::from(y));
                self.memory.push(e);
            }
            Token::Minus => {
                let y = self.memory.pop().ok_or("Missing operands")?;
                let x = self.memory.pop().ok_or("Missing operand")?;
                let e = Expr::Subtract(Box::from(x), Box::from(y));
                self.memory.push(e);
            }
            Token::Slash => {
                let y = self.memory.pop().ok_or("Missing operands")?;
                let x = self.memory.pop().ok_or("Missing operand")?;
                let e = Expr::Divide(Box::from(x), Box::from(y));
                self.memory.push(e);
            }
            Token::Star => {
                let y = self.memory.pop().ok_or("Missing operands")?;
                let x = self.memory.pop().ok_or("Missing operand")?;
                let e = Expr::Multiply(Box::from(x), Box::from(y));
                self.memory.push(e);
            }
            Token::Caret => {
                let y = self.memory.pop().ok_or("Missing operands")?;
                let x = self.memory.pop().ok_or("Missing operand")?;
                let e = Expr::Power(Box::from(x), Box::from(y));
                self.memory.push(e);
            }
            Token::Sqrt => {
                let x = self.memory.pop().ok_or("Missing operand")?;
                let e = Expr::Sqrt(Box::from(x));
                self.memory.push(e);
            }
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
            Token::Unknown(t) => return Err(format!("Unknown token: {}", t)),
            _ => {}
        };
        Ok(())
    }
}

impl Display for Calc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&join(self.memory.clone(), " "))
    }
}

impl PartialEq for Calc {
    fn eq(&self, other: &Self) -> bool {
        self.eval() == other.eval()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("2", 2.0)]
    #[case("2 2 +", 4.0)]
    #[case("2 2 -", 0.0)]
    #[case("2 2 *", 4.0)]
    #[case("2 2 /", 1.0)]
    #[case("2 3 ^", 8.0)]
    #[case("4 sqrt", 2.0)]
    fn should_parse_postfix(#[case] input: &str, #[case] output: f64) {
        let calc = Calc::postfix(input).unwrap();
        let expr = calc.eval().pop().unwrap();
        assert_eq!(expr, output);
    }

    #[rstest]
    #[case("2", 2.0)]
    #[case("2 + 2", 4.0)]
    #[case("2 - 2", 0.0)]
    #[case("2 * 2", 4.0)]
    #[case("2 / 2", 1.0)]
    #[case("sqrt 4", 2.0)]
    #[case("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3", 3.0001220703125)]
    fn should_parse_infix(#[case] input: &str, #[case] output: f64) {
        let calc = Calc::infix(input).unwrap();
        let expr = calc.eval().pop().unwrap();
        assert_eq!(expr, output)
    }

    #[rstest]
    #[case("2", "")]
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
    #[case("_", "Unknown token: _")]
    #[case("+", "Missing operands")]
    #[case("1 +", "Missing operand")]
    #[case("-", "Missing operands")]
    #[case("2 -", "Missing operand")]
    #[case("*", "Missing operands")]
    #[case("3 *", "Missing operand")]
    #[case("/", "Missing operands")]
    #[case("4 /", "Missing operand")]
    #[case("^", "Missing operands")]
    #[case("4 ^", "Missing operand")]
    #[case("sqrt", "Missing operand")]
    #[case("undo", "Nothing to undo")]
    fn should_error(#[case] input: &str, #[case] error: &str) {
        let result = Calc::postfix(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[rstest]
    #[case("2 2 +", "2 2 *")]
    #[case("1 1 +", "4 2 /")]
    #[case("3 2 ^", "81 sqrt")]
    fn should_eq(#[case] a: &str, #[case] b: &str) {
        let x = Calc::postfix(a).unwrap();
        let y = Calc::postfix(b).unwrap();
        assert_eq!(x.eval(), y.eval());
    }
}
