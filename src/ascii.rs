use std::{fs, path::PathBuf};

pub fn load_raw_ascii(ascii_path: PathBuf) -> String {
    fs::read_to_string(&ascii_path).unwrap_or_else(|e| {
        panic!(
            "Unable to read ascii file \"{}\": {}",
            ascii_path.to_string_lossy(),
            e
        )
    })
}
