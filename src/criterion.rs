use std::collections::HashMap;


pub struct Criterion {
    pub name: String,
    pub worth: i16,
    pub messages: (&'static str, &'static str),
    pub test: Box<dyn Fn(HashMap<String, String>) -> bool>,
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
    /// The `HashMap` parameter allows data from outside the closure to be passed in. The signature
    /// of the `HashMap` is `HashMap<String, String>`, so all keys and values must be `String`s.
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
    /// let c = Criterion::new(
    ///     "A test criterion",
    ///     10,
    ///     ("Success!", "Failure!"),
    ///     Box::new(|_: HashMap<String, String>| {
    ///         // Code to test criterion goes here,
    ///         // and should return false or...
    ///         true
    ///     })
    /// );
    /// assert!((c.test)(HashMap::new()));
    /// ```
    ///
    /// **A criterion with data**
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use lab_grader::criterion::Criterion;
    ///
    /// // Establish the data to send into the criterion
    /// let mut my_data = HashMap::new();
    /// my_data.insert("my_key".to_string(), "my_value".to_string());
    ///
    /// let c = Criterion::new(
    ///     "A test criterion with data!",
    ///     10,
    ///     ("Success!", "Failure!"),
    ///     Box::new(|data: HashMap<String, String>| {
    ///         return data["my_key"] == "my_value";
    ///     })
    /// );
    /// assert!((c.test)(my_data));
    /// ```
    pub fn new<S: AsRef<str>>(
        name: S,
        worth: i16,
        messages: (&'static str, &'static str),
        test: Box<dyn Fn(HashMap<String, String>) -> bool>
        ) -> Self {

            Criterion {
            name: String::from(name.as_ref()),
            worth,
            messages,
            test
        }
    }

    pub fn success_message(&self) -> &'static str {
        self.messages.0
    }

    pub fn failure_message(&self) -> &'static str {
        self.messages.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_criterion() {
        let c = Criterion::new(
            "A test criterion",
            10,
            ("passed!", "failed!"),
            Box::from(|_: HashMap<String, String>| {
                true
            })
        );
        assert_eq!(c.name, "A test criterion");
        assert_eq!(c.worth, 10);
        assert!((c.test)(HashMap::new()));
    }

    #[test]
    fn test_a_criterion_with_data_passes() {
        let c = Criterion::new(
            "A test criterion",
            10,
            ("succes!", "failure!"),
            Box::from(|data: HashMap<String, String>| {
                return data["my_var"] == "value";
            })
        );

        let mut data: HashMap<String, String> = HashMap::new();
        data.insert(String::from("my_var"), String::from("value"));

        assert!((c.test)(data));
    }
}
