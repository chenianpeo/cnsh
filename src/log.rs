use std::{
    fs::{OpenOptions, create_dir_all},
    io::BufWriter,
};

use crate::{error::Error, init::Env};

#[derive(Debug)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
        }
    }
}

pub struct Logger {
    max_level: LogLevel,
}

impl Logger {
    pub fn new(max_level: LogLevel) -> Self {
        Self { max_level }
    }

    fn log(&self, max_level: LogLevel, module: &str, message: &str) {
        let timestamp = timestamp();

        let _ = log_writer(&timestamp, max_level, module, message);
    }

    pub fn error(&self, module: &str, message: &str) {
        self.log(LogLevel::Error, module, message);
    }

    pub fn warn(&self, module: &str, message: &str) {
        self.log(LogLevel::Warn, module, message);
    }

    pub fn debug(&self, module: &str, message: &str) {
        self.log(LogLevel::Debug, module, message);
    }

    pub fn info(&self, module: &str, message: &str) {
        self.log(LogLevel::Info, module, message);
    }
}

// obtain system time by use third-party lib
fn timestamp() -> String {
    use chrono::Local;

    Local::now().format("%d/%m/%Y %H:%M:%S").to_string()
}

fn log_writer(time: &str, log_level: LogLevel, module: &str, message: &str) -> Result<(), Error> {
    use std::io::{self, Write};

    let mut stderr = io::stderr().lock();
    let log_msg = format!(
        "[{}] [{}] [{}] {}",
        time,
        log_level.as_str(),
        module,
        message
    );

    let data_path = Env::load()?.data_dir;
    create_dir_all(&data_path)?;
    let log_path = data_path.join("log.txt");

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let mut log_buf = BufWriter::new(log_file);
    writeln!(log_buf, "{}", log_msg)?;
    log_buf.flush()?;

    writeln!(stderr, "{}", log_msg)?;

    Ok(())
}
