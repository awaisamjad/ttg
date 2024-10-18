//? This file checks to see if the input provided by the repl is a valid operation
//? Operations must follow one of the following syntax : p1.operation_name(p1); 
//?     p1 and p2 must be previously defined




pub fn is_valid(input: &str) -> bool {
    //? check to see if line ends in semi-colon which is then removed so we deal only with NAME, =, VALUE
    if !input.ends_with(";") {
        println!("Missing semi-colon");
        return false;
    }

    let input: String = input.replace(";", "");

    //? This is only done to ensure that "=" exists as its not checked in `.split("=")`
    let values: Vec<&str> = input.split_whitespace().collect();

    true
}
