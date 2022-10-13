use directories::BaseDirs;
use pest::Parser as PestParser;
use qalqulator::ast::Ast;
use qalqulator::eval::{eval, Env, Number};
use qalqulator::parser::{Parser, Rule};
use rustyline::error::ReadlineError;
use rustyline::Config;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let mut editor =
        rustyline::Editor::<()>::with_config(Config::builder().auto_add_history(true).build())?;

    let history_file =
        BaseDirs::new().map(|base_dirs| base_dirs.data_dir().join("qalqulator/history.txt"));

    if let Some(history_file) = &history_file {
        if !history_file.exists() {
            std::fs::create_dir_all(history_file.parent().unwrap())?;
        }
        let _ = editor.load_history(history_file);
    }

    let mut env = HashMap::new();

    loop {
        match editor.readline(">>> ") {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                } else if line.trim() == "exit" {
                    break;
                }

                match run(&line, &mut env) {
                    Ok(result) => println!("{}", result),
                    Err(e) => eprintln!("{}", e),
                }
                if let Some(history_file) = &history_file {
                    let _ = editor.append_history(history_file);
                }
            }
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

fn run(line: &str, env: &mut Env) -> anyhow::Result<Number> {
    let parse = Parser::parse(Rule::line, line)?;
    let ast = Ast::from_line(parse);
    eval(ast, env)
}
