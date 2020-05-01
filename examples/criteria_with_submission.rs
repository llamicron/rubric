extern crate lab_grader;

use std::collections::HashMap;

use lab_grader::data;
use lab_grader::criterion::Criterion;
use lab_grader::submission::Submission;


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
            Box::new(|data: &HashMap<String, String>| {
                return data["key"] == "value";
            })
        ),
        Criterion::new(
            "Second Criterion",
            15,
            ("Passed", "Failed"),
            Box::new(|data: &HashMap<String, String>| {
                return data["key"] != "value";
            })
        )
    ];

    println!("Before grading: {:?}", sub);
    sub.grade_against(&mut crits);
    for crit in crits {
        println!("{}", crit);
    }
    println!("After grading: {:?}", sub);
}
