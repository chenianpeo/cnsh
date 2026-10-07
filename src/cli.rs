use std::env::{self};
use std::fmt::Debug;

use crate::error::Error;

// parse command line arguments
pub fn cli() -> Result<ExecMode, Error> {
    let mut cli_args: Vec<String> = env::args().collect();
    cli_args.remove(0);

    let mut exec_mode = ExecMode::Repl;

    if cli_args.is_empty() {
        exec_mode = ExecMode::Repl
    } else {
        let option = cli_args
            .first()
            .ok_or(Error::Arg("least provide 1 argument".into()))?;

        // conduct special command line arguments
        if option == "-h" {
            exec_mode = ExecMode::Special(Box::new(Help))
        }
        if option == "-v" {
            exec_mode = ExecMode::Special(Box::new(Version))
        }

        if option == "-c" {
            let is_provide_exec_args = cli_args.len() > 1;
            if !is_provide_exec_args {
                return Err(Error::Arg("least provide 1 argument".into()));
            }

            // collect execute arguments
            let exec_args = cli_args
                .iter()
                .skip(1)
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(" ");
            exec_mode = ExecMode::Exec(exec_args)
        }

        let options = ["-h", "-v", "-c"];
        let mut is_script_mode = true;
        for supported_option in options {
            if supported_option == option {
                is_script_mode = false;
            }
        }

        if is_script_mode {
            let script_args = cli_args.join(" ");
            exec_mode = ExecMode::Script(script_args)
        }
    }

    Ok(exec_mode)
}

#[derive(Debug)]
pub enum ExecMode {
    Exec(String),
    Repl,
    Script(String),

    Special(Box<dyn CliOptions>),
}

#[derive(Debug)]
pub struct Help;

#[derive(Debug)]
pub struct Version;

pub trait CliOptions: Debug {
    fn execute(&self) -> Result<(), Error>;
}

impl CliOptions for Help {
    fn execute(&self) -> Result<(), Error> {
        println!("{}", include_str!("../docs/help.txt"));
        Ok(())
    }
}

impl CliOptions for Version {
    fn execute(&self) -> Result<(), Error> {
        println!("{}", env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}
