//! Functions and macros that deal with the terminal

// std uses
use std::io::{stdin, stdout, Write};
use std::process::Command;


// Flushes stdout, this is only used internally
fn flush() {
    stdout().flush().expect("Failed to flush output");
}

/// Calls [`prompt`](./helpers/cli/fn.prompt.html), then tries to parse the input
/// into the provided type. If parsing fails, it will try again.
///
/// This method trims whitespace on the beginning and end of the input string.
///
/// ## Example
/// ```no_run
/// #[macro_use] extern crate rubric;
/// use rubric::helpers;
/// use std::net::Ipv4Addr;
///
/// fn main() {
///     let string = prompt!("Enter a string: ", String);
///     println!("{}", string);
///
///     let number = prompt!("Enter a number: ", u32);
///     println!("{}", number);
///
///     let another = prompt!("Enter an IP: ", Ipv4Addr);
///     println!("{}", another);
/// }
/// ```
/// They input:
/// ```text
/// Enter a string: Here's a string
/// Here's a string
/// Enter a number: 123
/// 123
/// Enter an IP: not an IP
/// Could not parse input. Try again.
/// Enter an IP: 192.168.0.1
/// 192.168.0.1
/// ```
#[macro_export]
macro_rules! prompt {
    ( $msg:expr, $type:ty ) => {
        loop {
            match rubric::helpers::cli::prompt($msg).parse::<$type>() {
                Ok(val) => break val,
                Err(_) => {
                    eprintln!("Could not parse input. Try again.");
                }
            };
        };
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
/// use rubric::helpers::cli::prompt;
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


/// Runs a command and returns a Result with the output. This is the Windows version.
/// 
/// This is equivilent to using [`Command`](std::process::Command), but it
/// handles platform differences for you. This is only meant for basic commands. For
/// anything more advanced than a simple command, use [`Command`](std::process::Command)
/// yourself.
/// 
/// ```rust
/// use rubric::helpers::cli;
/// 
/// let output = cli::cmd("dir");
/// assert!(output.is_ok());
/// assert!(output.unwrap().stdout.len() > 0);
/// ```
#[cfg(target_family = "windows")]
pub fn cmd(command: &str) -> std::result::Result<std::process::Output, std::io::Error> {
    Command::new("cmd")
        .args(&["/C", command])
        .output()
}


/// Runs a command and returns a Result with the output. This is the Unix version.
/// 
/// This is equivilent to using [`Command`](std::process::Command), but it
/// handles platform differences for you. This is only meant for basic commands. For
/// anything more advanced than a simple command, use [`Command`](std::process::Command)
/// yourself.
/// 
/// ```rust
/// use rubric::helpers::cli;
/// 
/// let output = cli::cmd("ls");
/// assert!(output.is_ok());
/// assert!(output.unwrap().stdout.len() > 0);
/// ```
#[cfg(target_family = "unix")]
pub fn cmd(command: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    #[cfg(target_family = "windows")]
    fn test_windows_command() {
        let result = cmd("dir");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.stdout.len() > 0);
        assert!(output.stderr.len() == 0);
        assert!(output.status.success());
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn test_unix_command() {
        let result = cmd("ls");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.stdout.len() > 0);
        assert!(output.stderr.len() == 0);
        assert!(output.status.success());
    }
}
