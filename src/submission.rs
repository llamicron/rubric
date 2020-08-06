//! A bundle of data that rubrics are graded against, and is submitted for review

// std uses
use std::collections::HashMap;

// external uses
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use reqwest::blocking::Response;

// internal uses
use crate::results_file::AsCsv;
use crate::rubric::Rubric;
use crate::helpers::web::post_json;

const DEFAULT_TIME_FORMAT: &'static str = "%F %a %T %:z";


/// A type alias to `HashMap<String, String>`
///
/// This is the data type that all criteria accept,
/// and how data is stored in a submission
pub type TestData = HashMap<String, String>;


/// A macro to easily create a [`TestData`](crate::submission::TestData)
/// struct, which is really just an alias to `HashMap<String, String>`.
///
/// ## Example
/// ```rust
/// # extern crate rubric;
/// use rubric::{TestData, data};
///
/// // The long way
/// let mut map = TestData::new();
/// map.insert(String::from("key"), String::from("value"));
///
/// // the macro way
/// let data = data! { "key" => "value" };
/// assert_eq!(map, data);
/// ```
#[macro_export]
macro_rules! data (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert(String::from($key), String::from($value));
            )+
            m
        }
     };
);


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
    /// Numerical grade for the submission.
    /// Each criterion will add to this grade if it passes.
    pub grade: i16,
    /// Extra data attached to the submission.
    /// Leave it empty if you don't need it
    pub data: TestData,
    /// The criteria (name) that this submission passed
    pub passed: Vec<String>,
    /// The citeria (name) that this submission failed
    pub failed: Vec<String>,
    /// How to format the timestamp.
    /// See https://docs.rs/chrono/0.4.13/chrono/format/strftime/index.html for more options
    #[serde(skip)]
    timestamp_format: String,
    /// If the submission is late or not
    late: bool,
}

impl Submission {
    /// Creates a new submission.
    ///
    /// ## Example
    /// ```rust
    /// use rubric::Submission;
    ///
    /// // You probably want it to be mutable so
    /// // you can attach data and change the grade
    /// let mut sub = Submission::new();
    ///
    /// assert_eq!(sub.grade, 0);
    /// assert_eq!(sub.data.len(), 0);
    /// ```
    pub fn new() -> Submission {
        Submission {
            time: Local::now(),
            grade: 0,
            data: TestData::new(),
            passed: Vec::new(),
            failed: Vec::new(),
            timestamp_format: String::from(DEFAULT_TIME_FORMAT),
            late: false
        }
    }

    /// Attaches data to a submission
    ///
    /// The data must be a [`TestData`](crate::submission::TestData).
    /// You may want to use the [`data!`](../macro.data.html) macro to make it
    /// easier to establish your data.
    ///
    /// You may be interested in [`Submission::from_data`](crate::submission::Submission::from_data).
    ///
    /// ## Example
    /// ```rust
    /// # use rubric::data;
    /// # use rubric::Submission;
    /// #
    /// let data = data! {
    ///     "key" => "value",
    ///     "key2" => "value2"
    /// };
    ///
    /// let mut sub = Submission::new();
    /// sub.use_data(data);
    ///
    /// assert_eq!(sub.data["key"], "value");
    /// assert_eq!(sub.data["key2"], "value2");
    /// ```
    pub fn use_data(&mut self, data: TestData) {
        self.data = data;
    }

    /// Creates a new submission and attaches data to it in one step
    ///
    /// ## Example
    /// ```rust
    /// # use rubric::{Submission, data};
    ///
    /// let sub = Submission::from_data(data! {
    ///     "name" => "luke i guess",
    ///     "id" => "1234"
    /// });
    ///
    /// assert_eq!(sub.data["id"], "1234");
    /// ```
    pub fn from_data(data: TestData) -> Self {
        let mut sub = Submission::new();
        sub.use_data(data);
        sub
    }

