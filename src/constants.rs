pub const AND: (&str, &str) = ("^", "AND");
pub const OR: (&str, &str) = ("v", "OR");
pub const QUIT_OPTIONS: [&str; 3] = ["quit", "exit", "q"];
pub const OUTPUT_FILE: &str = "file.txt";
pub const HELP_MESSAGE: &str = "
    Welcome to TTG (Truth Table Generator) v0.0.1!

    This tool allows you to create logical statements, perform operations on them, and generate truth tables interactively.

    Commands:
    - help : Show this help message.
    - quit | q | exit : Exit the program.

    Creating a Statement:
    - Syntax: NAME = [value1, value2, ...];
    - Example: p1 = [t, t, f, f];

        Rules:
        1. Values must be 't' (true) or 'f' (false).
        - Case insensitive (e.g., 'T', 'TRUE', 't', 'true' are all valid for true).
        2. Use '=' to assign values to a statement.
        3. NAME must be alphanumeric and can include underscores or hyphens.
        - Length: 1 to 10 characters.
        4. Each statement must end with a semicolon.
        5. Ensure there is a space between the NAME, '=', and VALUES.

    Operations:
    - and: Logical AND operation.
    - or: Logical OR operation.
    - not: Logical NOT operation.
    - xor: Logical XOR operation.
    - xnor: Logical XNOR operation.
    - if_then: Logical implication (if-then).
    - if: Logical reverse implication.
    - if_and_only_if: Logical biconditional (if and only if).

    Example Usage:
    - ttg> p1 = [t, t, f, f];
    - ttg> p2 = [t, f, t, f];
    - ttg> p1;
    - ttg> [t, t, f, f];
    - ttg> result = p1 and p2;
    - ttg> result;
    - ttg> [t, f, f, f]

    For more detailed instructions, refer to the documentation or use the 'man' command.
";
