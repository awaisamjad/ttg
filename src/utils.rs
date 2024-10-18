//? Helpful utility functions

use std::io::Write;

pub fn append_to_file(filename: &str, text: &str) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    writeln!(file, "{}", text)?;
    Ok(())
}
