//! A batch of criteria, with some extra metadata
use std::str::FromStr;
use std::process::exit;

use crate::Criteria;
use crate::yaml::BatchYaml;


/// A bundle of metadata with a set of criteria
///
/// The main purpose of a `Batch` is to deserialize criteria from a yaml file. This
/// struct provides a `from_yaml` method that takes yaml data and turns it into a batch.
pub struct Batch {
    pub name: String,
    pub desc: Option<String>,
    pub criteria: Criteria,
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

impl Batch {
    /// Builds a Batch from yaml data
    ///
    /// Can't use serde to derive this because it contains criterion, which cannot be deserialized
    pub fn from_yaml(yaml: &str) -> Self {
        match yaml.parse::<Self>() {
            Ok(b) => return b,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
}
