// render
use std::fmt::Display;

pub trait Color: Display {
    fn color(&self, code: u8) -> String {
        format!("\x1b[{}m{}\x1b[0m", code, self)
    }
    fn red(&self) -> String {
        self.color(31)
    }
    fn _green(&self) -> String {
        self.color(32)
    }
    fn _yellow(&self) -> String {
        self.color(33)
    }
    fn _blue(&self) -> String {
        self.color(34)
    }
}

impl<T: Display> Color for T {}
