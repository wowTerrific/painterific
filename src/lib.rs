// #[macro_use]
// extern crate cpython;

use cpython::{PyResult, Python, py_module_initializer, py_fn};
use std::io::{self, Write};


// TODO: Not sure how to import this into cpython
// pub enum Colors {
//     Black,
//     Red,
//     Green,
//     Yellow,
//     Blue,
//     Magenta,
//     Cyan,
//     White,
// }

pub fn paint(color: &str, txt: &str) -> io::Result<()> {
    let color_code = match color {
        "black" => b"\x1b[30m",
        "red" => b"\x1b[31m",
        "green" => b"\x1b[32m",
        "yellow" => b"\x1b[33m",
        "blue" => b"\x1b[34m",
        "magenta" => b"\x1b[35m",
        "cyan" => b"\x1b[36m",
        "white" => b"\x1b[37m",
        _ => b"\x1b[00m",
    };

    let default_code = b"\x1b[0m";

    let full_line_in_bytes = [color_code, txt.as_bytes(), default_code].concat();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(&full_line_in_bytes)?;

    Ok(())

}

[#pymodule]
fn paint_py(_: Python, color: &str, txt: &str) -> PyResult<bool> {
    match paint(color, txt) {
        Ok(_) => Ok(true),
        Err(e) => panic!("There was an issue with Painter: {e}")
    }
}

py_module_initializer!(painterific, initpainterific, Pyinit_painterific, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "painterific", py_fn!(py, paint_py(color: &str, txt: &str)))?; 
    Ok(())
});