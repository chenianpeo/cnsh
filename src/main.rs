use std::process::ExitCode;

mod cli;
mod common;
mod editor;
mod error;
mod lexer;
mod run;

use crate::run::run;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::from(0),
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::from(2)
        }
    }
}
