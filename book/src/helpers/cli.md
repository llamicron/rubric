# Command Line/Programs - `helpers::cli`
Functions and macros for command line interaction

## `prompt!` (macro)
This macro will prompt the user for input and try to cast it to the given type.

Whitespace around string ends will be trimmed.

> Warning! if the user enters something that can't be parsed into the type you specify, this will print an error message and *exit the current process*

```rust ,noplaypen
use std::net::Ipv4Addr;

let string: String = prompt!("Enter a string: ", String);
let number: i64 = prompt!("Enter a number: ", i64);
let ip: Ipv4Addr = prompt!("Enter an IPv4 address: ", Ipv4Addr);
```

## `prompt` (function)
This is similar to the macro `prompt!`, but if the user enter invalid input it will print an error message and run again. This ensures you always get *something* from the user. However, it won't automatically cast to a type. It just returns a `String`, which you should cast manually.

Whitespace around the ends will be trimmed.

```rust ,noplaypen
let input: String = prompt("Enter something: ");
```

## `Program`
Currently, this enum allows you to get the installed version of a program. The current programs supported are:
* Git
* Docker
* Docker-Compose
* Python
* Ruby

```rust ,noplaypen
let version: Version = cli::Program::Git.version();

// If the program isn't installed, this will be None
assert!( version.is_some() );

// You can get the major, minor, and patch numbers of the version
assert!( version.unwrap().major() >= 1 );
```