    /// Marks a criterion as passed. Provide the name of the criterion.
    ///
    /// This struct does not include an actual [`Criterion`](crate::criterion::Criterion)
    /// struct in it's `passed` and `failed` fields, because it's impossible to
    /// serialize a `Criterion`. `Submission`s must be serializable. Instead, only the
    /// name and message of the criterion are stored on the submission
    ///
    /// ## Example
    /// ```rust
    /// # use rubric::Submission;
    /// let mut sub = Submission::new();
    /// sub.pass("Some criterion name");
    ///
    /// assert!(sub.passed.contains(&"Some criterion name".to_string()));
    /// ```
    pub fn pass<C: AsRef<str>>(&mut self, criterion: C) {
        self.passed.push(criterion.as_ref().to_string());
    }

    /// Same as [`pass`](crate::submission::Submission::pass), but adds to the `failed` vector
    pub fn fail<C: AsRef<str>>(&mut self, criterion: C) {
        self.failed.push(criterion.as_ref().to_string());
    }

    /// Tests a submission against a list of criterion
    pub fn grade_against(&mut self, rubric: &mut Rubric) {
        if rubric.past_due() {
            // Submission is late, mark it as such
            self.late = true;
            // And subtract the late penalty
            self.grade -= rubric.late_penalty as i16;
            self.fail(format!("Late submission (-{})", rubric.late_penalty));
            // If they disallow late submission
            if !rubric.allow_late {
                // Inform the student and return early without grading
                eprintln!("Deadline ({}) has passed.", rubric.deadline.unwrap());
                eprintln!("Your instructor has chosen to not allow late submission");
                eprintln!("This submission will be recorded, but with a grade of 0");
                self.grade = 0;
                return;
            }
        }

        for crit in &mut rubric.sorted().into_iter() {
            crit.test_with_data(&self.data);

            if crit.status.unwrap() {
                self.grade += crit.worth;
                self.pass(format!("{}: {}", crit.name, crit.success_message()));
            } else {
                self.fail(format!("{}: {}", crit.name, crit.failure_message()));
            }
        }
    }

    /// Posts the submission to the URL in JSON format. Meant to be sent
    /// to a dropbox. Really just calls [`helpers::web::post_json`](rubric::helpers::web::post_json).
    pub fn submit(&self, url: &str) -> Result<Response, reqwest::Error> {
        post_json(url, self)
    }

    /// Overrides the default timestamp format.
    /// The default is `%F %a %T %:z` which gives
    /// ```text
    /// 2001-08-04 Thu 00:34:60 +09:30
    /// ```
    /// This timestamp format is human readable and also sortable in a spreadsheet.
    pub fn set_timestamp_format(&mut self, new_format: &str) {
        self.timestamp_format = String::from(new_format);
    }
}

impl AsCsv for TestData {
    /// Returns the test data, serialized to a csv string. It will be
    /// sorted alphabetically by key.
    fn as_csv(&self) -> String {
        let mut v: Vec<_> = self.into_iter().collect();
        v.sort_by(|x,y| x.0.cmp(&y.0));
        v.iter().map(|v| v.1.replace(",", ";") ).collect::<Vec<_>>().join(",")
    }

    /// Returns the filename that the [`ResultsFile`](crate::results_file::ResultsFile)
    /// uses as its output
    ///
    /// This probably shouldn't get used for test data, as it will be written as part
    /// of a submission, not on it's own.
    fn filename(&self) -> String {
        String::from("submission_data.csv")
    }

    /// Returns a header to write to a csv file. This should match the fields in `as_csv` above.
    fn header(&self) -> String {
        // let keys: Vec<&String> = self.keys().collect();
        // let mut owned_keys: Vec<String> = keys.iter().map(|&k| k.to_owned() ).collect();
        // owned_keys.sort_by(|a,b| a.cmp(&b) );
        // return format!("{}", owned_keys.join(","));
        let mut v: Vec<_> = self.into_iter().collect();
        v.sort_by(|x,y| x.0.cmp(&y.0));
        v.iter().map(|v| v.0.to_owned() ).collect::<Vec<_>>().join(",")
    }
}

