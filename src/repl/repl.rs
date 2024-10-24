use crate::constants::{HELP_MESSAGE, OPERATION_FILE, QUIT_OPTIONS, STATEMENT_FILE};
use crate::repl::{operation, statement};
use crate::utils::{append_to_file, delete_files};

use rustyline::completion::Completer;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::validate::Validator;
use rustyline::{DefaultEditor, EditMode, Helper};

// #[derive(Helper, Completer, Hinter, Validator)]
// struct MyHelper {
//     #[rustyline(Completer)]
//     completer: rustyline::completion::FilenameCompleter,
//     highlighter: rustyline::highlight::MatchingBracketHighlighter,
//     #[rustyline(Validator)]
//     validator: rustyline::validate::MatchingBracketValidator,
//     #[rustyline(Hinter)]
//     hinter: rustyline::hint::HistoryHinter,
//     colored_prompt: String,
// }

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = rustyline::Config::builder()
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    // let h = MyHelper {
    //     completer: rustyline::completion::FilenameCompleter::new(),
    //     highlighter: rustyline::highlight::MatchingBracketHighlighter::new(),
    //     hinter: rustyline::hint::HistoryHinter::new(),
    //     colored_prompt: "".to_owned(),
    //     validator: rustyline::validate::MatchingBracketValidator::new(),
    // };
    let mut rl = rustyline::Editor::<(), rustyline::history::FileHistory>::with_config(config)?;
    // rl.set_helper(Some(h));
    // let mut rl: rustyline::Editor<(), rustyline::history::FileHistory> = rustyline::DefaultEditor::new()?;
    rl.bind_sequence(
        rustyline::KeyEvent::alt('n'),
        rustyline::Cmd::HistorySearchForward,
    );
    rl.bind_sequence(
        rustyline::KeyEvent::alt('p'),
        rustyline::Cmd::HistorySearchBackward,
    );

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("ttg> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                let _ = rl.add_history_entry(input);

                match input {
                    _ if QUIT_OPTIONS.contains(&input) => {
                        println!("quitting program");
                        delete_files();
                        break;
                    }
                    //? Press enter to just get new lines
                    "" => {}
                    "clear" => {
                        let _ = rl.clear_screen();
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
