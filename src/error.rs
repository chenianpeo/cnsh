use std::io;

use crate::common::Color;

#[derive(Debug)]
pub enum Error {
    Arg(String),
    Io(io::Error),
    Option(String),

    // develop stage error
    Unfinished,
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl std::fmt::Display for Error {
    #[track_caller]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Arg(err) => {
                let err = format!("[ERROR] {}", err).red();
                write!(f, "{}", err)
            }
            Error::Io(err) => {
                let err = format!("[ERROR] {}", err).red();
                write!(f, "{}", err)
            }
            Error::Option(err) => {
                let err = format!("[ERROR] {}", err).red();
                write!(f, "{}", err)
            }
            Error::Unfinished => {
                let err = "[ERROR] Not Finished".red();
                write!(f, "{}", err)
            }
        }
    }
}
