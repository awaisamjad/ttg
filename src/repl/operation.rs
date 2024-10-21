//? This file checks to see if the input provided by the repl is a valid operation
//? Operations must follow one of the following syntax : p1.operation_name(p1);
//?     p1 and p2 must be previously defined

use crate::constants;

pub fn is_valid(input: &str) -> bool {
    //? check to see if line ends in semi-colon which is then removed so we deal only with NAME, =, VALUE
    if !input.ends_with(";") {
        println!("Missing semi-colon");
        return false;
    }

    let file = match std::fs::read_to_string(constants::STATEMENT_FILE) {
        Ok(content) => content,
        Err(_) => {
            println!("Failed to find statement");
            return false;
        }
    };

    let input: String = input.replace(";", "");
    if !check_for_statement_existence(file, &input) {
        println!("Statement does not exist")
    }
    //? This is only done to ensure that "=" exists as its not checked in `.split("=")`
    let values: Vec<&str> = input.split_whitespace().collect();

    false
}

//? Checks that a statement has been defined and therefore exists
fn check_for_statement_existence(file: String, statement_name: &str) -> bool {
    //? Goes in the reverse order to check the latest statement definition
    for line in file.lines().rev() {
        //? We can split at whitespace here because the lines in the file that have whitespace are valid as checked by the statement::is_valid
        // ! Assuming the statement as valid may be incorrect. Needs extra verifying
        // ! Taking the nth values may cause problems if whitespace is incorrect
        let statement_in_file = line.split_whitespace().nth(0);
        let value_in_file = line.split_whitespace().nth(2);

        match statement_in_file {
            Some(statement_in_file) => {
                //? If a statement is called by itself it must have a semi-colon so we remove it here
                let statement_in_file = statement_in_file.replace(";", "");
                if statement_name == statement_in_file.trim() {
                    match value_in_file {
                        Some(value) => println!("{}", value.replace(";", "")),
                        None => {}
                    }
                    return true;
                }
            }
            // TODO should probably do something here
            None => {}
        }
    }
    false
}
