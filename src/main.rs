mod cli;
mod version;
mod executor;
mod lexer;
mod parser;
mod ast;

use version::show_version;
use cli::{parse_args, CliAction, show_help};
use executor::{execute_file, dump_tokens, dump_ast, syntax_check};
use std::env;

fn main() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("error: {}", info);
    }));
    let args: Vec<String> = env::args().collect();

    match parse_args(&args) {
        CliAction::ShowHelp => show_help(),
        CliAction::ShowVersion => show_version(),
        CliAction::RunFile(path) => execute_file(&path),
        CliAction::ShowTokens(path) => dump_tokens(&path),
        CliAction::ShowAst(path) => dump_ast(&path),
        CliAction::SyntaxCheck(path) => syntax_check(&path),
        CliAction::Invalid(arg) => {
            eprintln!("Unknown argument: {}", arg);
            show_help();
        }
    }
}