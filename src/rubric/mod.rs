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
//!     func: first_crit
//!     index: 0
//!     desc: "First criterion short desc"
//!     worth: 50
//!     messages: ["success", "failure"]
//!     hide: false
//!
//!   Second Criterion:
//!     func: second_crit
//!     worth: 30
//! ```
//! See the [YAML specification](https://github.com/llamicron/rubric/wiki/YAML-Specification) for more info.

// Re exports to be available from this module
pub mod criterion;
pub mod criterion_builder;

pub use criterion::Criterion;
pub use criterion_builder::CriterionBuilder;


// std uses
use std::str::FromStr;
use std::default::Default;

// external uses
use chrono::{DateTime, Local};
use anyhow::Context;
use paris::Logger;

// internal uses
use crate::{Result, yaml::RubricYaml};



/// A collection of criteria, meant to be serialized from `yaml`.
///
/// ## Example
/// ```rust
/// use rubric::{Rubric, yaml};
///
/// // Relative path to the yaml file
/// let yaml = yaml!("../../test_data/test_rubric.yml").expect("Couldn't load yaml");
/// let mut rubric = Rubric::from_yaml(yaml).expect("Bad yaml!");
///
/// assert_eq!(rubric.name, "Test Rubric");
/// assert_eq!(rubric.len(), 2);
/// ```
pub struct Rubric {
    pub name: String,
    pub desc: Option<String>,
    pub criteria: Vec<Criterion>,
    pub total: isize,
    pub deadline: Option<DateTime<Local>>,
    pub final_deadline: Option<DateTime<Local>>,
    pub allow_late: bool,
    pub late_penalty: isize,
    pub daily_penalty: isize
}

impl Default for Rubric {
    /// This is only used for testing and examples.
    /// You shouldn't use this to create a new Rubric, instead
    /// use `from_yaml()`.
    fn default() -> Rubric {
        Rubric {
            name: String::new(),
            desc: None,
            criteria: Vec::new(),
            total: 0,
            deadline: None,
            final_deadline: None,
            allow_late: true,
            late_penalty: 0,
            daily_penalty: 0
        }
    }
}

impl Rubric {

    /// Parses `yaml` data into a `Rubric`.
    ///
    /// This is equivilent to calling `parse()` on a string, except
    /// this will return a [`rubric::Error`](crate::error::ErrorKind::BadYaml)
    /// error instead of a [`serde_yaml::Error`].
    ///
    /// ## Example
    /// ```rust
    /// use rubric::{Rubric, yaml};
    /// let yaml = yaml!("../../test_data/test_rubric.yml").expect("Couldn't load yaml");
    /// // If this is an Err, it will panic with the line/col of the yaml err
    /// let mut rubric = Rubric::from_yaml(yaml).expect("Bad yaml!");
    ///
    /// assert_eq!(rubric.name, "Test Rubric");
    /// assert_eq!(rubric.criteria().len(), 2);
    /// ```
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        yaml.parse::<Self>().context("Couldn't parse YAML into rubric")
    }

    /// Searches for a criterion with the given func,
    /// returning None if it couldn't be found
    ///
    /// ```rust
    /// # use rubric::{Rubric, yaml};
    /// # let yaml = yaml!("../../test_data/test_rubric.yml").expect("Couldn't load yaml");
    /// # let mut rubric = Rubric::from_yaml(yaml).expect("Bad yaml!");
    /// // `rubric` contains a criterion with the func 'first_crit`
    /// let criterion = rubric.get("first_crit");
    /// assert!(criterion.is_some());
    /// let not_criterion = rubric.get("doesnt-exist");
    /// assert!(not_criterion.is_none());
    /// ```
    pub fn get(&mut self, func: &str) -> Option<&mut Criterion> {
        self.criteria.iter_mut().find(|c| c.func == func)
    }

    /// Adds a criterion to the rubric.
    ///
    /// You probably shouldn't use this, instead define all
    /// your criteria in yaml.
    pub fn add(&mut self, criterion: Criterion) {
        self.criteria.push(criterion);
    }

    /// Returns the criteria as a `&mut Vec<Criterion>`, sorted
    /// by the criterion's index.
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

    pub fn past_due(&self) -> bool {
        if let Some(deadline) = self.deadline {
            return deadline.timestamp() < Local::now().timestamp();
        }
        false
    }

    pub fn past_final_deadline(&self) -> bool {
        if let Some(final_deadline) = self.final_deadline {
            return final_deadline.timestamp() < Local::now().timestamp();
        }
        false
    }

}


