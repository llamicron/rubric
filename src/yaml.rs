//! YAML representations of some key structs
//!
//! Much bourbon went into the creation of this module.
use serde::Deserialize;
use std::collections::HashMap;


use crate::{Criterion, TestData};

// Idea: custom implemention of serde's deserialize so we
// don't need separate types for each struct


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



#[derive(Deserialize)]
pub struct BatchYaml {
    pub name: String,
    pub desc: Option<String>,
    pub criteria: HashMap<String, CriterionYaml>,
}


/// A yaml representation of [`Criterion`](crate::criterion::Criterion)
#[derive(Deserialize)]
pub struct CriterionYaml {
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
    // Normally I would implement FromStr but I can't because i can't attach the `name`,
    // just because of the yaml format
    pub fn into_criterion(self, name: String) -> Criterion {
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
    // use super::*;


}
