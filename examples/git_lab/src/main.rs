#[macro_use] extern crate rubric;

// We'll need this to parse command line args
use std::env;

// Bring in criteria tests (functions from tests.rs)
mod tests;
use tests::*;

// Bring in the needed items from the rubric crate
use rubric::{Submission, Rubric, helpers::web, dropbox, report};


// This function will create a submission with some data
// and return it. We'll call it from main() later.
fn create_submission() -> Submission {
    // We're using the data! macro to build a HashMap of data,
    // and creating a submission containing that data.
    //
    // We can put whatever data we want in here. We'll want the student's
    // name and ID in order to identify them, and we'll need their GH username
    // and the repo name to ensure that they pushed the repo to Github.
    Submission::from_data(data! {
        "name"      => prompt!("Name: ", String),
        "id"        => prompt!("ID: ", String),
        "gh_name"   => prompt!("Github Username: ", String),
        "repo"      => prompt!("Repo name: ", String)
    })
}



// This function will load our rubric from rubrics/main.yml
fn load_rubric() -> Rubric {
    // This will read from the filesystem and embed the contents into the
    // compiled executable. This way, you won't have to distribute the .yml file.
    let yaml = yaml!("../rubrics/main.yml").expect("Couldn't load rubric!");

    Rubric::from_yaml(&yaml).expect("Bad yaml!")
}

fn main() {
    // This won't run unless we use the "open_sesame" argument.
    // You'll need to run this on a publicly available server,
    // and each students grader will submit to this web server.
    dropbox::open_with_arg("open_sesame", 8080);

    // Use the functions from above
    let mut sub = create_submission();
    let mut rubric = load_rubric();

    // Now we attach all the tests to the proper criteria
    attach!(
        rubric,
        git_init,
        git_installed,
        commits_present,
        repo_pushed
    );

    // Grade the submission against the rubric
    sub.grade_against(&mut rubric);

    // Print the rubric to show the student the results
    report::long(&mut rubric);


    // after grading, we need to submit to the dropbox
    // we can use one of the web helpers
    let url = "http://localhost:8080/submit";
    let result = web::post_json(&url, &sub);
    if result.is_ok() {
        println!("Success! Submission recorded");
    } else {
        println!("There was an error! Submission could not be sent. {}", result.unwrap_err());
    }
}
