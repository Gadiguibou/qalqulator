use std::collections::HashMap;

use anyhow::anyhow;

use crate::{
    ast,
    rational::{PowResult, Rational},
};

#[derive(Clone, Copy)]
pub enum Number {
    FloatingPoint(f64),
    Rational(Rational),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::FloatingPoint(n) => write!(f, "{}", n),
            Number::Rational(n) => write!(f, "{}", n),
        }
    }
}

pub type Env = HashMap<String, Number>;

pub fn eval(ast: ast::Ast, env: &mut Env) -> anyhow::Result<Number> {
    eval_node(ast.root, env)
}

fn eval_node(node: ast::Node, env: &mut Env) -> anyhow::Result<Number> {
    match node {
        ast::Node::Number(n) => Ok(Number::Rational(n)),
        ast::Node::Identifier(name) => env
            .get(&name)
            .copied()
            .ok_or_else(|| anyhow!("Unknown variable: {}", name)),
        ast::Node::Binding { name, value } => {
            let value = eval_node(*value, env)?;
            env.insert(name, value);
            Ok(value)
        }
        ast::Node::UnaryOp { op, value } => {
            let value = eval_node(*value, env)?;
            match op {
                ast::UnaryOp::Neg => match value {
                    Number::FloatingPoint(n) => Ok(Number::FloatingPoint(-n)),
                    Number::Rational(n) => Ok(Number::Rational(-n)),
                },
            }
        }
        ast::Node::BinaryOp { op, lhs, rhs } => {
            let lhs = eval_node(*lhs, env)?;
            let rhs = eval_node(*rhs, env)?;

            macro_rules! call {
                ($trait:ty, $op:ident, $lhs:ident, $rhs:ident) => {
                    match ($lhs, $rhs) {
                        (Number::FloatingPoint(lhs), Number::FloatingPoint(rhs)) => {
                            Ok(Number::FloatingPoint(<f64 as $trait>::$op(lhs, rhs)))
                        }
                        (Number::Rational(lhs), Number::Rational(rhs)) => {
                            Ok(Number::Rational(<Rational as $trait>::$op(lhs, rhs)))
                        }
                        (Number::FloatingPoint(lhs), Number::Rational(rhs)) => Ok(
                            Number::FloatingPoint(<f64 as $trait>::$op(lhs, rhs.to_f64())),
                        ),
                        (Number::Rational(lhs), Number::FloatingPoint(rhs)) => Ok(
                            Number::FloatingPoint(<f64 as $trait>::$op(lhs.to_f64(), rhs)),
                        ),
                    }
                };
            }

            // TODO: Handle overflow safely
            match op {
                ast::BinaryOp::Add => call!(std::ops::Add, add, lhs, rhs),
                ast::BinaryOp::Sub => call!(std::ops::Sub, sub, lhs, rhs),
                ast::BinaryOp::Mul => call!(std::ops::Mul, mul, lhs, rhs),
                ast::BinaryOp::Div => call!(std::ops::Div, div, lhs, rhs),
                ast::BinaryOp::Modulo => call!(std::ops::Rem, rem, lhs, rhs),
                ast::BinaryOp::Power => match (lhs, rhs) {
                    (Number::FloatingPoint(lhs), Number::FloatingPoint(rhs)) => {
                        Ok(Number::FloatingPoint(lhs.powf(rhs)))
                    }
                    (Number::Rational(lhs), Number::Rational(rhs)) => Ok(match lhs.pow(rhs) {
                        PowResult::Rational(r) => Number::Rational(r),
                        PowResult::FloatingPoint(f) => Number::FloatingPoint(f),
                    }),
                    (Number::FloatingPoint(lhs), Number::Rational(rhs)) => {
                        Ok(Number::FloatingPoint(lhs.powf(rhs.to_f64())))
                    }
                    (Number::Rational(lhs), Number::FloatingPoint(rhs)) => {
                        Ok(Number::FloatingPoint(lhs.to_f64().powf(rhs)))
                    }
                },
            }
        }
        ast::Node::ToFloat(expr) => {
            let expr = eval_node(*expr, env)?;
            match expr {
                Number::FloatingPoint(n) => Ok(Number::FloatingPoint(n)),
                Number::Rational(n) => Ok(Number::FloatingPoint(n.to_f64())),
            }
        }
    }
}
