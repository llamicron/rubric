extern crate lab_grader;

use lab_grader::*;

fn main() {
    let mut c = Criterion::new(
        "My First Criterion",
        15,
        ("passed", "failed"),
        Box::new(|_: &TestData| -> bool {
            true
        })
    );

    println!("Individual criterion:\n");
    println!("{}", c);
    c.status = Some(true);
    println!("{}", c);
    c.status = Some(false);
    println!("{}", c);

    let mut criteria = Criteria::from(vec![
        Criterion::new(
            "First of three",
            15,
            ("passed", "failed"),
            Box::new(|_: &TestData| -> bool {
                true
            })
        ),
        Criterion::new(
            "Second of three",
            25,
            ("passed", "failed"),
            Box::new(|_: &TestData| -> bool {
                false
            })
        ),
        Criterion::new(
            "Third of three with a longer name",
            25,
            ("passed", "failed"),
            Box::new(|_: &TestData| -> bool {
                true
            })
        ),
    ]);


    let mut sub = Submission::new("luke", 1234);
    sub.grade_against(&mut criteria);
    println!("{}", criteria);
}
