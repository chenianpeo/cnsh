use std::process::ExitCode;

mod cli;
mod common;
mod editor;
mod error;
mod executor;
mod init;
mod lexer;
mod log;
mod parser;
mod run;

use crate::run::run;

fn main() -> ExitCode {
    let logger = log::Logger::new();
    logger.info("cnsh", "start to running procedure");

    match run() {
        Ok(_) => ExitCode::from(0),
        Err(err) => {
            logger.error("cnsh", &err.to_string());
            eprintln!("{}", err);
            ExitCode::from(2)
        }
    }
}
