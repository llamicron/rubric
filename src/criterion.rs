//! Definitions for creating and running criteria
//!
//! A criterion is one specific item in a series of items that form a grade.
//! Each criterion has a name, point value, and a related function.
//! Testing the criterion is running the function, which will return true
//! or false. A final grade can be calculated by adding up all the values
//! of the criteria, if they passed.
//!
//! You **probably shouldn't** create criteria individually through this module,
//! but you can if you want. Instead, you should define your criteria in `YAML` then
//! build that into a [`Batch`](crate::batch::Batch).

// std uses
use std::fmt;
use std::fmt::Write;

// external uses
use ansi_term::Color::{Green, Red, White};

// internal uses
use crate::TestData;
use crate::criterion_builder::CriterionBuilder;


/// A single Criterion
pub struct Criterion {
    /// An ID stub used to identify this criterion
    pub stub: String,
    /// A short (< 30 characters), descriptive name
    pub name: String,
    /// Point value of this criterion. If it passes, this value
    /// will be added to the [`Submission`](crate::submission::Submission) grade.
    ///
    /// Can be negative if you wish to subtract points. Be sure to get your logic right.
    /// This value is added to the submission grade *if the test returns true*.
    pub worth: i16,
    /// An index to sort by when running.
    ///
    /// Lowest first. Defaults to 100.
    pub index: i64,
    /// Pass or fail messages, respectively
    ///
    /// When printing a criterion, the appropriate message
    /// will be printed. Not much use other than that.
    pub messages: (String, String),
    /// An optional description
    pub desc: Option<String>,
    /// The criterion's test
    ///
    /// Determines if the criterion passes or fails. This signature is
    /// required.
    pub test: Box<dyn Fn(&TestData) -> bool>,
    /// If the test passed, failed, or hasn't been run.
    ///
    /// `None` if it hasn't been run, Some(`true`) or Some(`false`) otherwise.
    /// If this value is `Some`, the test has been run.
    pub status: Option<bool>,
    /// Renders the criterion unable to be printed
    pub hide: bool,
}

impl Criterion {
    /// Returns a [`CriterionBuilder`](crate::criterion_builder::CriterionBuilder),
    /// which can be `built()`.
    /// ## Example
    /// **A basic criterion**
    /// ```rust
    /// use lab_grader::Criterion;
    ///
    /// let c = Criterion::new("my crit").build();
    /// assert_eq!(c.name, "my crit");
    /// ```
    pub fn new(name: &str) -> CriterionBuilder {
        CriterionBuilder::new(name)
    }

    /// Returns the success message, ie. the first message in the
    /// [`messages`](crate::criterion::Criterion::messages) tuple.
    pub fn success_message(&self) -> &String {
        &self.messages.0
    }

    /// Returns the failure message, ie. the second message in the
    /// [`messages`](crate::criterion::Criterion::messages) tuple.
    pub fn failure_message(&self) -> &String {
        &self.messages.1
    }

    /// Sets the test method of a criterion
    pub fn attach(&mut self, test: Box<dyn Fn(&TestData) -> bool>) {
        self.test = test
    }

    /// Runs the criterion's test function with the data provided.
    ///
    /// This is almost equivilent to calling `(criterion.test)(data)`, but this
    /// method also sets the status of the criterion to the result of the test.
    /// You should avoid calling the test directly, and call this or the
    /// [`test`](Criterion::test) method instead.
    ///
    /// The criterion must be mutable to call this method, as the status is changed
    /// to the result of the test.
    ///
    /// You shouldn't call this method directly, instead grade an entire
    /// [`Batch`](crate::batch::Batch).
    pub fn test_with_data(&mut self, data: &TestData) -> bool {
        self.status = Some((self.test)(data));
        self.status.unwrap()
    }

