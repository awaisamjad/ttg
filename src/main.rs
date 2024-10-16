use std::io::Write;

const AND: (&str, &str) = ("^", "AND");
const OR: (&str, &str) = ("v", "OR");

const QUIT_OPTIONS: [&str; 3] = ["quit", "exit", "q"];

const HELP_MESSAGE : &str = "
    This is `ttg` which stands for Truth Table Generator
    Through the interactive shell, you can create logical statements, perform operations on them and other statements and generate truth tables

    Enter the following to exit : quit | q | exit

    Create a statement : NAME = [value1, value2 ...];
        Example : p1 = [t,t,f,f];

        Rules : 
            1. Values can be as t/true and f/false only.
                1. Case insensitive
            2. Equals must be used
            3. NAME MUST be mixture of letters and numbers
                1. NO special characters except for underscore and hyphen (easier reading)
            4. Line must end in semi colon or will continue to line break until finished
";

#[derive(Debug)]
enum OperationError {
    LengthsNotEqual,
    ZeroLength,
}

#[repr(u8)]
#[derive(PartialEq)]
enum Value {
    TRUE,
    FALSE,
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::TRUE => write!(f, "TRUE"),
            Value::FALSE => write!(f, "FALSE"),
        }
    }
}

trait OPERATIONS {
    fn and(self, s2: &Statement) -> Result<Statement, OperationError>;
    fn or(self, s2: &Statement) -> Result<Statement, OperationError>;
    fn not(self) -> Result<Statement, OperationError>;
    fn xor(self, s2: &Statement) -> Result<Statement, OperationError>;
    fn xnor(self, s2: &Statement) -> Result<Statement, OperationError>;
    fn if_then(self, s2: &Statement) -> Result<Statement, OperationError>;
    fn r#if(self, s2: &Statement) -> Result<Statement, OperationError>;
    fn if_and_only_if(self, s2: &Statement) -> Result<Statement, OperationError>;
}

#[derive(Debug)]
struct Statement {
    fields: Vec<Value>,
}

impl OPERATIONS for Statement {
    fn and(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let mut result_fields = Vec::new();

        for (field1, field2) in self.fields.iter().zip(s2.fields.iter()) {
            let result_field = if *field1 == Value::TRUE && *field2 == Value::TRUE {
                Value::TRUE
            } else {
                Value::FALSE
            };
            result_fields.push(result_field);
        }

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn not(self) -> Result<Statement, OperationError> {
        let result_fields: Vec<Value> = self
            .fields
            .iter()
            .map(|field| {
                if field == &Value::TRUE {
                    Value::FALSE
                } else {
                    Value::TRUE
                }
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn or(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let result_fields = self
            .fields
            .iter()
            .zip(&s2.fields)
            .map(|(value1, value2)| match (value1, value2) {
                (Value::FALSE, Value::FALSE) => Value::FALSE,
                _ => Value::TRUE,
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn xor(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let result_fields = self
            .fields
            .iter()
            .zip(&s2.fields)
            .map(|(value1, value2)| match (value1, value2) {
                (Value::FALSE, Value::FALSE) => Value::FALSE,
                (Value::TRUE, Value::TRUE) => Value::FALSE,
                _ => Value::TRUE,
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn xnor(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let result_fields = self
            .fields
            .iter()
            .zip(&s2.fields)
            .map(|(value1, value2)| match (value1, value2) {
                (Value::FALSE, Value::FALSE) => Value::TRUE,
                (Value::TRUE, Value::TRUE) => Value::TRUE,
                _ => Value::FALSE,
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn if_then(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let result_fields = self
            .fields
            .iter()
            .zip(&s2.fields)
            .map(|(value1, value2)| match (value1, value2) {
                (Value::TRUE, Value::FALSE) => Value::FALSE,
                _ => Value::TRUE,
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn r#if(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let result_fields = self
            .fields
            .iter()
            .zip(&s2.fields)
            .map(|(value1, value2)| match (value1, value2) {
                (Value::FALSE, Value::TRUE) => Value::FALSE,
                _ => Value::TRUE,
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn if_and_only_if(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let result_fields = self
            .fields
            .iter()
            .zip(&s2.fields)
            .map(|(value1, value2)| match (value1, value2) {
                (Value::FALSE, Value::FALSE) => Value::TRUE,
                (Value::TRUE, Value::TRUE) => Value::TRUE,
                _ => Value::FALSE,
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p1 = Statement {
        fields: vec![
            Value::TRUE,
            Value::TRUE,
            Value::FALSE,
            Value::FALSE,
            Value::TRUE,
            Value::TRUE,
            Value::FALSE,
            Value::FALSE,
            Value::TRUE,
            Value::TRUE,
            Value::FALSE,
            Value::FALSE,
        ],
    };

    let p2 = Statement {
        fields: vec![
            Value::TRUE,
            Value::FALSE,
            Value::FALSE,
            Value::TRUE,
            Value::TRUE,
            Value::FALSE,
            Value::FALSE,
            Value::TRUE,
            Value::TRUE,
            Value::FALSE,
            Value::FALSE,
        ],
    };

    let result = p1.if_and_only_if(&p2);
    // match result {
    //     Ok(value) => println!("{:?}", value),
    //     Err(err) => println!("{:?}", err),
    // }
    // let result = p1.not();

    // for value in result.fields {
    //     println!("{:?}", value);
    // }

    //TODO if user enters `ttg` then go to interactive shell, if they enter example:`ttg -s [t,t,f,f] --operation not` then give cli like experience

    let mut buffer = String::new();
    println!("TTG version 0.0.1");
    println!("Enter `help` for instructions");

    while {
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
            _ if QUIT_OPTIONS.contains(&input) => {
                println!("quitting program");
                // TODO
                // delete_file()
                false
            }
            _ => {
                if is_valid_operation(&input) {
                    println!("valid operation");
                    append_to_file("file.txt", &input)?;
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

//? Check if the input is valid
//? this includes the following : declaring a statement, performing an operation on a statement or multiple,
//TODO functionality should probably be split up
fn is_valid_operation(input: &str) -> bool {

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
    let values : Vec<&str> = input.split("=").collect();

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
    let allowed_truth_and_false_values = [
        "t", "f", "truth", "false", "true", "false", "T", "F", "TRUTH", "FALSE", "TRUE", "FALSE",
    ];

    if !value.starts_with("[") && !value.ends_with("]") {
        return false;
    }

    let value = value.replace("[", "").replace("]", "");
    let value = value.split(",");

    for v in value {
        if !allowed_truth_and_false_values.contains(&v.trim()) {
            println!("Invalid truth value: {}", v);
            return false;
        }
    }
    true
}

fn is_valid_statement(statement: &str) -> bool {
    false
}

fn append_to_file(filename: &str, text: &str) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    writeln!(file, "{}", text)?;
    Ok(())
}
