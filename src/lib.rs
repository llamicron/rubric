//! A criterion runner
//!
//! For complete examples, see the [examples](https://github.com/llamicron/lab_grader/tree/master/examples)
//! directory on Github.
//!
//! ## Example
//! ```no_run
//! extern crate lab_grader;
//!
//! use std::collections::HashMap;
//! use lab_grader::*;
//!
//! fn main() {
//!     // Step 1: Build a Submission
//!     // Collect name and ID from the command line
//!     let mut sub = Submission::from_cli();
//!     // Give the submission some data using the data! macro
//!     sub.use_data(data! {
//!         "some_key" => "some value"
//!     });
//!
//!     // Step 2: Establish Criteria
//!     let mut criteria = vec![
//!         Criterion::new(
//!             // The criterion's name
//!             "First criterion",
//!             // How many points it's worth
//!             10,
//!             // pass/fail messages
//!             ("passed", "failed"),
//!             // The test that determines if the criterion passes or not
//!             Box::new(|data: &HashMap<String, String>| -> bool {
//!                 data["some_key"] == "some value"
//!             })
//!         )
//!     ];
//!
//!     // Grade the submission against the criteria.
//!     // This will assign it a grade and fill it's `passed` and `failed` fields
//!     sub.grade_against(&mut criteria);
//!
//!     // Print out all the criteria to the student
//!     for crit in criteria {
//!         println!("{}", crit);
//!     }
//! }
//! ```

// External crates
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate ansi_term;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

// Testing external crates
#[cfg(test)]
#[macro_use] extern crate pretty_assertions;

// Public modules
pub mod results_file;
pub mod submission;
pub mod criterion;
pub mod helpers;

// Private modules
mod server;



// Public uses
pub use submission::Submission;
pub use criterion::Criterion;
pub use results_file::AsCsv;
pub use helpers::web;

// private uses
use results_file::ResultsFile;
