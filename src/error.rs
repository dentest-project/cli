use termion::{color, style};

pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Error {
        Error { message }
    }

    pub fn print_message(&self) {
        eprintln!(
            "{}{}Error{} {}",
            style::Bold,
            color::Fg(color::Red),
            style::Reset,
            self.message
        );
    }
}
