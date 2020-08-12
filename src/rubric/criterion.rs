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
//! build that into a [`Rubric`](crate::rubric::Rubric).

// external uses
use paris::{Logger, formatter::Formatter};

// internal uses
use crate::TestData;
use crate::rubric::CriterionBuilder;


/// A single Criterion
pub struct Criterion {
    /// The name of the function that serves as this criterions test
    /// This is just the name, used to attach the function.
    /// See the `test` field.
    pub func: String,
    /// A short (< 30 characters), descriptive name
    pub name: String,
    /// Point value of this criterion. If it passes, this value
    /// will be added to the [`Submission`](crate::submission::Submission) grade.
    ///
    /// Can be negative if you wish to subtract points. Be sure to get your logic right.
    /// This value is added to the submission grade *if the test returns true*.
    pub worth: isize,
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
    /// use rubric::rubric::Criterion;
    ///
    /// let c = Criterion::new("my crit").build();
    /// assert_eq!(c.name, "my crit");
    /// ```
    pub fn new(name: &str) -> CriterionBuilder {
        CriterionBuilder::new(name)
    }

    /// Returns the success message, ie. the first message in the
    /// [`messages`](crate::rubric::criterion::Criterion::messages) tuple.
    pub fn success_message(&self) -> &String {
        &self.messages.0
    }

    /// Returns the failure message, ie. the second message in the
    /// [`messages`](crate::rubric::criterion::Criterion::messages) tuple.
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
    /// [`Rubric`](crate::rubric::Rubric).
    pub fn test_with_data(&mut self, data: &TestData) -> bool {
        self.status = Some((self.test)(data));
        self.status.unwrap()
    }

    /// Runs the criterions test and assigns the result to `criterion.status`.
    ///
    /// This is equivilent to running [`test_with_data`](crate::rubric::criterion::Criterion::test_with_data) with
    /// an empty `TestData`.
    ///
    /// You shouldn't call this method directly, instead grade an entire
    /// [`Rubric`](crate::rubric::Rubric).
    pub fn test(&mut self) -> bool {
        self.test_with_data(&TestData::new())
    }

    /// Prints the essential criterion information in one line.
    /// Will do nothing if the `hide` field is true
    pub fn print_short(&self) {
        if self.hide {
            return;
        }

        let mut log = Logger::new();
        
        if let Some(s) = self.status {
            // Already tested, diff color based on status
            if s {
                log.same().success(&self.name).log(
                    format!("\t<green>{}</>", self.status_message())
                );
            } else {
                log.same().error(&self.name).log(
                    format!("\t<red>{}</>", self.status_message())
                );
            }
        } else {
            // Not tested
            log.same().warn(&self.name).log("<bold>Not Tested</>");
        }
    }

    pub fn print_long(&self) {
        // Never print if it's hidden
        if self.hide {
            return;
        }

        let mut log = Logger::new();
        // Name and status
        if let Some(s) = self.status {
            if s {
                log.same().success(&self.name);
            } else {
                log.same().error(&self.name);
            }
            // Status message, color already added
            log.same().log("  ").log(self.colored_status_message());
        } else {
            // Hasn't been tested
            log.warn(format!("{}  <bold>Not Tested</>", self.name));
        }

        // Description
        if let Some(desc) = &self.desc {
            log.info(desc);
        }
        
        // Worth
        log.info(format!("Worth: <bold>{}</>", self.worth));
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


    /// Same as [`status_message`](crate::rubric::criterion::Criterion::status_message), but
    /// the success message will be colored green and the failure message red.
    pub fn colored_status_message(&self) -> String {
        let fmt = Formatter::new();
        if self.status == Some(true) {
            fmt.colorize(&format!("<green>{}</>", self.success_message()))
        } else {
            fmt.colorize(&format!("<red>{}</>", self.failure_message()))
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;

    fn test_crit() -> Criterion {
        Criterion::new("test")
            .func("test_func")
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
