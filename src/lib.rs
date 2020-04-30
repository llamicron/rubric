//! A crate to assist with grading TCMG labs.
//!
//! This crate provides some tools to help with grading labs.
//!
//! Examples coming soon...
#[cfg(test)]
#[macro_use] extern crate pretty_assertions;


pub mod results_file;
pub mod criterion;


mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
