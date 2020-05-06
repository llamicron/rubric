//! A bundle of data that represents a students work.

// std uses
use std::collections::HashMap;

// external uses
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

// internal uses
use crate::prompt;
use crate::results_file::AsCsv;
use crate::criteria::Criteria;
use crate::server;



/// A type alias to `HashMap<String, String>`
///
/// This is the data type that all criteria accept,
/// and how data is stored in a submission
pub type TestData = HashMap<String, String>;

/// A submission is a bundle of data that represents
/// one student's submission. They will do some sort of work
/// for a lab, then run a rust script that builds some criteria,
/// runs those criteria with some data from the student, and submits
/// a Submission to a central webserver where the instructor can
/// collect the graded submissions.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Submission {
    /// A local timestamp when the submission was created
    pub time: DateTime<Local>,
    /// The students name
    pub name: String,
    /// The students institutional ID
    pub id: u32,
    /// Numerical grade for the submission.
    /// Each criterion will add to this grade if it passes.
    pub grade: i16,
    /// Extra data attached to the submission.
    /// Leave it empty if you don't need it
    pub data: TestData,
    /// The criteria (name) that this submission passed
    pub passed: Vec<String>,
    /// The citeria (name) that this submission failed
    pub failed: Vec<String>
}

impl Submission {
    /// Creates a new submission with a name and id.
    ///
    /// The `data` field is set to an empty TestData, and `grade` is set to 0.
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
            time: Local::now(),
            name: name.as_ref().to_string(),
            id,
            grade: 0,
            data: TestData::new(),
            passed: Vec::new(),
            failed: Vec::new()
        }
    }


    /// Prompts the user for a name and ID number
    /// and returns a Submission. Equivalent to getting
    /// a name and ID from the console and calling
    /// `Submission::new()` with those values
    ///
    /// Warning: If the user doesn't enter valid values for name and id,
    /// this will **terminate the program**. Be sure that's what you want to do
    /// before using it.
    ///
    /// ## Example
    /// **Rust:**
    /// ```no_run
    /// # use lab_grader::submission::Submission;
    /// let mut sub = Submission::from_cli();
    /// ```
    /// **In the terminal:**
    /// ```text
    /// Name: Luke
    /// ID: 123
    /// ```
    /// **With invalid input:**
    /// ```text
    /// Name: Luke
    /// ID: not a number
    /// Could not parse input
    /// ```
    pub fn from_cli() -> Submission {
        let name = prompt!("Name: ", String);
        let id = prompt!("ID: ", u32);
        Submission::new(name, id)
    }

    /// Attaches data to a submission
    ///
    /// The data must be a [`TestData`](crate::submission::TestData).
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
    pub fn use_data(&mut self, data: TestData) {
        self.data = data;
    }

    /// Marks a criterion as passed. Provide the name of the criterion.
    ///
    /// This struct does not include an actual [`Criterion`](crate::criterion::Criterion)
    /// struct in it's `passed` and `failed` fields, because it's impossible to
    /// serialize a `Criterion`. `Submission`s must be serializable.
    ///
    /// ## Example
    /// ```rust
    /// # use lab_grader::submission::Submission;
    /// let mut sub = Submission::new("Luke", 1234);
    /// sub.pass("Some criterion name");
    ///
    /// assert!(sub.passed.contains(&"Some criterion name".to_string()));
    /// ```
    pub fn pass<C: AsRef<str>>(&mut self, criterion: C) {
        self.passed.push(criterion.as_ref().to_string());
    }

    /// Same as `pass`, but adds to the `failed` vector
    pub fn fail<C: AsRef<str>>(&mut self, criterion: C) {
        self.failed.push(criterion.as_ref().to_string());
    }

    /// Tests a submission against a list of criterion
    ///
    /// The submissions grade will change for every passed criterion,
    /// and every criterion will add it's name and message to the submissions
    /// `passed` or `failed` vectors.
    ///
    /// ## Example
    /// ```rust
    /// # use lab_grader::data;
    /// # use lab_grader::criterion::Criterion;
    /// # use lab_grader::criteria::Criteria;
    /// # use lab_grader::submission::Submission;
    /// # use lab_grader::TestData;
    /// #
    /// let mut sub = Submission::new("Luke", 1234);
    /// sub.use_data(data! {
    ///     "key" => "value"
    /// });
    /// // Just one criterion here to save space
    /// let mut crits = Criteria::from(vec![
    ///     Criterion::new("Test Criterion", 10, ("passed", "failed"), Box::new(
    ///         |data: &TestData| -> bool {
    ///             data["key"] == "value"
    ///         }
    ///     ))
    /// ]);
    /// sub.grade_against(&mut crits);
    /// assert_eq!(sub.grade, 10);
    /// assert_eq!(sub.passed.len(), 1);
    /// assert_eq!(sub.failed.len(), 0);
    /// ```
    pub fn grade_against(&mut self, criteria: &mut Criteria) {
        for crit in &mut criteria.0 {
            crit.test_with_data(&self.data);

            if crit.status.unwrap() {
                self.grade += crit.worth;
                self.pass(format!("{}: {}", crit.name, crit.success_message()));
            } else {
                self.fail(format!("{}: {}", crit.name, crit.failure_message()));
            }
        }
    }


    /// Spins up a webserver to accept submission.
    ///
    /// Accepted submissions will be written to a [`ResultsFile`](crate::results_file::ResultsFile).
    /// The web server will run on the provided port.
    ///
    /// The results file will be placed in the directory you execute the code in,
    /// and be called `results.csv`.
    ///
    /// Support for custom results file locations is coming...
    /// ```no_run
    /// use lab_grader::Submission;
    /// Submission::server(8080);
    /// ```
    pub fn server(port: u16) {
        server::run(port);
    }
}

