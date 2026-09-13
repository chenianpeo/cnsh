use std::process::ExitCode;

mod common;
mod error;
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
