use ast::Ast;
use eval::eval;
use parser::Parser;
use parser::Rule;
use pest::Parser as PestParser;

pub mod ast;
pub mod eval;
pub mod parser;
pub mod rational;

pub fn run(line: &str, env: &mut eval::Env) -> anyhow::Result<eval::Number> {
    let parse = Parser::parse(Rule::line, line)?;
    let ast = Ast::from_line(parse);
    eval(ast, env)
}
