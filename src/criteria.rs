//! A collection of [`Criterion`](crate::criterion::Criterion)
//!
//! This is basically a fancy `Vec<Criterion>`. It implements `FromIterator` so you
//! can `collect()` an iterator of criterions into criteria.
//!
//! ## Example
//! ```rust
//! use lab_grader::*;
//!
//! let criteria = Criteria::from(vec! [
//!     Criterion::new("test 1", 15, ("p", "f"), Box::new(|_: &TestData| true)),
//!     Criterion::new("test 2", 10, ("p", "f"), Box::new(|_: &TestData| false)),
//! ]);
//!
//! assert!(criteria.len() == 2);
//! ```
//! **Or**
//! ```rust
//! // same as above..
//! # use lab_grader::*;
//! #
//! # let loose = vec! [
//! #     Criterion::new("test 1", 15, ("p", "f"), Box::new(|_: &TestData| true)),
//! #     Criterion::new("test 2", 10, ("p", "f"), Box::new(|_: &TestData| false)),
//! # ];
//!
//! let criteria: Criteria = loose.into_iter().collect();
//! assert!(criteria.len() == 2);
//! ```

use std::iter::FromIterator;

use crate::criterion::Criterion;

/// The Criteria struct, just a collection of [`Criterion`](crate::criterion::Criterion)
pub struct Criteria(Vec<Criterion>);


impl Criteria {
    // Creates a new empty criteria
    fn new() -> Criteria {
        Criteria(Vec::new())
    }

    /// Add a `Criterion` to the collection
    pub fn add(&mut self, criterion: Criterion) {
        self.0.push(criterion);
    }

    /// Creates a `Criteria` collection from a `Vec<Criterion>`
    ///
    /// ## Example
    /// ```rust
    /// use lab_grader::*;
    ///
    /// let critiera = Criteria::from(vec![
    ///     Criterion::new("name", 1, ("p", "f"), Box::new(|_: &TestData| true))
    /// ]);
    /// ```
    pub fn from(criteria: Vec<Criterion>) -> Self {
        criteria.into_iter().collect()
    }

    /// Returns the amount of `Criterion`s in this collection
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromIterator<Criterion> for Criteria {
    fn from_iter<I: IntoIterator<Item=Criterion>>(iter: I) -> Self {
        let mut criteria = Criteria::new();

        for i in iter {
            criteria.add(i);
        }

        criteria
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestData;

    #[test]
    fn test_build_criteria() {
        let loose = vec![
            Criterion::new("test 1", 1, ("p", "f"), Box::new(|_: &TestData| true)),
            Criterion::new("test 2", 1, ("p", "f"), Box::new(|_: &TestData| true)),
        ].into_iter();
        let criteria: Criteria = loose.collect();
        assert!(criteria.0.len() == 2);
    }

    #[test]
    fn test_build_from_vec() {
        let criteria = Criteria::from(vec![
            Criterion::new("test 1", 1, ("p", "f"), Box::new(|_: &TestData| true)),
            Criterion::new("test 2", 1, ("p", "f"), Box::new(|_: &TestData| true)),
        ]);
        assert!(criteria.0.len() == 2);
    }

    #[test]
    fn test_len() {
        let criteria = Criteria::from(vec![
            Criterion::new("test 1", 1, ("p", "f"), Box::new(|_: &TestData| true)),
            Criterion::new("test 2", 1, ("p", "f"), Box::new(|_: &TestData| true)),
        ]);
        assert!(criteria.len() == 2);
        assert!(criteria.0.len() == criteria.len());
    }

    #[test]
    fn test_add_criterion() {
        let mut criteria = Criteria::from(vec![
            Criterion::new("test 1", 1, ("p", "f"), Box::new(|_: &TestData| true)),
            Criterion::new("test 2", 1, ("p", "f"), Box::new(|_: &TestData| true)),
        ]);

        assert!(criteria.len() == 2);

        criteria.add(Criterion::new(
            "test 3", 1, ("p", "f"), Box::new(|_: &TestData| false)
        ));

        assert!(criteria.len() == 3);
    }
}
