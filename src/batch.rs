// std uses
use std::str::FromStr;
use std::fmt;

// external uses
use ansi_term::Color;

// internal uses
use crate::{Criterion, TestData};
use crate::yaml::BatchYaml;
use crate::error::{Result, Error};

#[macro_export]
macro_rules! attach {
    ( $batch:ident, $($stub:literal => $func:ident),* ) => {
        $(
            $batch.attach($stub, Box::new($func)).expect("criterion not found");
        )+
    };
}


pub struct Batch {
    pub name: String,
    pub desc: Option<String>,
    pub criteria: Vec<Criterion>,
    pub total: isize
}

impl Batch {
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

    pub fn get(&mut self, stub: &str) -> Result<&mut Criterion> {
        let crit = self.criteria.iter_mut().find(|c| c.stub == stub);
        if crit.is_some() {
            return Ok(crit.unwrap());
        }
        return Err(Error::stub_not_found(stub));
    }

    pub fn attach(&mut self, stub: &str,
        func: Box<dyn Fn(&TestData) -> bool>) -> Result<()> {

        let crit = self.get(stub)?;
        crit.attach(func);
        Ok(())
    }

    pub fn sorted(&mut self) -> &mut Vec<Criterion> {
        let sorted = &mut self.criteria;
        sorted.sort_by(|a, b| a.index.cmp(&b.index));
        sorted
    }

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

    pub fn total_points(&self) -> isize {
        let mut total: isize = 0;
        for crit in &self.criteria {
            total += crit.worth as isize;
        }
        total
    }

    pub fn print_short(&self) {
        println!("{}", Color::White.bold().paint(&self.name));
        for crit in &self.criteria {
            crit.print_short();
        }
        println!("{}/{}", self.points(), self.total_points());
    }
}


impl FromStr for Batch {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // Construct BatchYaml from yaml data
        let batch_yaml = serde_yaml::from_str::<BatchYaml>(s)?;

        // Pull out the criteria and count the total
        let mut criteria_total: isize = 0;
        let mut criteria = vec![];
        for (name, crit_yaml) in batch_yaml.criteria {
            let crit = crit_yaml.into_criterion(name);
            criteria_total += crit.worth as isize;
            criteria.push(crit);
        }


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
        for crit in &self.criteria {
            writeln!(f, "{}", crit).unwrap();
        }
        write!(f, "")
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
        assert!(!batch.get("first-crit").unwrap().test());

        attach! {
            batch,
            "first-crit" => test_fn
        };

        assert!(batch.get("first-crit").unwrap().test());

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
