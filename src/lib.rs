//! A criterion runner
//!
//! For complete examples, see the [examples](https://github.com/llamicron/lab_grader/tree/master/examples)
//! directory on Github.
//!
//! ## Example
//! ```no_run
//! extern crate lab_grader;
//!
//! use lab_grader::*;
//!
//! fn main() {
//!     // Step 1: Build a Submission
//!     // Give the submission some data using the data! macro
//!     // You can also prompt the user for some data, with type enforcement
//!     let mut sub = Submission::from_data(data! {
//!         "some_key" => "some value",
//!         "other_data" => prompt!("Enter a number: ", String)
//!     });
//!
//!     // Step 2: Establish Criteria
//!     let mut criteria = Criteria::from(vec![
//!         Criterion::new(
//!             // The criterion's name
//!             "First criterion",
//!             // How many points it's worth
//!             10,
//!             // pass/fail messages
//!             ("passed", "failed"),
//!             // The test that determines if the criterion passes or not
//!             Box::new(|data: &TestData| -> bool {
//!                 data["some_key"] == "some value"
//!             })
//!         )
//!     ]);
//!
//!     // Grade the submission against the criteria.
//!     // This will assign it a grade and fill it's `passed` and `failed` fields
//!     sub.grade_against(&mut criteria);
//!
//!     // Print out all the criteria to the student
//!     println!("{}", criteria);
//!
//!     // Post the submission somewhere
//!     // (this crates provides a server that accepts them)
//!     // (i'm not gonna actually do it because i test against these docs)
//!     // web::post_json("http://url.somewhere/submit", &sub);
//!
//! }
//! ```

// External crates
#![feature(proc_macro_hygiene, decl_macro, stmt_expr_attributes)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate serde_yaml;
extern crate serde_json;
extern crate ansi_term;
extern crate reqwest;
extern crate chrono;
extern crate serde;
extern crate regex;

// Testing external crates
#[cfg(test)]
#[macro_use] extern crate pretty_assertions;

// Public modules
pub mod results_file;
pub mod submission;
pub mod criterion;
pub mod criteria;
pub mod helpers;
pub mod batch;

// Private modules
mod server;
mod yaml;



// Public uses
pub use helpers::{web, fs, cli};
pub use submission::Submission;
pub use submission::TestData;
pub use criterion::Criterion;
pub use results_file::AsCsv;
pub use criteria::Criteria;
pub use batch::Batch;

// private uses
use results_file::ResultsFile;
