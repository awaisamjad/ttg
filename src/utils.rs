//? Helpful utility functions

use crate::constants::{OPERATION_FILE, STATEMENT_FILE};
use std::io::Write;

pub fn append_to_file(filename: &str, text: &str) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    writeln!(file, "{}", text)?;
    Ok(())
}

pub fn delete_files() {
    if let Err(e) = std::fs::remove_file(STATEMENT_FILE) {
        eprintln!("Failed to delete {}: {}", STATEMENT_FILE, e);
    }
    if let Err(e) = std::fs::remove_file(OPERATION_FILE) {
        eprintln!("Failed to delete {}: {}", OPERATION_FILE, e);
    }
}
