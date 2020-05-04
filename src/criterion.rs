//! Definitions for creating and running criteria
//!
//! A criterion is one specific item in a series of items that form a grade.
//! Each criterion has a name, point value, and a related function.
//! Testing the criterion is running the function, which will return true
//! or false. A final grade can be calculated by adding up all the values
//! of the criteria, if they passed.
//!
//! The aim of this application is the make the definition of Criteria
//! as easy as possible, and the make that definition most of the work involved.
//! You shouldn't have to worry about students submitting, or persisting results.
//! Just define the criteria and distribute the program to your students.
use std::fmt;
use std::fmt::Write;

use ansi_term::Color::{Green, Red, White};

use crate::TestData;


/// A macro to easily create a `TestData` struct, which is
/// really just an alias to `HashMap<String, String>`
///
/// ## Example
/// ```rust
/// # #[macro_use] extern crate lab_grader;
/// use lab_grader::TestData;
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
macro_rules! data(
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

/// A criterion
///
/// This is the heart of the application. Each criterion is responsible for
/// checking one thing, and *one thing only*. You should build a list of criteria.
///
/// ## A lone `Criterion`
/// A Criterion has some informational fields (`name`, `messages`), a point value (`worth`),
/// a `status`, and most importantly a `test`. The test takes in a reference to
/// [`TestData`](crate::submission::TestData) and returns a `bool`. The signature of every
/// criterion's test is always the same.
///
///
/// ```rust
/// use lab_grader::*;
///
/// let mut crit = Criterion::new(
///     // Name
///     "My First Criterion",
///     // Worth
///     10,
///     // Pass/Fail messages, a tuple
///     ("passed", "failed"),
///     // Test function, contained in a Box
///     Box::new(|_: &TestData| -> bool {
///         // test code goes here
///         // determine if this should pass or fail
///         true
///     })
/// );
///
/// assert!(crit.status.is_none());
/// crit.test();
/// assert_eq!(crit.status, Some(true));
/// ```
///
///
/// We can also extract the test into a function defined elsewhere. This just helps with organization.
/// ```rust
/// # use lab_grader::*;
/// fn my_test(_: &TestData) -> bool {
///     // code here...
///     true
/// }
///
/// fn main() {
///     let mut crit = Criterion::new(
///         "My Second Criterion",
///         10,
///         ("passed", "failed"),
///         Box::new(my_test)
///     );
///
///     crit.test();
///     // ...
/// }
/// ```
///
///
/// We can also pass data to a criterion. This data *must* be a `&TestData`
/// ```rust
/// # use lab_grader::*;
///
/// fn my_test(data: &TestData) -> bool {
///     if let Some(value) = data.get("key") {
///         return value == "value"
///     }
///     false
/// }
///
/// fn main() {
///     let mut crit = Criterion::new(
///         "My Third Criterion",
///         10,
///         ("passed", "failed"),
///         Box::new(my_test)
///     );
///
///     // Now we need some data to pass to the criterion
///     // this crate provides a data macro that builds TestData
///     let data = data! {
///         "key" => "value"
///     };
///     crit.test_with_data(&data);
///     assert_eq!(crit.status, Some(true));
/// }
/// ```
pub struct Criterion {
    /// A short (< 30 characters), descriptive name
    pub name: String,
    /// Point value of this criterion. If it passes, this value
    /// will be added to the [`Submission`](crate::submission::Submission) grade.
    ///
    /// Can be negative if you wish to subtract points. Be sure to get your logic right.
    /// This value is added to the submission grade *if the test returns true*.
    pub worth: i16,
    /// Pass or fail messages, respectively
    ///
    /// When printing a criterion, the appropriate message
    /// will be printed. Not much use other than that.
    pub messages: (&'static str, &'static str),
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
    /// Currently does nothing because i'm lazy
    pub hide: bool,
}

impl Criterion {
    /// Creates a new Criterion with the given parameters.
    ///
    /// The `messages` parameter should be a tuple of
    /// `&str` containing a success then failure message, respectively.
    /// These messages will be printed when printing the criterion.
    ///
    /// The `test` parameter is a [`Box`][box] around a closure accepting
    /// a reference to [TestData][testdata] returning a bool. This can get a bit confusing.
    /// The `test` closure should return true if the criterion passes, otherwise false.
    /// The `&TestData` parameter allows data from outside the closure to be passed in. `TestData` is
    /// just an alias to `HashMap<String, String>`, so all keys and values must be `String`s.
    ///
    /// [testdata]: crate::submission::TestData
    /// [box]: std::boxed::Box
    ///
    /// ## Example
    /// **A basic criterion**
    /// ```rust
    /// use lab_grader::{Criterion, TestData};
    ///
    /// let mut c = Criterion::new(
    ///     "A test criterion",
    ///     10,
    ///     ("Success!", "Failure!"),
    ///     Box::new(|_: &TestData| {
    ///         // Code to test criterion goes here,
    ///         // and should return false or...
    ///         true
    ///     })
    /// );
    /// assert!(c.test());
    /// ```
    ///
    /// **A criterion with data**
    /// ```rust
    /// # #[macro_use] extern crate lab_grader;
    /// # use lab_grader::{Criterion, TestData};
    ///
    /// let mut c = Criterion::new(
    ///     "A test criterion with data!",
    ///     10,
    ///     ("Success!", "Failure!"),
    ///     Box::new(|data: &TestData| {
    ///         return data["my_key"] == "my_value";
    ///     })
    /// );
    ///
    /// // The above criterion takes a `&TestData` into it's closure,
    /// // so we must establish the data to send into the closure
    /// let my_data = data! {
    ///     "my_key" => "my_value"
    /// };
    ///
    /// assert!(c.test_with_data(&my_data));
    /// ```
    pub fn new<S: AsRef<str>>(
        name: S,
        worth: i16,
        messages: (&'static str, &'static str),
        test: Box<dyn Fn(&TestData) -> bool>,
    ) -> Self {
        Criterion {
            name: String::from(name.as_ref()),
            worth,
            messages,
            test,
            status: None,
            hide: false,
        }
    }

    /// Returns the success message, ie. the first message in the [`messages`][msg] tuple
    ///
    /// [msg]: Criterion::new
    pub fn success_message(&self) -> &'static str {
        self.messages.0
    }

    /// Returns the failure message, ie. the second message in the [`messages`][msg] tuple
    ///
    /// [msg]: Criterion::new
    pub fn failure_message(&self) -> &'static str {
        self.messages.1
    }


    /// Toggles the `hide` field on a criterion
    ///
    /// If hide is true, printing the criterion with the default
    /// formatter will print nothing. Good if you want a secret criterion
    /// that the students don't know about
    pub fn hide(&mut self) {
        self.hide = !self.hide;
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
    /// ## Example
    /// ```rust
    /// # #[macro_use] extern crate lab_grader;
    /// # use lab_grader::{Criterion, TestData};
    ///
    /// let mut c = Criterion::new(
    ///     "A test criterion with data!",
    ///     10,
    ///     ("Success!", "Failure!"),
    ///     Box::new(|data: &TestData| {
    ///         return data["my_key"] == "my_value";
    ///     })
    /// );
    ///
    /// let my_data = data! {
    ///     "my_key" => "my_value"
    /// };
    ///
    /// c.test_with_data(&my_data);
    /// // It's either Some(true) or Some(false) since we've tested
    /// assert!(c.status.is_some());
    /// ```
    pub fn test_with_data(&mut self, data: &TestData) -> bool {
        self.status = Some((self.test)(data));
        self.status.unwrap()
    }

    /// Runs the criterions test and assigns the result to `criterion.status`.
    ///
    /// This is equivilent to running [`test_with_data`](crate::criterion::Criterion::test_with_data) with
    /// an empty `TestData`.
    ///
    /// Criterion must be mutable.
    ///
    /// ## Example
    /// ```rust
    /// # use lab_grader::{Criterion, TestData};
    ///
    /// let mut c = Criterion::new(
    ///     "A test criterion with data!",
    ///     10,
    ///     ("Success!", "Failure!"),
    ///     Box::new(|_: &TestData| {
    ///         true
    ///     })
    /// );
    ///
    /// assert!(c.test());
    /// assert!(c.status.is_some());
    /// ```
    pub fn test(&mut self) -> bool {
        self.test_with_data(&TestData::new())
    }

}

