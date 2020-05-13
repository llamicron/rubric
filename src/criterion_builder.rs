use crate::{TestData, Criterion};


/// A builder struct that builds a Criterion. You should create one
/// of these through [`Criterion::new`](crate::criterion::Criterion::new)
/// instead of directly.
pub struct CriterionBuilder {
    name: String,
    stub: Option<String>,
    worth: i16,
    messages: (String, String),
    desc: Option<String>,
    test: Option<Box<dyn Fn(&TestData) -> bool>>,
    index: i64,
    hide: bool
}

impl CriterionBuilder {
    /// Creates a new CriterionBuilder
    ///
    /// ```rust
    /// # use lab_grader::criterion_builder::CriterionBuilder;
    /// let builder = CriterionBuilder::new("my crit").build();
    /// ```
    pub fn new(name: &str) -> Self {
        CriterionBuilder {
            name: String::from(name.trim()),
            stub: None,
            worth: 0,
            messages: ("passed".to_string(), "failed".to_string()),
            desc: None,
            test: None,
            index: 100,
            hide: false
        }
    }

    /// Sets the stub of a criterion. It's like an identifier.
    ///
    /// A stub should be lowercase and not contain whitespace, it should
    /// also be unique among criteria.
    ///
    /// ```rust
    /// # use lab_grader::criterion_builder::CriterionBuilder;
    /// let crit = CriterionBuilder::new("my crit")
    ///     .stub("my-stub")
    ///     .build();
    /// ```
    pub fn stub(mut self, stub: &str) -> Self {
        self.stub = Some(String::from(stub));
        self
    }

    /// Sets the index
    pub fn index(mut self, index: i64) -> Self {
        self.index = index;
        self
    }

    /// Attaches a test.
    ///
    /// ```rust
    /// # use lab_grader::criterion_builder::CriterionBuilder;
    /// # use lab_grader::TestData;
    /// fn my_test(_: &TestData) -> bool {
    ///     true
    /// }
    ///
    /// let crit = CriterionBuilder::new("my crit")
    ///     .test(Box::new(my_test))
    ///     .build();
    /// ```
    pub fn test(mut self,
        test: Box<dyn Fn(&TestData) -> bool>) -> Self {
        self.test = Some(test);
        self
    }

    /// Sets the messages of a criterion.
    ///
    /// ```rust
    /// # use lab_grader::criterion_builder::CriterionBuilder;
    /// let crit = CriterionBuilder::new("my crit")
    ///     .messages("passed", "failed")
    ///     .build();
    /// ```
    pub fn messages(mut self, success: &str, failure: &str) -> Self {
        self.messages = (
            String::from(success),
            String::from(failure)
        );
        self
    }

    /// Sets the description of a criterion. It should be
    /// relatively short.
    ///
    /// ```rust
    /// # use lab_grader::criterion_builder::CriterionBuilder;
    /// let crit = CriterionBuilder::new("Git installed")
    ///     .desc("Tests that Git is installed")
    ///     .build();
    /// ```
    pub fn desc(mut self, desc: &str) -> Self {
        self.desc = Some(String::from(desc));
        self
    }

    /// Sets the worth on a Criterion
    pub fn worth(mut self, worth: i16) -> Self {
        self.worth = worth;
        self
    }

    /// Sets the hide flag on a criterion. If hide is true,
    /// the criterion can't be printed.
    ///
    /// ```rust
    /// # use lab_grader::criterion_builder::CriterionBuilder;
    /// let crit = CriterionBuilder::new("my crit")
    ///     .hide(true)
    ///     .build();
    /// ```
    pub fn hide(mut self, hide: bool) -> Self {
        self.hide = hide;
        self
    }

    /// Finalizes the criterion.
    ///
    /// If a stub wasn't manually set, it will create one based on the
    /// name. It will lowercase it, then replace all whitespace with dashes.
    ///
    /// ```rust
    /// # use lab_grader::criterion_builder::CriterionBuilder;
    /// let crit = CriterionBuilder::new("my crit")
    ///     // more confiuration options...
    ///     .build();
    /// ```
    pub fn build(self) -> Criterion {
        // Build stub
        let stub;
        if self.stub.is_none() {
            let re = regex::Regex::new(r" +").unwrap();
            stub = re.replace_all(self.name.trim(), "-").to_lowercase();
        } else {
            stub = self.stub.unwrap();
        }

        let test: Box<dyn Fn(&TestData) -> bool>;
        if self.test.is_none() {
            test = Box::new(|_: &TestData| false );
        } else {
            test = self.test.unwrap();
        }

        Criterion {
            stub: stub,
            name: self.name,
            worth: self.worth,
            messages: self.messages,
            desc: self.desc,
            test: test,
            index: self.index,
            status: None,
            hide: self.hide
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_name() {
        let cb = CriterionBuilder::new("my crit");
        assert_eq!(cb.name, "my crit");
        assert_eq!(cb.worth, 0);

        assert_eq!(cb.messages, (
            "passed".to_string(),
            "failed".to_string()
        ));

        assert!(cb.stub.is_none());
        assert!(cb.desc.is_none());
        assert!(cb.test.is_none());
        assert!(!cb.hide);
    }

    #[test]
    fn test_default_values() {
        let crit = CriterionBuilder::new("My Crit").build();
        assert_eq!(crit.name, "My Crit");
        assert_eq!(crit.stub, "my-crit");
        assert_eq!(crit.messages.0, "passed");
        assert_eq!(crit.messages.1, "failed");
        assert!(crit.desc.is_none());
        assert!(!crit.hide);
    }

    #[test]
    fn test_build_parameters() {
        let crit = CriterionBuilder::new("my crit")
            .stub("my-stub")
            .messages("success", "failed :(")
            .desc("Here's my desc")
            .hide(true)
            .build();

        assert_eq!(crit.name, "my crit");
        assert_eq!(crit.stub, "my-stub");
        assert_eq!(crit.messages, (
            "success".to_string(),
            "failed :(".to_string()
        ));
        assert_eq!(crit.desc.unwrap(), "Here's my desc");
        assert!(crit.hide);

    }

    #[test]
    fn test_build() {
        let cb = CriterionBuilder::new("my crit");
        let crit = cb.build();
        assert_eq!(crit.name, "my crit");
    }
}
