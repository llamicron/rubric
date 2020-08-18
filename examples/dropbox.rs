// This is an example of using the dropbox,
// so the actual grader is very basic and I may cut corners

#[macro_use] extern crate rubric;

use rubric::{Rubric, Submission, TestData, dropbox};

fn only(_: &TestData) -> bool { true }


fn main() {
    // Open the dropbox if we run with the "open_sesame" arg
    dropbox::open_with_arg("open_sesame", 8080);


    // Rubric and submission stuff
    let yaml = yaml!("dropbox.yml").unwrap();
    let mut rubric = Rubric::from_yaml(&yaml).unwrap();
    attach!(rubric, only);
    let mut sub = Submission::from_data(data! {
        "key1" => "value1",
        "key2" => "value2",
        "name" => "luke"
    });
    sub.grade_against(&mut rubric);

    match sub.submit("http://localhost:8080/submit") {
        Ok(_) => println!("Submitted successfully!"),
        Err(e) => println!("Error, couldn't submit. {}", e),
    }
}
