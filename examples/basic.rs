extern crate lab_grader;

use lab_grader::submission::Submission;

fn main() {
    let sub = Submission::from_cli();
    println!("{:?}", sub);
}
