use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "values")]
pub enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Sqrt(Box<Expr>),
    Power(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Number(value) =>
                *value,
            Expr::Add(x, y) =>
                x.eval() + y.eval(),
            Expr::Subtract(x, y) =>
                x.eval() - y.eval(),
            Expr::Divide(x, y) =>
                x.eval() / y.eval(),
            Expr::Multiply(x, y) =>
                x.eval() * y.eval(),
            Expr::Sqrt(x) =>
                x.eval().sqrt(),
            Expr::Power(x, y) =>
                x.eval().powf(y.eval()),
        }
    }

    pub fn undo(&self) -> VecDeque<&Expr> {
        match self {
            Expr::Add(x, y) |
            Expr::Subtract(x, y) |
            Expr::Divide(x, y) |
            Expr::Multiply(x, y) |
            Expr::Power(x, y) => {
                let mut q = VecDeque::with_capacity(2);
                q.push_back(x.as_ref());
                q.push_back(y.as_ref());
                q
            }
            Expr::Sqrt(x) => {
                let mut q = VecDeque::with_capacity(1);
                q.push_back(x.as_ref());
                q
            }
            _ => VecDeque::with_capacity(0)
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Add(x, y) => write!(f, "{} {} +", x, y),
            Expr::Subtract(x, y) => write!(f, "{} {} -", x, y),
            Expr::Divide(x, y) => write!(f, "{} {} /", x, y),
            Expr::Multiply(x, y) => write!(f, "{} {} *", x, y),
            Expr::Power(x, y) => write!(f, "{} {} ^", x, y),
            Expr::Sqrt(x) => write!(f, "{} sqrt", x),
            Expr::Number(x) => write!(f, "{}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expr::*;

    #[test]
    fn add() {
        let expr = Add(Box::from(Number(2.0)), Box::from(Number(2.0)));
        assert_eq!(expr.eval(), 4.0);
        assert_eq!(format!("{expr}"), "2 2 +");
    }

    #[test]
    fn subtract() {
        let expr = Subtract(Box::from(Number(2.0)), Box::from(Number(2.0)));
        assert_eq!(expr.eval(), 0.0);
        assert_eq!(format!("{expr}"), "2 2 -");
    }

    #[test]
    fn multiply() {
        let expr = Multiply(Box::from(Number(2.0)), Box::from(Number(2.0)));
        assert_eq!(expr.eval(), 4.0);
        assert_eq!(format!("{expr}"), "2 2 *");
    }

    #[test]
    fn divide() {
        let expr = Divide(Box::from(Number(2.0)), Box::from(Number(2.0)));
        assert_eq!(expr.eval(), 1.0);
        assert_eq!(format!("{expr}"), "2 2 /");
    }

    #[test]
    fn pow() {
        let expr = Power(Box::from(Number(2.0)), Box::from(Number(2.0)));
        assert_eq!(expr.eval(), 4.0);
        assert_eq!(format!("{expr}"), "2 2 ^");
    }

    #[test]
    fn sqrt() {
        let expr = Sqrt(Box::from(Number(4.0)));
        assert_eq!(expr.eval(), 2.0);
        assert_eq!(format!("{expr}"), "4 sqrt");
    }

    #[test]
    fn undo_number() {
        let expr = Number(2.0);
        let mut undo = expr.undo();
        let x = undo.pop_front();
        assert!(x.is_none());
    }

    #[test]
    fn undo_add() {
        let expr = Add(Box::from(Number(2.0)), Box::from(Number(1.0)));
        let mut undo = expr.undo();
        let x = undo.pop_front().unwrap();
        let y = undo.pop_front().unwrap();
        assert_eq!(x.eval(), 2.0);
        assert_eq!(y.eval(), 1.0);
    }

    #[test]
    fn undo_subtract() {
        let expr = Subtract(Box::from(Number(2.0)), Box::from(Number(1.0)));
        let mut undo = expr.undo();
        let x = undo.pop_front().unwrap();
        let y = undo.pop_front().unwrap();
        assert_eq!(x.eval(), 2.0);
        assert_eq!(y.eval(), 1.0);
    }

    #[test]
    fn undo_multiply() {
        let expr = Multiply(Box::from(Number(2.0)), Box::from(Number(4.0)));
        let mut undo = expr.undo();
        let x = undo.pop_front().unwrap();
        let y = undo.pop_front().unwrap();
        assert_eq!(x.eval(), 2.0);
        assert_eq!(y.eval(), 4.0);
    }

    #[test]
    fn undo_divide() {
        let expr = Divide(Box::from(Number(2.0)), Box::from(Number(2.0)));
        let mut  undo = expr.undo();
        let x = undo.pop_front().unwrap();
        let y = undo.pop_front().unwrap();
        assert_eq!(x.eval(), 2.0);
        assert_eq!(y.eval(), 2.0);
    }

    #[test]
    fn undo_power() {
        let expr = Power(Box::from(Number(2.0)), Box::from(Number(2.0)));
        let mut  undo = expr.undo();
        let x = undo.pop_front().unwrap();
        let y = undo.pop_front().unwrap();
        assert_eq!(x.eval(), 2.0);
        assert_eq!(y.eval(), 2.0);
    }

    #[test]
    fn undo_sqrt() {
        let expr = Sqrt(Box::from(Number(25.0)));
        let mut undo = expr.undo();
        let x = undo.pop_front().unwrap();
        assert_eq!(x.eval(), 25.0);
    }
}