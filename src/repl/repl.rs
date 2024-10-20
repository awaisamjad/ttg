use std::io::Write;

use crate::constants::{HELP_MESSAGE, OUTPUT_FILE, QUIT_OPTIONS};
use crate::repl::operation;
use crate::repl::statement;
use crate::utils::append_to_file;

pub fn new() -> Result<(), Box<dyn std::error::Error>> {
    while {
        let mut buffer = String::new();
        print!("ttg> ");

        std::io::stdout().flush()?; // Ensure the prompt is displayed immediately
        buffer.clear();

        std::io::stdin().read_line(&mut buffer)?;
        let input = buffer.trim();

        match input {
            "help" => {
                println!("{}", HELP_MESSAGE);
                true
            }
            "man" => {
                println!("man command");
                true
            }

            //? If input is CTRL-C then quit properly
            // ! Doesnt work. Maybe use ctrlc crate?
            _ if input == "\u{3}" => {
                println!("quitting program");
                if std::fs::remove_file(OUTPUT_FILE).is_err() {
                    println!("Failed to delete the file");
                }
                false
            }

            _ if QUIT_OPTIONS.contains(&input) => {
                println!("quitting program");
                // TODO
                // delete_file()
                if std::fs::remove_file(OUTPUT_FILE).is_err() {
                    println!("Failed to delete the file");
                }
                false
            }
            _ => {
                // ! Change && to ||
                if statement::is_valid(&input) && operation::is_valid(&input) {
                    println!("valid operation");
                    append_to_file(OUTPUT_FILE, &input)?;
                    true
                } else {
                    //TODO show how to create valid operation here
                    println!("invalid operation. check help");
                    true
                }
            }
        }
    } {}
    Ok(())
}
