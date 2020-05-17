//! A criterion runner
//!
//! For complete examples, see the [examples](https://github.com/llamicron/lab_grader/tree/master/examples)
//! directory on Github.

// External crates
#![feature(proc_macro_hygiene, decl_macro, stmt_expr_attributes)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate serde_yaml;
extern crate serde_json;
extern crate ansi_term;
extern crate failure;
extern crate reqwest;
extern crate chrono;
extern crate serde;
extern crate regex;

// Public modules
pub mod criterion_builder;
pub mod results_file;
pub mod submission;
pub mod criterion;
pub mod helpers;
pub mod batch;

// Private modules
mod server;
mod error;
mod yaml;


// Public uses
pub use helpers::{web, fs, cli};
pub use submission::Submission;
pub use submission::TestData;
pub use criterion::Criterion;
pub use results_file::AsCsv;
pub use batch::Batch;
pub use error::Error;


// private uses
use results_file::ResultsFile;


// Testing external crates
#[cfg(test)]
#[macro_use] extern crate pretty_assertions;
