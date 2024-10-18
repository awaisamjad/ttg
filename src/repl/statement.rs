//? This file checks to see if the input provided by the repl is a valid statement

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

    return true;
}

fn is_name_valid(name: &str) -> bool {
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
