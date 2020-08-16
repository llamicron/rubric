#[macro_use] extern crate rubric;

use rubric::{Rubric, Submission, TestData, report};

fn first(_: &TestData)  -> bool { true  }
fn second(_: &TestData) -> bool { true  }
fn third(_: &TestData)  -> bool { true  }
fn fourth(_: &TestData) -> bool { false }

fn main() {
    let mut sub = Submission::new();
    let yaml_data = yaml!("rubric_display.yml").unwrap();
    let mut rubric = Rubric::from_yaml(yaml_data).unwrap();

    attach!(rubric, first, second, third, fourth);

    sub.grade_against(&mut rubric);

    // Different printing options, try them out
    // rubric.print_short();
    // rubric.print_table();
    // rubric.print_long();
    report::long(&mut rubric, &sub);
}