    /// Runs the criterions test and assigns the result to `criterion.status`.
    ///
    /// This is equivilent to running [`test_with_data`](crate::criterion::Criterion::test_with_data) with
    /// an empty `TestData`.
    ///
    /// You shouldn't call this method directly, instead grade an entire
    /// [`Batch`](crate::batch::Batch).
    pub fn test(&mut self) -> bool {
        self.test_with_data(&TestData::new())
    }

    /// Prints the essential criterion information in one line.
    ///
    /// Names will be padded with spaces on the left to 30 characters wide,
    /// so that printing many criteria at once will look better.
    pub fn print_short(&self) {
        if let Some(s) = self.status {
            if s {
                println!("{}  {}",
                    self.name,
                    Green.paint(self.success_message()));
            } else {
                println!("{}  {}",
                    self.name,
                    Red.paint(self.failure_message()));
            }
        } else {
            println!("{}  not tested", self.name);
        }
    }


    /// Returns the success message if the criterion passed, otherwise
    /// returns the failure message
    pub fn status_message(&self) -> String {
        if self.status == Some(true) {
            self.success_message().clone()
        } else {
            self.failure_message().clone()
        }
    }


    /// Same as [`status_message`](crate::criterion::Criterion::status_message), but
    /// the success message will be colored green and the failure message red.
    pub fn colored_status_message(&self) -> String {
        if self.status == Some(true) {
            format!("{}", Green.paint(self.success_message()))
        } else {
            format!("{}", Red.paint(self.failure_message()))
        }
    }


}

/// Displays the results of the criterion.
/// You should test the criterion before printing it.
impl fmt::Display for Criterion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hide {
            return write!(f, "");
        }
        let mut buffer = String::new();
        if let Some(status) = self.status {
            if status {
                // Success
                writeln!(&mut buffer, "{}", Green.bold().paint(&self.name)).unwrap();
                if let Some(d) = &self.desc {
                    writeln!(&mut buffer, "{}", White.paint(d)).unwrap();
                }
                writeln!(&mut buffer, "Worth: {} pts", self.worth).unwrap();
                writeln!(&mut buffer, "Status: {}", Green.paint(self.success_message())).unwrap();
            } else {
                // Failure
                writeln!(&mut buffer, "{}", Red.bold().paint(&self.name)).unwrap();
                if let Some(d) = &self.desc {
                    writeln!(&mut buffer, "{}", White.paint(d)).unwrap();
                }
                writeln!(&mut buffer, "Worth: {} pts", self.worth).unwrap();
                writeln!(&mut buffer, "Status: {}", Red.paint(self.failure_message())).unwrap();
            }
        } else {
            // Neutral
            writeln!(&mut buffer, "{}", White.bold().paint(&self.name)).unwrap();
            if let Some(d) = &self.desc {
                writeln!(&mut buffer, "{}", White.paint(d)).unwrap();
            }
            writeln!(&mut buffer, "Worth: {} pts", self.worth).unwrap();
            writeln!(&mut buffer, "Status: not tested").unwrap();
        }
        write!(f, "{}", buffer)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;

    fn test_crit() -> Criterion {
        Criterion::new("test")
            .stub("test-stub")
            .worth(10)
            .messages("success", "failure")
            .desc("short desc")
            .hide(false)
            .test(Box::new(|_: &TestData| true ))
            .build()
    }

    #[test]
    fn test_a_criterion_with_data_passes() {
        let mut crit = test_crit();
        let data = data! {
            "key" => "value"
        };
        assert!(crit.test_with_data(&data));
    }

    #[test]
    fn test_success_and_failure_messages() {
        let c = test_crit();
        assert_eq!(c.success_message(), "success");
        assert_eq!(c.failure_message(), "failure");
    }

    #[test]
    fn test_data_macro() {
        // The long way
        let mut map = TestData::new();
        map.insert(String::from("key"), String::from("value"));

        // the macro way
        let data = data! { "key" => "value" };
        assert_eq!(map, data);
    }
}
