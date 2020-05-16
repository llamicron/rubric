extern crate lab_grader;

use lab_grader::*;

fn main() {
    let sub = Submission::from_data(data! {
        "key" => "value with, a comma"
    });

    println!("{}", sub.header());
    println!("{}", sub.as_csv());
}
