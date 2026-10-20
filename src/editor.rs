use crate::error::Error;
use std::{
    fs::{OpenOptions, create_dir_all},
    io::{self, BufWriter, Write},
    path::PathBuf,
};

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
    history_writer(cnsh_path, line_command)?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn commands_history(line_command: &str) -> Result<(), Error> {
    let local_app_data = std::env::var("LOCALAPPDATA").map_err(|_| Error::Unfinished)?;

    let cnsh_path = PathBuf::from(local_app_data).join("cnsh");
    history_writer(cnsh_path, line_command)?;

    Ok(())
}

fn history_writer(path: PathBuf, line_command: &str) -> Result<(), Error> {
    create_dir_all(&path)?;
    let history_path = path.join("history.txt");

    // modify file open options
    let history_file = OpenOptions::new()
        .create(true) // auto create file if not exist
        .append(true) // append content, not delete history
        .open(history_path)?;

    let mut history_buf = BufWriter::new(history_file);
    writeln!(history_buf, "{line_command}")?;

    // instantly obtain commands and write to low-level
    history_buf.flush()?;

    Ok(())
}
