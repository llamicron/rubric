//! A batch of criteria, with some extra metadata

// std uses
use std::str::FromStr;
use std::process::exit;
use std::fmt;

// external uses
use ansi_term::Color;

// internal uses
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
    pub total: isize
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
        let batch = Batch::from_yaml(yaml_data());
        assert_eq!(batch.name, "Test batch");
        assert!(batch.desc.is_some());
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
