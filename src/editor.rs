use crate::error::Error;
use std::{
    fs::{OpenOptions, create_dir_all},
    io::{self, BufWriter, Write},
    path::PathBuf,
};

use crate::init::Env;

pub fn line_editor() -> Result<String, Error> {
    print!("CNSH> ");
    io::stdout().flush()?;

    let mut line_buffer = String::new();
    io::stdin().read_line(&mut line_buffer)?;

    // Trim end remove line buffer end switch line symbol
    let exec_args = line_buffer.trim_end().to_string();

    commands_history(&exec_args)?;

    Ok(exec_args)
}

#[cfg(target_os = "linux")]
fn commands_history(line_command: &str) -> Result<(), Error> {
    history_writer(Env::load()?.data_dir, line_command)?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn commands_history(line_command: &str) -> Result<(), Error> {
    history_writer(Env::load()?.data_dir, line_command)?;

    Ok(())
}

fn history_writer(path: PathBuf, line_command: &str) -> Result<(), Error> {
    create_dir_all(&path)?;
    let history_path = path.join("history.txt");

    let history_file = OpenOptions::new()
        .create(true)
        .append(true) // add content into file last
        .open(history_path)?;

    let mut history_buf = BufWriter::new(history_file);
    writeln!(history_buf, "{line_command}")?;

    history_buf.flush()?;

    Ok(())
}
