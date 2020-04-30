extern crate lab_grader;

use std::collections::HashMap;
use lab_grader::criterion::Criterion;

// Write out criterion tests as individual functions so
// we don't have to write them inline.
// This helps a lot with code organization
fn criterion_1_test(_: &HashMap<String, String>) -> bool {
    true
}

fn criterion_2_test(data: &HashMap<String, String>) -> bool {
    return data["key"] == "value"
}

fn main() {

    // Establish criteria
    let mut crits = vec![
        // just pass in the appropriate function inside a Box
        Criterion::new(
            "criterion 1",
            10,
            ("passed", "failed"),
            Box::new(criterion_1_test)
        ),
        Criterion::new(
            "criterion 2",
            25,
            ("passed", "failed"),
            Box::new(criterion_2_test)
        ),
    ];

    // Prepare data
    let mut data = HashMap::new();
    data.insert("key".to_string(), "value".to_string());


    for crit in &mut crits {
        // We don't know which ones need data
        // so we just pass the data to all of them
        // The onces that don't need it won't use it
        crit.test_with_data(&data);
        println!("{}", crit);
    }
}
