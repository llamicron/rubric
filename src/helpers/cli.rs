//! Functions and macros that deal with the terminal

use regex::Regex;

use std::process::Command;
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
/// use std::net::Ipv4Addr;
///
/// fn main() {
///     let string = prompt!("Enter a string: ", String);
///     println!("{}", string);
///
///     let number = prompt!("Enter a number: ", u32);
///     println!("{}", number);
///
///     // This will exit with an error message if they
///     // don't enter a valid IP
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


pub enum Program {
    Git,
    Docker,
    Python,
    Ruby,
    DockerCompose
    // AzureCLI,
}


/// Returns the installed version of the program
///
/// This has not been extensively tested on windows
///
/// ```rust
/// use lab_grader::helpers::cli::{self, Program};
///
/// assert!(cli::version_of(Program::Git).is_some());
/// assert!(cli::version_of(Program::Ruby).is_some());
/// assert!(cli::version_of(Program::DockerCompose).is_some());
/// ```
pub fn version_of(program: Program) -> Option<String> {
    use Program::*;

    // Get command and regex pattern based on program
    let (cmd, pattern) = match program {
        Git => ("git --version", r"([\d\.]+)"),
        Docker => ("docker -v", r"([\d\.]+)"),
        DockerCompose => ("docker-compose -v", r"([\d\.]+)"),
        Python => ("python --version", r"([\d\.]+)"),
        Ruby => ("ruby -v", r"([\d\.]+)")
    };

    // Get the output of running the command
    let output = if cfg!(target_os = "windows") {
        // Run on windows
        Command::new("cmd").args(&["/C", cmd]).output()
    } else {
        // Run on anything else
        Command::new("sh").arg("-c").arg(cmd).output()
    };


    // If there's output
    if let Ok(resp) = output {
        // Build a regex pattern
        let re: Regex = pattern.parse().expect("Couldn't compile regex pattern");
        // If we can get valid text
        let text = match String::from_utf8(resp.stdout) {
            Ok(t)  => t,
            Err(_) => return None,
        };
        // If there's a version match
        if let Some(cap) = re.captures(&text) {
            if let Some(version) = cap.get(1) {
                // Return the version
                return Some(version.as_str().to_owned());
            }
        }

    }

    None
}
