mod fetch;

use std::io::stdout;
use crossterm::{
    queue,
    style::{PrintStyledContent, Stylize},
};

fn main() {
    let os = fetch::get_os();

    queue!(
        stdout(),
        PrintStyledContent(format!("os {}", os).cyan()),
    ).unwrap();
}