/// Displays the results of the criterion.
/// You should test the criterion before printing it.
///
/// Output will be aligned, as you'll normally be printing
/// a lot of these at once.
///
/// Given a configuration with the name `Test criterion`,
/// success message `passed!`, and failure message `failed!`,
/// this is what would print:
///
/// **Printed before testing**
/// ```text
/// My first criterion  +**  not tested
/// ```
/// **Printed after a successful test**
/// ```text
/// My first criterion  +10  passed!
/// ```
/// **Printed after a failed test**
/// ```text
/// My first criterion  + 0  failed!
/// ```
impl fmt::Display for Criterion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        if let Some(status) = self.status {
            if status {
                // Success
                writeln!(&mut buffer, "{}", Green.bold().paint(&self.name)).unwrap();
                writeln!(&mut buffer, "Worth: {} pts", self.worth).unwrap();
                writeln!(&mut buffer, "Status: {}", Green.paint(self.success_message())).unwrap();
            } else {
                // Failure
                writeln!(&mut buffer, "{}", Red.bold().paint(&self.name)).unwrap();
                writeln!(&mut buffer, "Worth: {} pts", self.worth).unwrap();
                writeln!(&mut buffer, "Status: {}", Red.paint(self.failure_message())).unwrap();
            }
        } else {
            // Neutral
            writeln!(&mut buffer, "{}", White.bold().paint(&self.name)).unwrap();
            writeln!(&mut buffer, "Worth: {} pts", self.worth).unwrap();
            writeln!(&mut buffer, "Status: not tested").unwrap();
        }
        write!(f, "{}", buffer)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_criterion() {
        let mut c = Criterion::new(
            "A test criterion",
            10,
            ("passed!", "failed!"),
            Box::from(|_: &TestData| -> bool { true }),
        );
        assert_eq!(c.name, "A test criterion");
        assert_eq!(c.worth, 10);
        assert!(c.status.is_none());
        assert!(c.test());
        assert!(c.status.is_some());
    }

    #[test]
    fn test_a_criterion_with_data_passes() {
        let mut c = Criterion::new(
            "A test criterion",
            10,
            ("succes!", "failure!"),
            Box::from(|data: &TestData| -> bool {
                return data["my_var"] == "value";
            }),
        );

        let data = data! {
            "my_var" => "value"
        };

        assert!(c.test_with_data(&data));
    }

    #[test]
    fn test_success_and_failure_messages() {
        let c = Criterion::new(
            "A test criterion",
            10,
            ("passed!", "failed!"),
            Box::from(|_: &TestData| -> bool { true }),
        );
        assert_eq!(c.success_message(), "passed!");
        assert_eq!(c.failure_message(), "failed!");
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
