//! A crate to assist with grading TCMG labs.
//!
//! This crate provides some tools to help with grading labs.
//!
//! Examples coming soon...
extern crate ansi_term;
extern crate serde;
extern crate serde_json;

#[cfg(test)]
#[macro_use] extern crate pretty_assertions;

pub mod results_file;
pub mod submission;
pub mod criterion;
// pub mod criteria;