impl AsCsv for Submission {
    /// Returns the submission's values in csv format. The `TestData` atttached will be
    /// sorted alphabetically by key.
    fn as_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{}",
            self.time.format(&self.timestamp_format),
            self.late,
            self.grade,
            self.passed.join(";"),
            self.failed.join(";"),
            self.data.as_csv()
        )
    }

    /// Returns the filename to use when writing submissions to disk
    fn filename(&self) -> String {
        String::from("submissions.csv")
    }

    /// Returns a header of all the fields, matching the data in `as_csv`
    fn header(&self) -> String {
        format!("time,late,grade,passed,failed,{}", self.data.header())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{data, yaml, attach};


    #[test]
    fn test_new_submission() {
        let sub = Submission::new();
        assert!(sub.data.len() == 0);
    }

    #[test]
    fn test_submission_use_data() {
        let data = data! {
            "key" => "value"
        };
        let mut sub = Submission::new();
        sub.use_data(data);
        assert!(sub.data.len() == 1);
        assert_eq!(sub.data["key"], "value");

        let sub2 = Submission::from_data(data! {
            "key" => "value"
        });
        assert_eq!(sub2.data["key"], "value");
    }

    #[test]
    fn test_submission_as_csv() {
        let sub = Submission::from_data(data! { "a" => "v", "b" => "v2" });

        // TestData keys are sorted alphabetically when converting to csv
        assert!((&sub).as_csv().contains("v,v2"));

        // Submission with no data, passes, or failures
        let sub2 = Submission::new();
        let expected = "0,,,";
        assert!((&sub2).as_csv().contains(expected));
    }

    #[test]
    fn test_serialize_deserialize_json() {
        let mut sub = Submission::from_data(data! { "k2" => "v2", "k" => "v" });
        sub.pass("something");
        sub.fail("something");

        assert!(serde_json::to_string(&sub).unwrap().contains(r#""k2":"v2""#));

        let data = r#"{"time":"2020-05-01T22:23:21.180875-05:00","late":false,"grade":0,"passed":["something"],"failed":["something"],"data":{"k2":"v2","k":"v"}}"#;
        let built_sub: Submission = serde_json::from_str(data).unwrap();
        assert_eq!(built_sub.grade, sub.grade);
    }

    #[test]
    fn test_grade_against_rubric() {
        let yaml = yaml!("../test_data/test_rubric.yml").unwrap();
        let mut rubric = Rubric::from_yaml(yaml).unwrap();
        let test = |_: &TestData| true;
        attach! {
            rubric,
            "first-crit" => test
        };

        let mut sub = Submission::new();

        sub.grade_against(&mut rubric);
        assert_eq!(sub.grade, 50);
    }

    #[test]
    fn test_test_data_as_csv() {
        let d = data! {
            "b2" => "value2",
            "a1" => "value1"
        };

        let expected_header = "a1,b2";
        let expected_values = "value1,value2";
        let expected_filename = "submission_data.csv";

        assert_eq!(d.header(), expected_header);
        assert_eq!(d.as_csv(), expected_values);
        assert_eq!(d.filename(), expected_filename);
    }

    #[test]
    fn test_as_csv_replaces_commas() {
        let sub = Submission::from_data(data! {
            "key" => "value with, comma"
        });

        assert!(sub.as_csv().contains("value with; comma"));
    }

    #[test]
    fn test_test_data_gets_sorted() {
        let data = data! { 
            "a" => "something",
            "b" => "else"
        };

        let csv = data.as_csv();
        assert!(csv.contains("something,else"));
    }

    #[test]
    fn test_custom_timestamp_format() {
        let mut sub = Submission::new();
        assert_eq!(sub.timestamp_format, DEFAULT_TIME_FORMAT);
        sub.set_timestamp_format("some other format");
        assert_eq!(sub.timestamp_format, "some other format");
    }

    #[test]
    fn test_grading_past_due() {
        let yaml = yaml!("../test_data/past_due_rubric.yml").unwrap();
        let mut past_due_rubric = Rubric::from_yaml(yaml).unwrap();

        let mut sub = Submission::new();

        // This rubric will allow late submission, with a 5 point penalty
        assert_eq!(sub.grade, 0);
        sub.grade_against(&mut past_due_rubric);
        assert_eq!(sub.grade, -5);
    }
}
