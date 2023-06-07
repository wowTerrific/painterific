#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python};
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

    let default_code = b"\x1b[0m";

    let full_line_in_bytes = [color_code, txt.as_bytes(), default_code].concat();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(&full_line_in_bytes)?;

    Ok(())
}