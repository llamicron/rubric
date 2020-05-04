//! Criteria built from `yaml`
use serde::Deserialize;
use std::collections::HashMap;

use crate::{TestData, Criteria, Criterion};

/// This is an important macro. It reads data from a file using
/// the include_bytes! macro. When compiling for debug, this will read
/// from the filesystem. When compiling for release, this will embed the data
/// in the executable. Graders built using this crate need to have the data embedded
/// in the executable to make it easier to distribute and to keep the data private.
#[macro_export]
macro_rules! yaml {
    ( $file:expr ) => {
        ::std::str::from_utf8(include_bytes!($file))
    };
}



/// A bundle of metadata with a set of criteria
///
/// The main purpose of a `Batch` is to deserialize criteria from a yaml file. This
/// struct provides a `from_yaml` method that takes yaml data and turns it into a batch.
pub struct Batch {
    pub name: String,
    pub desc: Option<String>,
    pub criteria: Criteria,
}

impl Batch {
    /// Builds a Batch from yaml data
    pub fn from_yaml(yaml: &str) -> Result<Batch, serde_yaml::Error> {
        // Construct BatchYaml from yaml data
        let batch_yaml = serde_yaml::from_str::<BatchYaml>(yaml)?;

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



#[derive(Deserialize)]
struct BatchYaml {
    name: String,
    desc: Option<String>,
    criteria: HashMap<String, CriterionYaml>,
}



/// A yaml representation of [`Criterion`](crate::criterion::Criterion)
#[derive(Deserialize)]
struct CriterionYaml {
    stub: String,
    #[allow(dead_code)]
    index: Option<i64>,
    #[allow(dead_code)]
    desc: String,
    worth: i16,
    messages: (String, String),
    hide: Option<bool>
}

impl CriterionYaml {
    fn into_criterion(self, name: String) -> Criterion {
        let mut c = Criterion::new(
            name,
            self.worth,
            self.messages,
            Box::new(|_: &TestData| false)
        );
        c.stub = self.stub;

        if let Some(h) = self.hide {
            c.hide = h;
        }

        c
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_batch() {
        let yaml_data = yaml!("../criteria/test_batch.yml").unwrap();
        let res = Batch::from_yaml(yaml_data);
        assert!(res.is_ok());
        match res {
            Ok(batch) => {
                assert_eq!(batch.name, "Test batch");
                assert!(batch.criteria.len() == 2);
            },
            Err(_) => assert!(false)
        }
    }
}
