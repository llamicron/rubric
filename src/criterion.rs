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
use std::collections::HashMap;
use std::fmt;

use ansi_term::Color::{Green, Red, White};
use ansi_term::ANSIGenericString;

/// A macro to easily create a `HashMap<String, String>`
///
/// ## Example
/// ```rust
/// # #[macro_use] extern crate lab_grader;
/// # use std::collections::HashMap;
///
/// // The long way
/// let mut map = HashMap::new();
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
pub struct Criterion {
    pub name: String,
    pub worth: i16,
    pub messages: (&'static str, &'static str),
    pub test: Box<dyn Fn(&HashMap<String, String>) -> bool>,
    pub status: Option<bool>
}

impl Criterion {
    /// Creates a new Criterion with the given parameters.
    ///
    /// The `messages` parameter should be a tuple of
    /// `&str` containing a success then failure message, respectively.
    /// These messages will be printed when printing the criterion.
    ///
    /// The `test` parameter is a [`Box`][box] around a closure accepting
    /// a [HashMap][hashmap] returning a bool. This can get a bit confusing.
    /// The `test` closure should return true if the criterion passes, otherwise false.
    /// The `&HashMap` parameter allows data from outside the closure to be passed in. The signature
    /// of the `&HashMap` is `&HashMap<String, String>`, so all keys and values must be `String`s.
    /// This is done to generalize the `test` field, as all criteria must have the same signature.
    ///
    /// [hashmap]: std::collections::HashMap
    /// [box]: std::boxed::Box
    ///
    /// ## Example
    /// **A basic criterion**
    /// ```rust
    /// use std::collections::HashMap;
    /// use lab_grader::criterion::Criterion;
    ///
    /// let mut c = Criterion::new(
    ///     "A test criterion",
    ///     10,
    ///     ("Success!", "Failure!"),
    ///     Box::new(|_: &HashMap<String, String>| {
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
    /// # use lab_grader::criterion::Criterion;
    /// # use std::collections::HashMap;
    ///
    /// let mut c = Criterion::new(
    ///     "A test criterion with data!",
    ///     10,
    ///     ("Success!", "Failure!"),
    ///     Box::new(|data: &HashMap<String, String>| {
    ///         return data["my_key"] == "my_value";
    ///     })
    /// );
    ///
    /// // The above criterion takes a HashMap into it's closure,
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
        test: Box<dyn Fn(&HashMap<String, String>) -> bool>
        ) -> Self {

            Criterion {
            name: String::from(name.as_ref()),
            worth,
            messages,
            test,
            status: None
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
    /// # use lab_grader::criterion::Criterion;
    /// # use std::collections::HashMap;
    ///
    /// let mut c = Criterion::new(
    ///     "A test criterion with data!",
    ///     10,
    ///     ("Success!", "Failure!"),
    ///     Box::new(|data: &HashMap<String, String>| {
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
    pub fn test_with_data(&mut self, data: &HashMap<String, String>) -> bool {
        self.status = Some((self.test)(data));
        self.status.unwrap()
    }

    /// Runs the criterions test and assigns the result to `criterion.status`.
    ///
    /// This is equivilent to running [`test_with_data`](Criterion::test_with_data) with
    /// an empty `HashMap`.
    ///
    /// Criterion must be mutable.
    ///
    /// ## Example
    /// ```rust
    /// # use lab_grader::criterion::Criterion;
    /// # use std::collections::HashMap;
    ///
    /// let mut c = Criterion::new(
    ///     "A test criterion with data!",
    ///     10,
    ///     ("Success!", "Failure!"),
    ///     Box::new(|_: &HashMap<String, String>| {
    ///         true
    ///     })
    /// );
    ///
    /// assert!(c.test());
    /// assert!(c.status.is_some());
    /// ```
    pub fn test(&mut self) -> bool {
        self.test_with_data(&HashMap::new())
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
        let name: ANSIGenericString<str>;
        let worth: ANSIGenericString<str>;
        let reason: ANSIGenericString<str>;
        if let Some(status) = self.status {
            if status {
                // success
                name   = Green.paint(format!("{:>20}", &self.name));
                worth  = Green.paint(format!("{:>2}", self.worth));
                reason = White.paint(self.success_message().to_string());
            } else {
                name   = Red.paint(format!("{:>20}", &self.name));
                worth  = Red.paint(format!("{:>2}", 0));
                reason = White.paint(self.failure_message().to_string());
                // Error
            }
        } else {
            // not yet run
            name   = White.paint(format!("{:>20}", &self.name));
            worth  = White.paint("**");
            reason = White.paint(format!("not tested"));
        }
        write!(f, "{}  +{}  {}", name, worth, reason)
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
            Box::from(|_: &HashMap<String, String>| {
                true
            })
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
            Box::from(|data: &HashMap<String, String>| {
                return data["my_var"] == "value";
            })
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
            Box::from(|_: &HashMap<String, String>| {
                true
            })
        );
        assert_eq!(c.success_message(), "passed!");
        assert_eq!(c.failure_message(), "failed!");
    }

    #[test]
    fn test_display() {
        // This criterion will always pass
        let mut c = Criterion::new(
            "Test criterion",
            10,
            ("passed!", "failed!"),
            Box::from(|_: &HashMap<String, String>| {
                true
            })
        );

        // Lots of hiddent characters following...
        // You need to test it before it will print successfully
        assert_eq!(format!("{}", c), "\u{1b}[37m      Test criterion\u{1b}[0m  +\u{1b}[37m**\u{1b}[0m  \u{1b}[37mnot tested\u{1b}[0m");

        // Test it first
        c.test();
        assert_eq!(format!("{}", c), "\u{1b}[32m      Test criterion\u{1b}[0m  +\u{1b}[32m10\u{1b}[0m  \u{1b}[37mpassed!\u{1b}[0m");

        // and this one will always fail
        let mut c2 = Criterion::new(
            "Test criterion",
            10,
            ("passed!", "failed!"),
            Box::from(|_: &HashMap<String, String>| {
                false
            })
        );
        // Test it first
        c2.test();
        assert_eq!(format!("{}", c2), "\u{1b}[31m      Test criterion\u{1b}[0m  +\u{1b}[31m 0\u{1b}[0m  \u{1b}[37mfailed!\u{1b}[0m");
    }

    #[test]
    fn test_data_macro() {
        // The long way
        let mut map = HashMap::new();
        map.insert(String::from("key"), String::from("value"));

        // the macro way
        let data = data! { "key" => "value" };
        assert_eq!(map, data);
    }
}