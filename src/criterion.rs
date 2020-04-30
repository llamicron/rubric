use std::collections::HashMap;
use std::fmt;


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
    /// # use std::collections::HashMap;
    /// # use lab_grader::criterion::Criterion;
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
    /// let mut my_data = HashMap::new();
    /// my_data.insert("my_key".to_string(), "my_value".to_string());
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
    /// let mut my_data = HashMap::new();
    /// my_data.insert("my_key".to_string(), "my_value".to_string());
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

impl fmt::Display for Criterion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg: String;
        if let Some(status) = self.status {
            if status {
                // success
                msg = format!("✅ {}: {}", self.name, self.success_message());
            } else {
                // Error
                msg = format!("❌ {}: {}", self.name, self.failure_message());
            }
        } else {
            // not yet run
            msg = format!("⚪ {}: not tested", self.name);
        }
        write!(f, "{}", msg)
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

        let mut data: HashMap<String, String> = HashMap::new();
        data.insert(String::from("my_var"), String::from("value"));

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

        // You need to test it before it will print successfully
        assert_eq!(format!("{}", c), "⚪ Test criterion: not tested");

        // Test it first
        c.test();
        assert_eq!(format!("{}", c), "✅ Test criterion: passed!");

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
        assert_eq!(format!("{}", c2), "❌ Test criterion: failed!");
    }

}
