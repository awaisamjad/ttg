//? This file checks to see if the input provided by the repl is a valid statement
//? Statements must follow the following syntax : NAME = [value1, value2, ...];

pub fn is_valid(input: &str) -> bool {
    //? check to see if line ends in semi-colon which is then removed so we deal only with NAME, =, VALUE
    if !input.ends_with(";") {
        println!("Missing semi-colon");
        return false;
    }

    let input: String = input.replace(";", "");

    //? This is only done to ensure that "=" exists as its not checked in `.split("=")`
    let values: Vec<&str> = input.split_whitespace().collect();

    //? This is done because there may be no whitespace so the split above doenst do anything
    //? This leads to a length of 1 which shouldnt happen and interferes with checking for "="
    if values.len() == 1 {
        println!("Invalid Syntax. Ensure spacing");
        return false;
    }

    if values[1].trim() != "=" {
        println!("Invalid Operator");
        return false;
    }

    //? Split by the '=' as it allows us to deal with both sides separately and we dont have to deal with whitespace
    //? This doesnt ensure that "=" exists as you can split on a value that doesnt exist so the check is made previously
    let values: Vec<&str> = input.split("=").collect();

    if values.len() != 2 {
        println!("Invalid input length");
        return false;
    }

    let name = values[0].trim();
    let values = values[1];

    if is_name_valid(name) == false {
        println!("Invalid Name");
        return false;
    }

    if is_value_valid(values) == false {
        println!("Invalid Truth Values");
        return false;
    }

    true
}

fn is_name_valid(name: &str) -> bool {

    // let disallowed_values = ("!", "^", "&", "|");

    if name.len() < 1 || name.len() > 10 {
        return false;
    }



    if !name.chars().all(|c| c.is_alphanumeric()) {
        return false;
    }

    true
}

fn is_value_valid(value: &str) -> bool {
    let allowed_true_and_false_values = [
        "t", "f", "true", "false", "T", "F", "TRUTH", "FALSE", "TRUE", "FALSE",
    ];

    if !value.starts_with("[") && !value.ends_with("]") {
        return false;
    }

    let value = value.replace("[", "").replace("]", "");
    let value = value.split(",");

    for v in value {
        if !allowed_true_and_false_values.contains(&v.trim()) {
            println!("Invalid truth value: {}", v);
            return false;
        }
    }
    true
}

mod tests {
    #[cfg(test)]
    mod tests {
        use crate::repl::statement::is_valid;

        use super::*;

        #[test]
        fn test_valid_statement() {
            assert!(is_valid("VAR1 = [true, false];"));
            assert!(is_valid("VAR2 = [t, f];"));
            assert!(is_valid("VAR3 = [T, F];"));
            assert!(is_valid("VAR4 = [TRUE, FALSE];"));
        }

        #[test]
        fn test_missing_semicolon() {
            assert!(!is_valid("VAR1 = [true, false]"));
        }

        #[test]
        fn test_invalid_operator() {
            assert!(!is_valid("VAR1 == [true, false];"));
            assert!(!is_valid("VAR1 : [true, false];"));
        }

        #[test]
        fn test_invalid_name() {
            // assert!(!is_valid("1VAR = [true, false];"));
            assert!(!is_valid("VAR_NAME_TOO_LONG = [true, false];"));
            assert!(!is_valid("VAR! = [true, false];"));
        }

        #[test]
        fn test_invalid_value_syntax() {
            assert!(!is_valid("VAR1 = true, false;"));
            assert!(!is_valid("VAR1 = [true false];"));
            assert!(!is_valid("VAR1 = [true, false"));
        }

        #[test]
        fn test_invalid_truth_values() {
            assert!(!is_valid("VAR1 = [yes, no];"));
            assert!(!is_valid("VAR1 = [1, 0];"));
            assert!(!is_valid("VAR1 = [TRUE, maybe];"));
        }

        #[test]
        fn test_empty_name() {
            assert!(!is_valid(" = [true, false];"));
        }

        #[test]
        fn test_empty_value() {
            assert!(!is_valid("VAR1 = [];"));
        }
    }
}