impl AsCsv for TestData {
    fn as_csv(&self) -> String {
        let values: Vec<&String> = self.values().collect();
        let mut owned_values: Vec<String> = values.iter().map(|&k| k.to_owned() ).collect();
        owned_values.sort_by(|a,b| a.cmp(&b) );
        return owned_values.join(",");
    }

    fn filename(&self) -> String {
        String::from("submission_data.csv")
    }

    fn header(&self) -> String {
        let keys: Vec<&String> = self.keys().collect();
        let mut owned_keys: Vec<String> = keys.iter().map(|&k| k.to_owned() ).collect();
        owned_keys.sort_by(|a,b| a.cmp(&b) );
        return format!("{}", owned_keys.join(","));
    }
}

impl AsCsv for Submission {
    fn as_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{},{}",
            self.time.to_rfc3339(),
            self.name,
            self.id,
            self.grade,
            self.passed.join(";"),
            self.failed.join(";"),
            self.data.as_csv()
        )
    }

    fn filename(&self) -> String {
        String::from("submissions.csv")
    }

    fn header(&self) -> String {
        format!("time,name,id,grade,passed,failed,{}", self.data.header())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;
    use crate::Criterion;


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
        // TestData items will change arbitrarily
        assert!((&sub).as_csv().contains("Luke,1234,0,"));

        // Submission with no data, passes, or failures
        let sub2 = Submission::new("Luke", 1234);
        let expected = "Luke,1234,0,,,";
        assert!((&sub2).as_csv().contains(expected));
    }

    #[test]
    fn test_serialize_deserialize_json() {
        let mut sub = Submission::new("Luke", 1234);
        sub.use_data(data! { "k2" => "v2", "k" => "v" });
        sub.pass("something");
        sub.fail("something");

        let expected = r#"{"time":"2020-05-01T22:23:21.180875-05:00","name":"Luke","id":1234,"grade":0,"passed":["something"],"failed":["something"],"data":{"k2":"v2","k":"v"}}"#;
        assert!(serde_json::to_string(&sub).unwrap().contains(r#""name":"Luke""#));
        let built_sub: Submission = serde_json::from_str(expected).unwrap();
        assert_eq!(built_sub.name, sub.name);
        assert_eq!(built_sub.id, sub.id);
        assert_eq!(built_sub.grade, sub.grade);
    }

    #[test]
    fn test_grade_against_criteria() {
        let mut sub = Submission::new("Luke", 1234);
        sub.use_data(data! {
            "key" => "value"
        });

        // Just one criterion here to save space
        let mut crits = Criteria::from(vec![
            Criterion::new("Test Criterion", 10, ("passed", "failed"),
                Box::new(|data: &TestData| {
                        data["key"] == "value"
                    }
                )
            )
        ]);

        sub.grade_against(&mut crits);
        assert_eq!(sub.grade, 10);
        assert_eq!(sub.passed.len(), 1);
        assert_eq!(sub.failed.len(), 0);
    }

    #[test]
    fn test_test_data_as_csv() {
        let d = data! {
            "key1" => "value1",
            "key2" => "value2"
        };

        let expected_header = "key1,key2";
        let expected_values = "value1,value2";
        let expected_filename = "submission_data.csv";

        assert_eq!(d.header(), expected_header);
        assert_eq!(d.as_csv(), expected_values);
        assert_eq!(d.filename(), expected_filename);

        let to_sort = data! {
            "bbb" => "value",
            "aaa" => "other value"
        };
        assert_eq!(to_sort.header(), "aaa,bbb");
    }
}
