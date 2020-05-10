extern crate lab_grader;

use lab_grader::*;

fn main() {
    let mut c = Criterion::new("Single Criterion")
        .worth(15)
        .messages("passed", "failed")
        .test(Box::new(|_: &TestData| -> bool {
            true
        }))
        .build();

    println!("Individual criterion:\n");
    println!("{}", c);
    c.status = Some(true);
    println!("{}", c);
    c.status = Some(false);
    println!("{}", c);

    let mut criteria = Criteria::from(vec![
        Criterion::new("First criterion")
            .worth(10)
            .messages("Passed", "Failed")
            .test(Box::new(|_: &TestData| true ))
            .build(),
        Criterion::new("Second Criterion")
            .worth(15)
            .messages("Passed", "Failed")
            .test(Box::new(|_: &TestData| true ))
            .build(),
        Criterion::new("Third Criterion")
            .worth(5)
            .messages("Passed", "Failed")
            .test(Box::new(|_: &TestData| false ))
            .build()
    ]);


    let mut sub = Submission::new();
    sub.grade_against(&mut criteria);
    println!("{}", criteria);
}
