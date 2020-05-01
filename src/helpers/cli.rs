//! Functions that deal with the terminal
use std::io::{stdout, stdin, Write};


fn flush() {
    stdout().flush().expect("Failed to flush output");
}

/// Calls [`prompt`](./helpers/cli/fn.prompt.html), then tries to parse the input
/// into the provided type. If parsing fails, it will print an error message
/// **and then quit the current process**.
///
/// This method trims whitespace on the beginning and end of the input string.
///
/// If you wish to deal with the error yourself instead of quitting,
/// then call `prompt`, then `parse` yourself.
///
/// ## Example
/// ```no_run
/// #[macro_use] extern crate lab_grader;
/// use lab_grader::helpers;
///
/// fn main() {
///     let string = prompt!("Enter a string: ", String);
///     println!("{}", string);
///
///     let number = prompt!("Enter a number: ", u32);
///     println!("{}", number);
///
///     let another = prompt!("Enter another number: ", u32);
///     println!("{}", another);
/// }
/// ```
/// They input:
/// ```text
/// Enter a string: Here's a string
/// Here's a string
/// Enter a number: 123
/// 123
/// Enter another number: not a number
/// Could not parse input
/// ```
#[macro_export]
macro_rules! prompt {
    ( $msg:expr, $type:ty ) => {
        match crate::helpers::cli::prompt($msg).parse::<$type>() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Could not parse input");
                std::process::exit(1);
            }
        }
    };
}

/// Prompts a user for input from the CLI.
///
/// Returns the string they entered, with leading and trailing whitespace trimmed.
/// This method will loop infinitely until a valid string is read.
///
/// If you're going to cast the result to a certain type, try the
/// [`prompt!`](../../macro.prompt.html) macro.
///
/// ## Example
/// ```no_run
/// use lab_grader::helpers::cli::prompt;
///
/// let input = prompt("Enter hello: ");
/// println!("{}", input);
/// ```
/// They see:
/// ```text
/// Enter hello: hello
/// hello
/// ```
pub fn prompt(msg: &str) -> String {
    let mut input = String::new();
    loop {
        print!("{}", msg);
        flush();
        if let Err(e) = stdin().read_line(&mut input) {
            println!("Error: {}", e);
            println!("Try again.");
            flush();
        } else {
            return input.trim().to_string();
        }
    }
}

