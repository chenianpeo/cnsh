use crate::{
    cli::{ExecMode, cli},
    editor::line_editor,
    error::Error,
};

pub fn run() -> Result<(), Error> {
    // Parse external commands arguments
    // split to four run mode: exec, real eval print loop, script, special
    //
    // Exec and script mode need to provide relate parameters, real print mode don't need parameters
    // special option include help and version, etc. run procedure before parse arguments
    let exec_mode = cli()?;

    // Parse external command arguments to c sh language base
    // Special options don't into parse mod, run ahead of time before parse
    //
    // Exec and script mode parse arguments to inner syntax
    //
    // Real eval print loop obtain arguments after interactive line editor
    // String obtain in interactive ending
    execute(exec_mode)?;

    Ok(())
}

// Run procedure by different mode
pub fn execute(exec_mode: ExecMode) -> Result<(), Error> {
    match exec_mode {
        ExecMode::Exec(exec_cli) => {
            println!("exec, {}", exec_cli);
        }

        ExecMode::Repl => loop {
            let repl_cli = line_editor()?;

            if repl_cli.is_empty() {
                continue;
            }
            if repl_cli == "exit" {
                break;
            }

            println!("{}", repl_cli);
        },

        ExecMode::Script(script_cli) => {
            println!("script, {}", script_cli);
        }

        ExecMode::Special(option) => {
            option.execute()?;
        }
    }

    Ok(())
}
