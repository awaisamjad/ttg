use crate::constants::{HELP_MESSAGE, OPERATION_FILE, QUIT_OPTIONS, STATEMENT_FILE};
use crate::repl::{operation, statement};
use crate::utils::{append_to_file, delete_files};

// pub fn new() -> Result<(), Box<dyn std::error::Error>> {
//     while {
//         let mut buffer = String::new();
//         print!("ttg> ");

//         std::io::stdout().flush()?; // Ensure the prompt is displayed immediately
//         buffer.clear();

//         std::io::stdin().read_line(&mut buffer)?;
//         let input = buffer.trim();

//         match input {
//             "help" => {
//                 println!("{}", HELP_MESSAGE);
//                 true
//             }
//             "man" => {
//                 println!("man command");
//                 true
//             }

//             //? If input is CTRL-C then quit properly
//             // ! Doesnt work. Maybe use ctrlc crate?
//             _ if input == "\u{3}" => {
//                 println!("quitting program");
//                 if std::fs::remove_file(STATEMENT_FILE).is_err()
//                     || std::fs::remove_file(OPERATION_FILE).is_err()
//                 {
//                     println!("Failed to delete the file");
//                 }
//                 false
//             }

//             _ if QUIT_OPTIONS.contains(&input) => {
//                 println!("quitting program");
//                 // TODO
//                 // delete_file()
//                 if std::fs::remove_file(STATEMENT_FILE).is_err()
//                     || std::fs::remove_file(OPERATION_FILE).is_err()
//                 {
//                     println!("Failed to delete the file");
//                 }
//                 false
//             }
//             _ => {
//                 // ! Change && to ||
//                 if statement::is_valid(&input) {
//                     println!("valid statement");
//                     append_to_file(STATEMENT_FILE, &input)?;
//                     true
//                 } else if operation::is_valid(&input) {
//                     println!("valid operation");
//                     append_to_file(OPERATION_FILE, &input)?;
//                     true
//                 } else {
//                     //TODO show how to create valid operation here
//                     println!("invalid operation. check help");
//                     true
//                 }
//             }
//         }
//     } {}
//     Ok(())
// }
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub fn new() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;

    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("ttg> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                rl.add_history_entry(input);

                match input {
                    _ if QUIT_OPTIONS.contains(&input) => {
                        println!("quitting program");
                        delete_files();
                        break;
                    }
                    "help" => {
                        println!("{}", HELP_MESSAGE);
                    }
                    "man" => {
                        println!("man command");
                    }
                    _ => {
                        if statement::is_valid(&input) {
                            println!("valid statement");
                            append_to_file(STATEMENT_FILE, &input)?;
                        } else if operation::is_valid(&input) {
                            println!("valid operation");
                            append_to_file(OPERATION_FILE, &input)?;
                        } else {
                            //TODO show how to create valid operation here
                            println!("invalid operation. check help");
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                delete_files();
                break;
            }
            // ! Dont see the point of this
            Err(ReadlineError::Eof) => {
                delete_files();
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                delete_files();
                break;
            }
        }
    }

    Ok(())
}
