extern crate lab_grader;

use lab_grader::*;


fn test(_: &TestData) -> bool {
    true
}


fn main() {
    let mut sub = Submission::new();

    let yaml = yaml!("github.yml").unwrap();
    let mut batch = Batch::from_yaml(yaml).unwrap();

    attach! {
        batch,
        "git-installed" => test
    }

    sub.grade_against(&mut batch);
    batch.print_table();
}
