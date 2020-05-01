use rocket::Config;
use rocket::http::Status;
use rocket::config::Environment;
use rocket_contrib::json::Json;

use crate::submission::{Submission, CSV_HEADER};
use crate::ResultsFile;


/// Accepts a submission and writes it to the results file
#[post("/submit", format = "application/json", data = "<submission>")]
pub fn accept_submission(submission: Json<Submission>) -> Status {
    let mut rf = ResultsFile::new("./results.csv", CSV_HEADER).expect("Could not open results file");
    let sub = submission.into_inner();

    if rf.write_csv(&sub).is_ok() {
        return Status::new(202, "accepted");
    } else {
        eprintln!("Error: Could not write following submission");
        eprintln!("{:#?}", sub);
        return Status::new(202, "accepted");
    }
}

/// Spins up a submission server on the given port.
///
/// You probably shouldn't call this directly, instead call
/// the `server` method on [`Submission`](crate::submission::Submission).
pub fn run(port: u16) {
    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(port)
        .finalize()
        .expect("Could not build submission server");
    rocket::custom(config).mount("/", routes![accept_submission]).launch();
}
