// Create a function that takes two arguments:
    // Enum of Color
    // &str of text to print to terminal

use std::io::{self, Write};


pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

pub fn paint(color: Colors, txt: &str) -> io::Result<()> {
    // start escape code: \x1b[3(color)m
    // end escape code: \x1b[30m

    let escape_code = b"\x1b[0m";

    let color_code = match color {
        Colors::Black => b"\x1b[30m",
        Colors::Red => b"\x1b[31m",
        Colors::Green => b"\x1b[32m",
        Colors::Yellow => b"\x1b[33m",
        Colors::Blue => b"\x1b[34m",
        Colors::Magenta => b"\x1b[35m",
        Colors::Cyan => b"\x1b[36m",
        Colors::White => b"\x1b[37m",
    };

    let full_line_in_bytes = [color_code, txt.as_bytes(), escape_code].concat();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(&full_line_in_bytes)?;

    Ok(())

}
