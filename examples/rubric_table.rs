extern crate lab_grader;

use lab_grader::*;


fn test(_: &TestData) -> bool {
    true
}


fn main() {
    let mut sub = Submission::new();

    let yaml = yaml!("github.yml").unwrap();
    let mut rubric = Rubric::from_yaml(yaml).unwrap();

    attach! {
        rubric,
        "git-installed" => test
    }

    sub.grade_against(&mut rubric);
    rubric.print_table();
}
