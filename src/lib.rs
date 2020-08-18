//! A criterion runner
//!
//! For complete examples, see the [examples](https://github.com/llamicron/rubric/tree/master/examples)
//! directory on Github.


// External crates
#![feature(proc_macro_hygiene, decl_macro, stmt_expr_attributes)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate serde_yaml;
extern crate serde_json;
extern crate reqwest;
extern crate chrono;
extern crate anyhow;
extern crate serde;
extern crate regex;
extern crate paris;

// External testing crates
#[cfg(test)]
#[macro_use] extern crate pretty_assertions;


// Private modules
mod yaml;

// Public modules
pub mod helpers;
pub mod dropbox;
pub mod rubric;
pub mod report;
mod macros;


// Public Re-exports
// These are commonly imported, so they're at the top
// also I don't like rubric::rubric::Rubric
pub use self::rubric::Rubric;
pub use self::dropbox::{open, Submission, TestData};

pub type Result<T> = anyhow::Result<T>;
pub type Error = anyhow::Error;


// This is the full timestamp format with date, time, and timezone
// Example: 2001-07-08 Sun 00:34:59 +09:30
const TIMESTAMP_FORMAT: &'static str = "%F %a %T %:z";
// This is a more human friendly timestamp
// Example: Sunday 01:34AM 8-Jul-2001
const HR_TIMESTAMP_FORMAT: &'static str = "%A %R%p %v";
