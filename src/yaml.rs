//! YAML representations of some key structs
//!
//! Much bourbon went into the creation of this module.
use serde::Deserialize;
use std::collections::HashMap;
// use std::process::exit;


use crate::{Criterion, TestData};


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

/// A yaml representation of a [`Batch`](crate::batch::Batch).
///
/// This struct is just used for deserializing yaml. [`Batch::from_str`](crate::batch::Batch::from_str)
/// uses one of these puppies for deserializing then consumes it to build a Batch.
#[derive(Deserialize)]
pub struct BatchYaml {
    pub name: String,
    pub desc: Option<String>,
    pub criteria: HashMap<String, CriterionYaml>,
}


/// A yaml representation of [`Criterion`](crate::criterion::Criterion)
///
/// This can be deserialized from valid yaml, then converted into a
/// Criterion with [`into_criterion`](crate::yaml::CriterionYaml::into_criterion).
#[derive(Deserialize)]
pub struct CriterionYaml {
    stub: String,
    #[allow(dead_code)]
    index: Option<i64>,
    desc: Option<String>,
    worth: i16,
    messages: Option<(String, String)>,
    hide: Option<bool>
}

impl CriterionYaml {
    // Normally I would implement FromStr but I can't because I can't attach the `name`,
    // just because of the yaml format. Kinda fucky, I know.
    pub fn into_criterion(self, name: String) -> Criterion {
        let mut msgs = (String::from("passed"), String::from("failed"));

        if self.messages.is_some() {
            msgs = self.messages.unwrap();
        }


        let mut c = Criterion::new(
            name,
            self.worth,
            msgs,
            Box::new(|_: &TestData| false )
        );
        c.stub = self.stub;

        if let Some(h) = self.hide {
            c.hide = h;
        }

        if let Some(desc) = self.desc {
            c.set_desc(desc);
        }

        return c;
    }
}
