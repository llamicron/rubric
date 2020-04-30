//! A submission is a bundle of data that represents
//! one student's submission. They will do some sort of work
//! for a lab, then run a rust script that builds some criteria,
//! runs those criteria with some data from the student, and submits
//! a Submission to a central webserver where the instructor can
//! collect the graded submissions.
use std::collections::HashMap;

use crate::data;

#[derive(Debug)]
pub struct Submission {
    /// The students name
    name: String,
    /// The students institutional ID
    id: u32,
    /// Numerical grade for the submission.
    /// Each criterion will add to this grade if it passes.
    grade: i16,
    /// A hashmap of extra data that may be sent by the submission.
    /// Leave it empty if you don't need it
    data: HashMap<String, String>
}

impl Submission {
    fn new<S: AsRef<str>>(name: S, id: u32) -> Submission {
        Submission {
            name: name.as_ref().to_string(),
            id,
            grade: 0,
            data: HashMap::new()
        }
    }

    pub fn use_data(&mut self, data: HashMap<String, String>) {
        self.data = data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_submission() {
        let sub = Submission::new("Luke", 1234);
        assert_eq!(sub.name, "Luke");
        assert_eq!(sub.id, 1234);
        assert!(sub.data.len() == 0);
    }

    #[test]
    fn test_submission_use_data() {
        let data = data! {
            "key" => "value"
        };
        let mut sub = Submission::new("Luke", 123);
        sub.use_data(data);
        assert!(sub.data.len() == 1);
        assert_eq!(sub.data["key"], "value");
    }
}
