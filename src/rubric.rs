//! A collection of criteria and other data
//!
//! A rubric is a collection of criteria, along with a name and optional
//! description. It's meant to be created by serializing `yaml` data.
//!
//! **Note:** throughout the `Rubric` documentation, I'll be loading some test
//! yaml, it looks like this:
//! ```yaml
//! name: Test Rubric
//! desc: here's a short description
//! total: 80
//!
//! criteria:
//!   First Criterion:
//!     stub: first-crit
//!     index: 0
//!     desc: "First criterion short desc"
//!     worth: 50
//!     messages: ["success", "failure"]
//!     hide: false
//!
//!   Second Criterion:
//!     stub: second-crit
//!     worth: 30
//! ```
//! See the [YAML specification](https://github.com/llamicron/lab_grader/wiki/YAML-Specification) for more info.

// std uses
use std::str::FromStr;
use std::fmt;

// external uses
use prettytable::{Table, row, cell};
use ansi_term::Color;

// internal uses
use crate::yaml::RubricYaml;
use crate::Criterion;
use crate::error::{Result, Error};

/// Attaches tests to criteria in a rubric.
///
/// When you create a rubric from `yaml`, the criteria inside
/// don't have tests attached to them. You can call
/// [`Rubric.attach()`](crate::rubric::Rubric::attach) to achieve the
/// same thing, but this is faster and easier.
///
/// ## Example
/// ```no_compile
/// // A test meant to be attached to a criteria
/// fn some_test(_: &TestData) -> bool {
///     true
/// }
///
/// fn main() {
///     let mut rubric = Rubric::from_yaml(/* some yaml data */);
///     attach! {
///         rubric,
///         "some-stub" => some_test
///     };
/// }
/// ```
#[macro_export]
macro_rules! attach {
    ( $rubric:ident, $($stub:literal => $func:ident),* ) => {
        $(
            if let Some(c) = $rubric.get($stub) {
                c.attach(Box::new($func));
            } else {
                panic!("Criterion with stub {} not found, can't attach function", $stub);
            }
        )+
    };
}


/// A collection of criteria, meant to be serialized from `yaml`.
///
/// ## Example
/// ```rust
/// use lab_grader::{Rubric, yaml};
///
/// // Relative path to the yaml file
/// let yaml = yaml!("../test_data/test_rubric.yml").expect("Couldn't load yaml");
/// let mut rubric = Rubric::from_yaml(yaml).expect("Bad yaml!");
///
/// assert_eq!(rubric.name, "Test Rubric");
/// assert_eq!(rubric.len(), 2);
/// ```
pub struct Rubric {
    pub name: String,
    pub desc: Option<String>,
    pub criteria: Vec<Criterion>,
    pub total: isize
}

impl Rubric {

