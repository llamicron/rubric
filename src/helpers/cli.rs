//! Functions and macros that deal with the terminal

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

/// Returns true if the provided program is installed
///
/// Various programs return different exit codes when run.
/// It's kind of a crap shoot knowing what's going on. For instance,
/// running `git` prints the git help page and exits with a 1 (err) exit code
/// if it is installed, and the same error code if it isn't installed.
///
/// On the other hand, `docker` prints the docker help page and responds with
/// a 0 (ok) exit code. Intrestingly enough, `docker-compose` behaves the opposite way.
/// For this reason, I've hard coded some command programs to deal with this problem.
///
/// See the [`Program`](crate::helpers::cli::Program) enum.
///
/// ```rust
/// use lab_grader::helpers::cli::{installed, Program};
///
/// assert!(installed(Program::Git));
/// ```
pub fn installed(program: Program) -> bool {
    use Program::*;

    let name = match program {
        Git => "git --version",
        Docker => "docker",
        DockerCompose => "docker-compose --version",
        Python => "python --version",
        Ruby => "ruby -v"
    };

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", name])
                .output()
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(name)
                .output()
    };

    if let Ok(resp) = output {
        return resp.status.success();
    }
    false
}
