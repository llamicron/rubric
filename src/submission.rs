//! A bundle of data that represents a students work.
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::results_file::AsCsv;

/// A submission is a bundle of data that represents
/// one student's submission. They will do some sort of work
/// for a lab, then run a rust script that builds some criteria,
/// runs those criteria with some data from the student, and submits
/// a Submission to a central webserver where the instructor can
/// collect the graded submissions.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Submission {
    /// The students name
    pub name: String,
    /// The students institutional ID
    pub id: u32,
    /// Numerical grade for the submission.
    /// Each criterion will add to this grade if it passes.
    pub grade: i16,
    /// A hashmap of extra data that may be sent by the submission.
    /// Leave it empty if you don't need it
    pub data: HashMap<String, String>
}

impl Submission {
    /// Creates a new submission with a name and id.
    ///
    /// The `data` field is set to an empty HashMap, and `grade` is set to 0.
    ///
    /// *Hint*: If you want to start with a grade and bring the grade
    /// down for every criterion not passed, set the grade manually here and
    /// set the point value for each criterion to be a negative number.
    ///
    /// ## Example
    /// ```rust
    /// use lab_grader::submission::Submission;
    ///
    /// // You probably want it to be mutable so
    /// // you can attach data and change the grade
    /// let mut sub = Submission::new("Luke", 1234);
    ///
    /// assert_eq!(sub.name, "Luke");
    /// assert_eq!(sub.id, 1234);
    /// assert_eq!(sub.grade, 0);
    /// assert_eq!(sub.data.len(), 0);
    /// ```
    pub fn new<S: AsRef<str>>(name: S, id: u32) -> Submission {
        Submission {
            name: name.as_ref().to_string(),
            id,
            grade: 0,
            data: HashMap::new()
        }
    }

    /// Attaches data to a submission
    ///
    /// The data must be a [`HashMap<String, String>`](std::collections::HashMap).
    /// You may want to use the [`data!`](../macro.data.html) macro to make it
    /// easier to establish your data.
    ///
    /// ## Example
    /// ```rust
    /// # use lab_grader::data;
    /// # use lab_grader::submission::Submission;
    /// #
    /// let data = data! {
    ///     "key" => "value",
    ///     "key2" => "value2"
    /// };
    ///
    /// let mut sub = Submission::new("Luke", 1234);
    /// sub.use_data(data);
    ///
    /// assert_eq!(sub.data["key"], "value");
    /// assert_eq!(sub.data["key2"], "value2");
    /// ```
    pub fn use_data(&mut self, data: HashMap<String, String>) {
        self.data = data
    }
}

impl AsCsv for Submission {
    fn as_csv(&self) -> String {
        let data_string = self.data.keys().map(|k| {
            format!("{}=>{}", k, self.data[k])
        }).collect::<Vec<String>>().join(";");

        format!(
            "{},{},{},{}",
            self.name,
            self.id,
            self.grade,
            data_string
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;


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

    #[test]
    fn test_submission_as_csv() {
        let mut sub = Submission::new("Luke", 1234);
        sub.use_data(data! { "k" => "v", "k2" => "v2" });

        // We can't directly compare it because the order of the
        // hashmap items will change arbitrarily
        assert!(sub.as_csv().contains("Luke,1234,0,"));

        // Submission with no data
        let sub2 = Submission::new("Luke", 1234);
        let expected = String::from("Luke,1234,0,");
        assert_eq!(sub2.as_csv(), expected);
    }

    #[test]
    fn test_serialize_deserialize_json() {
        let mut sub = Submission::new("Luke", 1234);
        sub.use_data(data! { "k2" => "v2", "k" => "v" });

        let expected = r#"{"name":"Luke","id":1234,"grade":0,"data":{"k2":"v2","k":"v"}}"#;
        assert!(serde_json::to_string(&sub).unwrap().contains(r#""name":"Luke""#));
        let built_sub: Submission = serde_json::from_str(expected).unwrap();
        assert_eq!(built_sub, sub);
    }
}
