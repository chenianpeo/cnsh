use crate::error::Error;
use std::io::{self, Write};

pub fn line_editor() -> Result<String, Error> {
    print!("cnsh> ");
    io::stdout().flush()?;

    let mut line_buffer = String::new();
    io::stdin().read_line(&mut line_buffer)?;
    // Trim end remove line buffer end switch line symbol
    Ok(line_buffer.trim_end().to_string())
}

#[derive(Debug)]
#[warn(unused)]
pub enum EditorKey {
    Enter,

    Char(char),

    Ctrl(char),
    Tab,
    Esc,

    Left,
    Right,
    Up,
    Down,
}
