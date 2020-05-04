extern crate lab_grader;

use lab_grader::*;

fn test_fn_1(data: &TestData) -> bool {
    true
}

fn test_fn_2(data: &TestData) -> bool {
    false
}

fn main() {
    let mut sub = Submission::new("luke", 1234);

    let mut batch = Batch::from_yaml(yaml!("criteria/example.yml").unwrap()).unwrap();
    if let Some(crit) = batch.criteria.get("some-unique") {
        crit.test = Box::new(test_fn_1);
    }
    if let Some(crit) = batch.criteria.get("second-crit") {
        crit.test = Box::new(test_fn_2);
    }

    println!("{:?}", batch.name);
    println!("{:?}", batch.desc);
    println!("{:?}", batch.criteria.len());

    sub.grade_against(&mut batch.criteria);
    println!("{}", batch.criteria);
}
