extern crate lab_grader;

use lab_grader::*;

// Note: Be sure the submission server is running.
// See the `examples/submission_server.rs` example,
// and run it with `cargo run --example submission_server`


// The test of a criterion.
// We can define it here just to keep it out of the way.
// We'll use it in a second
fn criterion_1_test(data: &TestData) -> bool {
    if let Some(v) = data.get("key") {
        return v == "value";
    }
    false
}

// Every criterion test must have the same signature
// ie. must accept the same parameter and return a bool
// If you don't need the data, just name it `_` and ignore it
fn criterion_2_test(data: &TestData) -> bool {
    !criterion_1_test(&data)
}


fn main() {
    // Build a submission, asking the user for name and ID
    let mut sub = Submission::new();

    // Add some arbitrary data
    //
    // Any data that may need to be used in one or more criterion tests
    // should be placed here. This data will be sent to the server as
    // part of the submission.
    sub.use_data(data! {
        "key" => "value"
    });

    // Criteria is just a vector of Criterions
    let mut criteria = Criteria::from(vec![
        Criterion::new("First criterion")
            .worth(10)
            .messages("Passed", "Failed")
            .test(Box::new(criterion_1_test))
            .build(),
        Criterion::new("Second Criterion")
            .worth(15)
            .messages("Passed", "Failed")
            .test(Box::new(criterion_2_test))
            .build(),
        Criterion::new("Third Criterion")
            .worth(5)
            .messages("Passed", "Failed")
            .test(Box::new(|_: &TestData| -> bool {
                true
            }))
            .build()
    ]);

    // Grade the submission against the criteria
    // This will assign a numerical grade to the submission
    sub.grade_against(&mut criteria);

    // Print the criteria if you want to report them to the student
    println!("{}", criteria);

    // Send the submission to the submission server
    //
    // You'll want to be running the server on some remote server,
    // maybe an Azure or AWS instance. Here, we're just running locally.
    let res = web::post_json("http://localhost:8080/submit", &sub);

    if res.is_ok() {
        println!("Submission accepted");
    } else {
        eprintln!("Submission not accepted, something went wrong");
    }

    // check submissions.csv after you submit
}
