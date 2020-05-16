//! A batch of criteria, with some extra metadata

// std uses
use std::str::FromStr;
use std::fmt;

// external uses
use ansi_term::Color;

// internal uses
use crate::{Criteria, TestData};
use crate::yaml::BatchYaml;
use crate::error::{Result, Error};

/// Attaches multiple functions to a batch. Will panic if
/// any criterion with the given stub isn't found.
///
/// ## Example
/// ```no_compile
/// use lab_grader::*;
/// fn my_test_func(data: &TestData) -> bool {
///     true
/// }
///
/// fn main() {
///     // Assume this has a criterion with the stub "my-stub"
///     let mut batch = Batch::from_yaml(/* ... */).unwrap();
///     attach! {
///         batch,
///         "my-stub" => my_test_func
///     }
/// }
///
/// ```
#[macro_export]
macro_rules! attach {
    ( $batch:ident, $($stub:literal => $func:ident),* ) => {
        $(
            $batch.attach($stub, Box::new($func)).expect("criterion not found");
        )+
    };
}

/// A bundle of metadata with a set of criteria.
///
/// The main purpose of a `Batch` is to deserialize criteria from a yaml file. This
/// struct provides a `from_yaml` method that takes yaml data and turns it into a batch.
///
/// [Specification for the yaml data](https://github.com/llamicron/lab_grader/wiki/YAML-Specification)
pub struct Batch {
    pub name: String,
    pub desc: Option<String>,
    pub criteria: Criteria,
    pub total: isize
}

impl Batch {
    /// Builds a batch from yaml data.
    ///
    /// ## Example
    /// ```rust
    /// # use lab_grader::batch::Batch;
    /// # use lab_grader::yaml;
    /// let yaml = yaml!("../test_data/test_batch.yml").unwrap();
    /// let batch = Batch::from_yaml(yaml).expect("Bad yaml!");
    /// ```
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        match yaml.parse::<Self>() {
            Ok(batch) => Ok(batch),
            Err(e) => {
                match e.location() {
                    Some(loc) => return Err(Error::bad_yaml(loc.line(), loc.column())),
                    None => return Err(Error::bad_yaml(0, 0)),
                }
            }
        }
    }


    /// Gets a criterion by stub
    pub fn attach(&mut self, stub: &str,
        func: Box<dyn Fn(&TestData) -> bool>) -> Result<()> {
        self.criteria.attach(stub, func)
    }

    /// An alternative to printing through `println!("{}", batch)`.
    ///
    /// This prints a shorted report where each criterion is on it's own line,
    /// and only the criterion name and status is printed. The batch description
    /// is also hidden.
    pub fn print_short(&self) {
        println!("{}", Color::White.bold().paint(&self.name));
        self.criteria.print_short();
        println!("{}/{}", self.criteria.points(), self.criteria.total_points());
    }
}


impl FromStr for Batch {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // Construct BatchYaml from yaml data
        let batch_yaml = serde_yaml::from_str::<BatchYaml>(s)?;

        // Pull out the criteria
        let mut criteria = Criteria::from(vec![]);
        for (name, crit) in batch_yaml.criteria {
            criteria.add(crit.into_criterion(name));
        }

        let criteria_total = criteria.total_points();
        if let Some(t) = batch_yaml.total {
            if criteria_total != t {
                eprint!("{}", Color::Red.paint("Warning: "));
                eprintln!("Batch total does not match criteria total: batch = {}, criteria = {}",
                    t, criteria_total);
            }
        }


        // Construct a batch
        Ok(Batch {
            name: batch_yaml.name,
            desc: batch_yaml.desc,
            criteria: criteria,
            total: criteria_total
        })
    }
}

impl fmt::Display for Batch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n-- {} --", Color::White.bold().paint(&self.name)).unwrap();
        if let Some(desc) = &self.desc {
            writeln!(f, "{}\n", desc).unwrap();
        }
        write!(f, "{}", self.criteria)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::yaml;

    fn yaml_data() -> &'static str {
        yaml!("../test_data/test_batch.yml").unwrap()
    }

    #[test]
    fn test_from_yaml() {
        let batch = Batch::from_yaml(yaml_data()).expect("Bad yaml");
        assert_eq!(batch.name, "Test batch");
        assert!(batch.desc.is_some());
    }

    #[test]
    fn test_attach_macro() {
        fn test_fn(_: &TestData) -> bool { true };

        let mut batch = Batch::from_yaml(yaml_data()).expect("Bad yaml");
        assert!(!batch.criteria.get("first-crit").unwrap().test());

        attach! {
            batch,
            "first-crit" => test_fn
        };

        assert!(batch.criteria.get("first-crit").unwrap().test());

    }

    #[test]
    fn test_parse_yaml() {
        let raw = r#"
            name: Test batch
            desc: here's a short description
            criteria:
                First Criterion:
                    stub: first-crit,
                    index: 0
                    desc: "First criterion short desc"
                    worth: 50
                    messages: ["success", "failure"]
                    hide: false

                Second Criterion:
                    stub: second-crit
                    worth: 30
        "#;

        assert!(raw.parse::<Batch>().is_ok());
    }
}
