//! A crate to assist with grading TCMG labs.
//!
//! This crate provides some tools to help with grading labs.
//!
//! Examples coming soon...
extern crate ansi_term;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

#[cfg(test)]
#[macro_use] extern crate pretty_assertions;

pub mod results_file;
pub mod submission;
pub mod criterion;
pub mod helpers;


pub use submission::Submission;
pub use results_file::{ResultsFile, AsCsv};
pub use helpers::{cli, web};
pub use criterion::Criterion;
