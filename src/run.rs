use std::{env, fmt::Debug};

use crate::error::Error;

pub fn run() -> Result<(), Error> {
    // Parse external commands arguments
    // split to four run mode: exec, real eval print loop, script, special
    //
    // Exec and script mode need to provide relate parameters, real print mode don't need parameters
    // special option include help and version, etc. run procedure before parse arguments

    // obtain command lines arguments
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    // initialize execute mode, default is real print
    let mut exec_mode = ExecMode::Repl;

    if args.is_empty() {
        exec_mode = ExecMode::Repl
    } else {
        let option = args
            .first()
            .ok_or(Error::Option("least provide 1 argument".into()))?;

        // special mode
        if option == "-h" {
            exec_mode = ExecMode::Special(Box::new(Help))
        }

        if option == "-v" {
            exec_mode = ExecMode::Special(Box::new(Version))
        }

        // exec mode
        if option == "-c" {
            let is_provide_exec_args = args.len() > 1;

            if !is_provide_exec_args {
                return Err(Error::Arg("need provide least 1 argument".into()));
            }

            let _exec_option = args.iter().skip(1);
            let exec_args = args.join(" ");
            exec_mode = ExecMode::Exec(exec_args)
        }

        // script mode
        let options = ["-h", "-v", "-c"];
        let mut is_script_mode = true;
        for supported_option in options {
            if supported_option == option {
                is_script_mode = false;
            }
        }

        if is_script_mode {
            let script_args = args.join(" ");
            exec_mode = ExecMode::Script(script_args)
        }
    }

    println!("{:?}", exec_mode);

    // Parse external command arguments to c sh language base
    // Special options don't into parse mod, run ahead of time before parse
    //
    // Exec and script mode parse arguments to inner syntax
    //
    // Real eval print loop obtain arguments after interactive line editor
    // String obtain in interactive ending
    if let ExecMode::Special(option) = exec_mode {
        option.execute();
    }

    Ok(())
}

// external command conduct mod
#[derive(Debug)]
pub enum ExecMode {
    Exec(String),
    Repl,
    Script(String),

    // external commands
    Special(Box<dyn CnOptions>),
}

#[derive(Debug)]
pub struct Help;

#[derive(Debug)]
pub struct Version;

pub trait CnOptions: Debug {
    fn execute(&self) -> Result<(), Error>;
}

impl CnOptions for Help {
    fn execute(&self) -> Result<(), Error> {
        println!("{}", include_str!("../docs/help.txt"));
        Ok(())
    }
}

impl CnOptions for Version {
    fn execute(&self) -> Result<(), Error> {
        println!("{}", env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}
