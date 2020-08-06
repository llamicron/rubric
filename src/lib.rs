//! A criterion runner
//!
//! For complete examples, see the [examples](https://github.com/llamicron/rubric/tree/master/examples)
//! directory on Github.


// External crates
#![feature(proc_macro_hygiene, decl_macro, stmt_expr_attributes)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate prettytable;
extern crate serde_yaml;
extern crate serde_json;
extern crate ansi_term;
extern crate failure;
extern crate reqwest;
extern crate chrono;
extern crate serde;
extern crate regex;



// Private modules
mod error;
mod yaml;




// Public modules
pub mod helpers;
pub mod dropbox;
pub mod rubric;

// Public Re-exports
pub use self::rubric::Rubric;
pub use self::dropbox::{open, Submission, TestData};
pub use error::{Result, Error};



// External testing crates
#[cfg(test)]
#[macro_use] extern crate pretty_assertions;
