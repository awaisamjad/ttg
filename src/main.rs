use std::env;
mod constants;
mod gui;
mod operations;
mod repl;
mod utils;

use crate::gui::gui as GUI;
use crate::operations::operations::*;

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

    let result = p1.biconditional(&p2);
    // match result {
    //     Ok(value) => println!("{:?}", value),
    //     Err(err) => println!("{:?}", err),
    // }
    // let result = p1.not();

    // for value in result.fields {
    //     println!("{:?}", value);
    // }

    //TODO if user enters `ttg` then go to interactive shell, if they enter example:`ttg -s [t,t,f,f] --operation not` then give cli like experience

    // GUI::main();
    println!("TTG version 0.0.1");
    println!("Enter `help` for instructions");

    let args: Vec<String> = env::args().collect();

    if args.len() != 1 {
        println!("Using CLI");
    } else {
        let _ = repl::repl::main();
    }

    Ok(())
}
