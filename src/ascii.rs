use std::fs;

pub fn print_ascii(ascii_path: &str) -> u16 {
    let ascii = fs::read_to_string(ascii_path)
        .unwrap_or_else(|e| panic!("Unable to read ascii file \"{}\": {}", ascii_path, e));

    let max_ascii_line_length = ascii.lines().fold(0, |acc, x| acc.max(x.len()));
    max_ascii_line_length.try_into().unwrap_or(u16::MAX)
}