impl FromStr for Rubric {
    type Err = anyhow::Error;


    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // Construct RubricYaml from yaml data
        // See yaml.rs
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
                let mut log = Logger::new();
                log.error(format!(
                    "Warning: Rubric total does not match criteria total: rubric = {}, criteria = {}",
                    t,
                    criteria_total
                ));
            }
        }



        // Parse deadline, if any
        let mut deadline: Option<DateTime<Local>> = None;
        if let Some(deadline_str) = rubric_yaml.deadline {
            // Add the local timezone to the end so they don't have to specify
            let added_timezone = format!("{} {}", deadline_str, Local::now().format("%z"));
            // Parse what they entered + timezone into a DateTime
            let parsed_deadline = DateTime::parse_from_str(&added_timezone, "%F %T %z").expect("Bad time format");
            // Convert from DateTime<FixedOffset> to DateTime<Local>
            deadline = Some(DateTime::from(parsed_deadline));
        }

        // Parse final deadline, if any
        let mut final_deadline: Option<DateTime<Local>> = None;
        if let Some(final_deadline_str) = rubric_yaml.final_deadline {
            // Add the local timezone to the end so they don't have to specify
            let added_timezone = format!("{} {}", final_deadline_str, Local::now().format("%z"));
            // Parse what they entered + timezone into a DateTime
            let parsed_deadline = DateTime::parse_from_str(&added_timezone, "%F %T %z").expect("Bad time format");
            // Convert from DateTime<FixedOffset> to DateTime<Local>
            final_deadline = Some(DateTime::from(parsed_deadline));
        }

        // Construct a rubric
        Ok(Rubric {
            name: rubric_yaml.name,
            desc: rubric_yaml.desc,
            criteria: criteria,
            total: criteria_total,
            deadline: deadline,
            final_deadline: final_deadline,
            allow_late: rubric_yaml.allow_late.unwrap_or(true),
            late_penalty: rubric_yaml.late_penalty.unwrap_or(0),
            daily_penalty: rubric_yaml.late_penalty_per_day.unwrap_or(0)
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{yaml, attach, TestData};

    fn yaml_data() -> &'static str {
        yaml!("../../test_data/test_rubric.yml").unwrap()
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
        assert!(!rubric.get("first_crit").unwrap().test());

        attach! {
            rubric,
            "first_crit" => test_fn
        };

        assert!(rubric.get("first_crit").unwrap().test());
    }

    #[test]
    fn test_parse_yaml() {
        let raw = r#"
            name: Test rubric
            desc: here's a short description
            criteria:
                First Criterion:
                    func: first_crit,
                    index: 0
                    desc: "First criterion short desc"
                    worth: 50
                    messages: ["success", "failure"]
                    hide: false

                Second Criterion:
                    func: second_crit
                    worth: 30
        "#;

        assert!(raw.parse::<Rubric>().is_ok());
    }

    #[test]
    fn test_rubric_past_due() {
        let ok_rubric = Rubric::from_yaml(yaml_data()).unwrap();
        assert!(!ok_rubric.past_due());

        let yaml = yaml!("../../test_data/past_due_rubric.yml").unwrap();
        let old_rubric = Rubric::from_yaml(yaml).unwrap();
        assert!(old_rubric.past_due());
    }
}
