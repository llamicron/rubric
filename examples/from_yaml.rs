extern crate lab_grader;

use lab_grader::*;

fn test1(_: &TestData) -> bool {
    true
}

fn test2(_: &TestData) -> bool {
    false
}

fn main() {
    let mut sub = Submission::new();

    let yaml_data = yaml!("criteria/example.yml").expect("Couldn't read that file!");
    let mut batch = Batch::from_yaml(yaml_data);

    batch.attach("second-crit", Box::new(test2));
    batch.attach("some-unique-stub", Box::new(test1));

    sub.grade_against(&mut batch.criteria);
    print!("{}", batch.criteria);
    web::post_json("url", sub).unwrap();
}