    /// Parses `yaml` data into a `Rubric`.
    ///
    /// This is equivilent to calling `parse()` on a string, except
    /// this will return a [`lab_grader::Error`](crate::error::ErrorKind::BadYaml)
    /// error instead of a [`serde_yaml::Error`].
    ///
    /// ## Example
    /// ```rust
    /// use lab_grader::{Rubric, yaml};
    /// let yaml = yaml!("../test_data/test_rubric.yml").expect("Couldn't load yaml");
    /// // If this is an Err, it will panic with the line/col of the yaml err
    /// let mut rubric = Rubric::from_yaml(yaml).expect("Bad yaml!");
    ///
    /// assert_eq!(rubric.name, "Test Rubric");
    /// assert_eq!(rubric.criteria().len(), 2);
    /// ```
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        match yaml.parse::<Self>() {
            Ok(rubric) => Ok(rubric),
            Err(e) => {
                match e.location() {
                    Some(loc) => return Err(Error::bad_yaml(loc.line(), loc.column())),
                    None => return Err(Error::bad_yaml(0, 0)),
                }
            }
        }
    }

    /// Searches for a criterion with the given stub,
    /// returning None if it couldn't be found
    ///
    /// ```rust
    /// # use lab_grader::{Rubric, yaml};
    /// # let yaml = yaml!("../test_data/test_rubric.yml").expect("Couldn't load yaml");
    /// # let mut rubric = Rubric::from_yaml(yaml).expect("Bad yaml!");
    /// // `rubric` contains a criterion with the stub 'first-crit`
    /// let criterion = rubric.get("first-crit");
    /// assert!(criterion.is_some());
    /// let not_criterion = rubric.get("doesnt-exist");
    /// assert!(not_criterion.is_none());
    /// ```
    pub fn get(&mut self, stub: &str) -> Option<&mut Criterion> {
        self.criteria.iter_mut().find(|c| c.stub == stub)
    }

    /// Adds a criterion to the rubric's collection.
    ///
    /// You probably shouldn't use this, instead define all
    /// your criteria in yaml.
    pub fn add(&mut self, criterion: Criterion) {
        self.criteria.push(criterion);
    }

    /// Returns the criteria as a `&mut Vec<Criterion>`, sorted
    /// by the index provided.
    ///
    /// Criteria with the same index/no index will not have guaranteed order.
    pub fn sorted(&mut self) -> &mut Vec<Criterion> {
        let sorted = &mut self.criteria;
        sorted.sort_by(|a, b| a.index.cmp(&b.index));
        sorted
    }

    /// The total points earned after grading.
    ///
    /// Each criterion stores a flag that determines if
    /// it passed or failed. This will give the total worth
    /// of all criteria that passed.
    ///
    /// If you run this before grading, it should return 0. If it
    /// doesn't, call me lmao.
    pub fn points(&self) -> usize {
        let mut total: usize = 0;
        for crit in &self.criteria {
            if let Some(status) = crit.status {
                if status {
                    // Only add to the total if they've graded
                    // and this criterion passed
                    total += crit.worth as usize;
                }
            }
        }
        total
    }

    /// Returns the total worth of all criteria, ie. the
    /// maximum number of points possible.
    pub fn total_points(&self) -> isize {
        let mut total: isize = 0;
        for crit in &self.criteria {
            total += crit.worth as isize;
        }
        total
    }

    /// Returns a reference to a `Vec` of the criteria. This
    /// is like [`sorted`](crate::rubric::Rubric::sorted), but
    /// they aren't sorted.
    pub fn criteria(&self) -> &Vec<Criterion> {
        &self.criteria
    }

    /// Returns the amount of criteria in the rubric
    pub fn len(&self) -> usize {
        self.criteria.len()
    }

    /// Prints the rubric name, then each criteria, only taking
    /// one line each. It's a shortened version of `println!("{}", rubric)`.
    pub fn print_short(&self) {
        println!("-- {} --", Color::White.bold().paint(&self.name));

        for crit in &self.criteria {
            crit.print_short();
        }
        println!("{}/{}", self.points(), self.total_points());
    }

    /// Prints a table with the rubric info and all the criteria to stdout
    pub fn print_table(&mut self) {
        let mut table = Table::new();

        // Rubric name and description
        table.add_row(row!["", "", b->self.name, self.desc.as_ref().unwrap_or(&String::new())]);

        // Headers
        table.add_row(row![b->"Criteria", b->"Worth", b->"Status", b->"Description"]);

        // Add each criterion as a row
        for crit in self.sorted() {
            if !crit.hide {
                let default_desc = String::new();
                let desc = crit.desc.as_ref().unwrap_or(&default_desc);
                table.add_row(row![crit.name, crit.worth, crit.colored_status_message(), desc]);
            }
        }

        // Add total to bottom of worth row
        table.add_row(row![br->"Total", br->format!("{}/{}", self.points(), self.total_points()), "", ""]);
        table.printstd();
    }

}


impl FromStr for Rubric {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // Construct RubricYaml from yaml data
        let rubric_yaml = serde_yaml::from_str::<RubricYaml>(s)?;

        // Pull out the criteria and count the total
        let mut criteria_total: isize = 0;
        let mut criteria = vec![];
        for (name, crit_yaml) in rubric_yaml.criteria {
            let crit = crit_yaml.into_criterion(name);
            criteria_total += crit.worth as isize;
            criteria.push(crit);
        }


        if let Some(t) = rubric_yaml.total {
            if criteria_total != t {
                eprint!("{}", Color::Red.paint("Warning: "));
                eprintln!("Rubric total does not match criteria total: rubric = {}, criteria = {}",
                    t, criteria_total);
            }
        }


        // Construct a rubric
        Ok(Rubric {
            name: rubric_yaml.name,
            desc: rubric_yaml.desc,
            criteria: criteria,
            total: criteria_total
        })
    }
}

impl fmt::Display for Rubric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n-- {} --", Color::White.bold().paint(&self.name)).unwrap();
        if let Some(desc) = &self.desc {
            writeln!(f, "{}\n", desc).unwrap();
        }
        for crit in &self.criteria {
            writeln!(f, "{}", crit).unwrap();
        }
        write!(f, "")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{yaml, TestData};

    fn yaml_data() -> &'static str {
        yaml!("../test_data/test_rubric.yml").unwrap()
    }

    #[test]
    fn test_from_yaml() {
        let rubric = Rubric::from_yaml(yaml_data()).expect("Bad yaml");
        assert_eq!(rubric.name, "Test Rubric");
        assert!(rubric.desc.is_some());
    }

    #[test]
    fn test_attach_macro() {
        fn test_fn(_: &TestData) -> bool { true };

        let mut rubric = Rubric::from_yaml(yaml_data()).expect("Bad yaml");
        assert!(!rubric.get("first-crit").unwrap().test());

        attach! {
            rubric,
            "first-crit" => test_fn
        };

        assert!(rubric.get("first-crit").unwrap().test());

    }

    #[test]
    fn test_parse_yaml() {
        let raw = r#"
            name: Test rubric
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

        assert!(raw.parse::<Rubric>().is_ok());
    }
}
