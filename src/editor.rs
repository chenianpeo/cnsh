use crate::{common::Color, error::Error};
use std::io::{self, Write};

pub fn line_editor() -> Result<String, Error> {
    print!("{}", "cnsh> ".green());
    io::stdout().flush()?;

    let mut line_buffer = String::new();
    io::stdin().read_line(&mut line_buffer)?;
    // Trim end remove line buffer end switch line symbol
    let exec_args = line_buffer.trim_end().to_string();

    commands_history(&exec_args)?;

    Ok(exec_args)
}

#[cfg(target_os = "linux")]
fn commands_history(line_args: &str) -> Result<(), Error> {
    use std::{
        fs::{OpenOptions, create_dir_all},
        io::BufWriter,
        path::PathBuf,
    };

    // obtain main directory environment variable
    let home = match std::env::var("HOME") {
        Ok(home) => home,
        Err(_) => return Err(Error::Unfinished),
    };

    let cnsh_path = PathBuf::from(home).join(".cnsh");
    create_dir_all(&cnsh_path)?;

    let history_path = cnsh_path.join("history.txt");

    // modify file open mode
    let history_file = OpenOptions::new()
        .create(true) // auto create
        .append(true) // append content, not delete history
        .open(history_path)?;

    let mut history_buf = BufWriter::new(history_file);
    writeln!(history_buf, "{line_args}")?;

    Ok(())
}

// todo
// windows history
