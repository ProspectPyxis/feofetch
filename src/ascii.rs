use crossterm::{
    cursor::{MoveUp, RestorePosition, SavePosition},
    queue,
    style::Print,
};
use std::{fs, io::stdout, path::PathBuf};

/// # Returns
///
/// Returns a tuple of two `u16`s - the first value is the ascii art's maximum length, while the
/// second value is the amount of lines printed.
pub fn print_ascii(ascii_path: PathBuf, align_spaces: u16) -> (u16, u16) {
    let ascii = fs::read_to_string(&ascii_path).unwrap_or_else(|e| {
        panic!(
            "Unable to read ascii file \"{}\": {}",
            ascii_path.to_string_lossy(),
            e
        )
    });
    let ascii = ascii.trim_end();

    let max_ascii_line_length = ascii.lines().fold(0, |acc, x| acc.max(x.chars().count()));

    let mut stdout = stdout();

    queue!(
        stdout,
        SavePosition,
        Print(&ascii),
        RestorePosition,
        MoveUp((ascii.lines().count() - 1).try_into().expect("Loaded ascii has too many lines")),
    )
    .unwrap_or_else(|e| panic!("Error writing to stdout: {}", e));

    (
        max_ascii_line_length
            .try_into()
            .unwrap_or(u16::MAX - align_spaces)
            + align_spaces,
        ascii.lines().count().try_into().unwrap_or(u16::MAX),
    )
}
