use crate::parser::Rule;
use crate::rational::Rational;
use pest::iterators::Pairs;
use pest::pratt_parser::{Assoc, Op, PrattParser};

pub struct Ast {
    pub root: Node,
}

pub enum Node {
    Number(Rational),
    Identifier(String),
    Binding {
        name: String,
        value: Box<Node>,
    },
    BinaryOp {
        op: BinaryOp,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    UnaryOp {
        op: UnaryOp,
        value: Box<Node>,
    },
    ToFloat(Box<Node>),
}

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    Power,
}

pub enum UnaryOp {
    Neg,
}

impl Ast {
    pub fn from_line(mut pairs: Pairs<Rule>) -> Self {
        let line = pairs.next().expect("Passed in an empty iterator");

        if line.as_rule() != Rule::line {
            panic!("Expected a line, got {:?}", line);
        }

        let inner = line.into_inner().next().unwrap();

        let root = match inner.as_rule() {
            Rule::binding => Node::from_binding(inner.into_inner()),
            Rule::expression => Node::from_expression(inner.into_inner()),
            _ => unreachable!(),
        };

        Self { root }
    }
}

impl Node {
    fn from_binding(mut pairs: Pairs<Rule>) -> Self {
        let name = pairs.next().unwrap().as_str().to_string();
        let value = Node::from_expression(pairs.next().unwrap().into_inner());

        Self::Binding {
            name,
            value: Box::new(value),
        }
    }

    fn from_expression(pairs: Pairs<Rule>) -> Self {
        thread_local! {
            static PRATT_PARSER: PrattParser<Rule> = PrattParser::new()
                .op(Op::infix(Rule::plus, Assoc::Left) | Op::infix(Rule::minus, Assoc::Left))
                .op(
                      Op::infix(Rule::times, Assoc::Left)
                    | Op::infix(Rule::implicit_times, Assoc::Left)
                    | Op::infix(Rule::divide, Assoc::Left)
                    | Op::infix(Rule::modulo, Assoc::Left)
                )
                .op(Op::infix(Rule::power, Assoc::Right))
                .op(Op::prefix(Rule::neg));
        }

        PRATT_PARSER.with(|parser| {
            parser
                .map_primary(|primary| match primary.as_rule() {
                    Rule::number => Node::from_number(primary.as_str()),
                    Rule::identifier => Node::Identifier(primary.as_str().to_string()),
                    Rule::expression => Node::from_expression(primary.into_inner()),
                    Rule::conversion => {
                        Node::ToFloat(Box::new(Node::from_expression(primary.into_inner())))
                    }
                    _ => unreachable!(),
                })
                .map_prefix(|op, rhs| match op.as_rule() {
                    Rule::neg => Node::UnaryOp {
                        op: UnaryOp::Neg,
                        value: Box::new(rhs),
                    },
                    _ => unreachable!(),
                })
                .map_infix(|lhs, op, rhs| {
                    let op = match op.as_rule() {
                        Rule::plus => BinaryOp::Add,
                        Rule::minus => BinaryOp::Sub,
                        Rule::times => BinaryOp::Mul,
                        Rule::divide => BinaryOp::Div,
                        Rule::modulo => BinaryOp::Modulo,
                        Rule::power => BinaryOp::Power,
                        Rule::implicit_times => BinaryOp::Mul,
                        _ => unreachable!(),
                    };

                    Node::BinaryOp {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                })
                .parse(pairs)
        })
    }

    fn from_number(number: &str) -> Self {
        let number = number.chars().filter(|c| *c != '_').collect::<String>();
        if let Some(number_of_decimals) = number.find('.').map(|n| number.len() - n - 1) {
            let numerator = number.replace('.', "").parse::<i128>().unwrap();
            let denominator = 10_i128.pow(number_of_decimals as u32);
            Self::Number(Rational::new(numerator, denominator))
        } else {
            Self::Number(Rational::new(number.parse::<i128>().unwrap(), 1))
        }
    }
}
