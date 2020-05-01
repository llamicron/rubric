extern crate lab_grader;

use std::collections::HashMap;

use lab_grader::data;
use lab_grader::criterion::Criterion;
use lab_grader::submission::Submission;

// The actual function of a criterion.
// We can define it here just to keep it out of the way.
// In a second we'll put it in a box
fn criterion_1_test(data: &HashMap<String, String>) -> bool {
    if let Some(v) = data.get("key") {
        return v == "value";
    } else {
        return false;
    }
}

// Just returns the opposite of the first test
fn criterion_2_test(data: &HashMap<String, String>) -> bool {
    !criterion_1_test(&data)
}


fn main() {
    // Collect data from students
    let mut sub = Submission::new("Luke", 1234);
    sub.use_data(data! {
        "key" => "value"
    });

    let mut crits: Vec<Criterion> = vec![
        Criterion::new(
            "First Criterion",
            10,
            ("Passed", "Failed"),
            Box::new(criterion_1_test)
        ),
        Criterion::new(
            "Second Criterion",
            15,
            ("Passed", "Failed"),
            Box::new(criterion_2_test)
        )
    ];

    sub.grade_against(&mut crits);
    for crit in crits {
        println!("{}", crit);
    }
}
