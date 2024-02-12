use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Slash,
    Star,
    Sqrt,
    Undo,
    Caret,
    GroupOpen,
    GroupClose,
    Unknown(String)
}

impl Token {

    pub fn new(token: &str) -> Token {
        match token {
            // Operations
            "+" => Token::Plus,
            "-" => Token::Minus,
            "/" | "÷" => Token::Slash,
            "*" | "×" => Token::Star,
            "^" => Token::Caret,
            // Functions
            "sqrt" => Token::Sqrt,
            "undo" => Token::Undo,
            // Grouping
            "(" => Token::GroupOpen,
            ")" => Token::GroupClose,
            other =>
                other.parse::<f64>()
                    .map(|n| Token::Number(n))
                    .unwrap_or(Token::Unknown(String::from(other)))
        }
    }

    pub fn shunting_yard(input: &str) -> Vec<Token> {
        let mut operators: VecDeque<Token> = VecDeque::new();
        let mut output = Vec::new();

        for token in input.split_ascii_whitespace().map(|t| Token::new(t)) {
            match token {
                // groups
                Token::GroupOpen => {
                    operators.push_front(token)
                }
                Token::GroupClose => {
                    while let Some(op) = operators.pop_front() {
                        if op == Token::GroupOpen {
                            break
                        } else {
                            output.push(op)
                        }
                    }
                }
                // operators
                Token::Plus | Token::Minus | Token::Slash | Token::Star | Token::Sqrt | Token::Caret => {
                    while let Some(last) = operators.get(0) {
                        if last.order() >= token.order() &&
                            *last != Token::GroupOpen &&
                            *last != Token::Caret
                        {
                            let t = operators.pop_front().unwrap();
                            output.push(t);
                        } else {
                            break
                        }
                    }
                    operators.push_front(token);
                }
                // operands
                Token::Number(_) | _ =>
                    output.push(token),
            }
        }

        for operator in operators {
            output.push(operator)
        }

        output
    }

    /// Rules that reflect conventions about which operations to perform first in order to evaluate
    /// a given mathematical expression.
    /// https://en.wikipedia.org/wiki/Order_of_operations
    pub fn order(&self) -> i8 {
        match self {
            Token::Number(_) | Token::Unknown(_) | Token::Undo => 0,
            // addition and subtraction
            Token::Plus | Token::Minus => 1,
            // multiplication and division
            Token::Star | Token::Slash => 3,
            // exponentiation
            Token::Caret | Token::Sqrt => 4,
            // parenthesis
            Token::GroupOpen | Token::GroupClose => i8::MAX
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Slash => write!(f, "/"),
            Token::Star => write!(f, "*"),
            Token::Caret => write!(f, "^"),
            Token::Sqrt => write!(f, "sqrt"),
            Token::Undo => write!(f, ""),
            Token::GroupOpen => write!(f, ""),
            Token::GroupClose => write!(f, ""),
            Token::Unknown(u) => write!(f, "{}", u),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use super::*;

    #[test]
    fn should_parse_operators() {
        assert_eq!(Token::new("+"), Token::Plus);
        assert_eq!(Token::new("-"), Token::Minus);
        assert_eq!(Token::new("/"), Token::Slash);
        assert_eq!(Token::new("*"), Token::Star);
        assert_eq!(Token::new("^"), Token::Caret);
        assert_eq!(Token::new("("), Token::GroupOpen);
        assert_eq!(Token::new(")"), Token::GroupClose);
        assert_eq!(Token::new("sqrt"), Token::Sqrt);
        assert_eq!(Token::new("undo"), Token::Undo);
        assert_eq!(Token::new("xxx"), Token::Unknown("xxx".to_string()));
    }

    #[test]
    fn should_parse_operands() {
        assert_eq!(Token::new("1"), Token::Number(1.0));
        assert_eq!(Token::new("1.2"), Token::Number(1.2));
    }

    #[test]
    fn should_shunting_yard() {
        let tokens = Token::shunting_yard("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3");
        let str = format!("{}", tokens.iter().map(|t| t.to_string()).join(" "));
        assert_eq!(str, "3 4 2 * 1 5 - 2 3 ^ ^ / +".to_string());
    }
}