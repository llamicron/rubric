//! A batch of criteria, with some extra metadata
use std::str::FromStr;
use std::process::exit;
use std::fmt;

use ansi_term::Color;

use crate::{Criteria, TestData};
use crate::yaml::BatchYaml;


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
}

impl Batch {
    /// Builds a Batch from yaml data
    ///
    /// This method calls `parse()` from the [`FromStr`](std::str::FromStr). `parse()`
    /// returns a `Result`, and this method just prints the result in a clean way and exits
    /// with [`exit`](std::process::exit). It's just one less thing to `unwrap`.
    ///
    /// `exit`ing from within a library function is usually bad practice, but this will only
    /// exit if the yaml passed in has an error. The yaml you'll write to form a Batch is written
    /// during development, and is embedded in the executable. This means that if you're code compiles,
    /// it can be distributed and this will never `exit`.
    ///
    /// If you want to deal with the result yourself, call `parse` instead of this.
    ///
    /// ## Example
    /// ```rust
    /// use lab_grader::*;
    ///
    /// let yaml_data = yaml!("../test_data/test_batch.yml").expect("Couldn't open that file");
    /// let batch = Batch::from_yaml(yaml_data);
    ///
    /// // Now you've got a batch
    /// assert!(batch.name.len() > 1);
    /// ```
    pub fn from_yaml(yaml: &str) -> Self {
        match yaml.parse::<Self>() {
            Ok(b) => return b,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }


    /// Gets a criterion by stub
    pub fn attach(&mut self, stub: &str, func: Box<dyn Fn(&TestData) -> bool>) {
        self.criteria.attach(stub, func);
    }
}


impl FromStr for Batch {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Construct BatchYaml from yaml data
        let batch_yaml = serde_yaml::from_str::<BatchYaml>(s)?;

        // Pull out the criteria
        let mut criteria = Criteria::from(vec![]);
        for (name, crit) in batch_yaml.criteria {
            criteria.add(crit.into_criterion(name));
        }

        // Construct a batch
        Ok(Batch {
            name: batch_yaml.name,
            desc: batch_yaml.desc,
            criteria: criteria
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
        yaml!("../examples/criteria/example.yml").unwrap()
    }

    #[test]
    fn test_from_yaml() {
        let batch = Batch::from_yaml(yaml_data());
        assert_eq!(batch.name, "My Example Batch");
        assert!(batch.desc.is_some());
    }

    #[test]
    fn test_parse_yaml() {
        let raw = r#"
            name: My Example Batch
            desc: Here's an example of a batch with a list of criteria
            criteria:
                First criterion:
                    stub: some-unique-stub
                    index: 0
                    desc: This is the first criterion
                    worth: 10
                    messages: ["success", "failure"]
                    hide: false
        "#;

        assert!(raw.parse::<Batch>().is_ok());
    }
}
