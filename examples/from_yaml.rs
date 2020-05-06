extern crate lab_grader;

use lab_grader::*;

fn main() {
    let sub = Submission::new("luke", 1234);

    let yaml_data = yaml!("criteria/example.yml").expect("Couldn't read that file!");
    let batch = Batch::from_yaml(yaml_data);

    println!("{}", batch.name);
    println!("{:?}", batch.desc);
}
