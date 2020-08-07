//! A dropbox to collect submissions
//! 
//! This is a webserver that accepts Submissions in JSON format
//! and writes them to a CSV file.
//! 
//! You should run this on a publicly available server and be sure 
//! the correct ports are open. You can run this on whatever port you'd like,
//! as long as you have permission. The `/submit` route is meant to accept a Submission.
//! See the [`helpers::web::post_json`](helpers::web::post_json) function for more info on
//! how to send a submission to the dropbox.

// Re exports to be available from this module
pub mod results_file;
pub mod submission;
pub mod fingerprint;

pub use results_file::{AsCsv, ResultsFile};
pub use submission::{Submission, TestData};


// std uses
use std::env;
use std::sync::Mutex;

// external uses
use rocket::{Rocket, Config, State};
use rocket::http::Status;
use rocket::error::LaunchError;
use rocket::config::Environment;
use rocket_contrib::json::Json;


struct SharedResultsFile { results_file: Mutex<ResultsFile> }


/// Just a test route so you can make sure the server is running
#[get("/")]
fn return_ok() -> Status {
    Status::Ok
}

/// Accepts a submission and writes it to the results file
#[post("/submit", format = "application/json", data = "<submission>")]
fn accept_submission(state: State<SharedResultsFile>, submission: Json<Submission>) -> Status {
    // Retrieve the submission
    let sub = submission.into_inner();

    // Lock the results file until we're done with it
    let shared_rf: &SharedResultsFile = state.inner();
    let mut lock = shared_rf.results_file.lock().expect("Lock shared results file");

    // Write the header based on first submission
    if lock.length() == 0 {
        if lock.append(&sub.header()).is_err() {
            eprintln!("Error! Could not write csv file header. File is likely locked by another process");
            return Status::InternalServerError;
        };
    }

    if lock.write_csv(&sub).is_ok() {
        return Status::Accepted;
    } else {
        eprintln!("Error: Could not write following submission");
        eprintln!("{:#?}", sub);
        return Status::InternalServerError;
    }
}

/// Builds a rocket instance to launch
fn new_rocket(port: u16) -> Rocket {
    // If debug
    #[cfg(debug_assertions)]
    let builder = Config::build(Environment::Development);
    // If production
    #[cfg(not(debug_assertions))]
    let builder = Config::build(Environment::Production);

    let config = builder
        .address("0.0.0.0")
        .port(port)
        .finalize()
        .expect("Could not build dropbox server");

    // Create a results file wrapped in a Mutex
    // The muted is necessary because route handling is asyncronous.
    // This allows mutliple submissions to be submitted at once. It also will
    // crash when the instructor opens the dropbox if the file is already in use.
    let shared_results_file = SharedResultsFile {
        results_file: Mutex::new(
            ResultsFile::new_blank("submissions.csv").expect("Couldn't open results file")
        )
    };

    println!("Dropbox is open! accepting POST requests to /submit");
    return rocket::custom(config)
        .manage(shared_results_file)
        .mount("/", routes![return_ok, accept_submission]);
}

/// Opens the dropbox for submissions on the given port.
/// 
/// You should probably use [`open_with_arg()`](crate::dropbox::open_with_arg)
pub fn open(port: u16) -> LaunchError {
    new_rocket(port).launch()
}

/// This is the same as [`open()`](crate::dropbox::open), but it will
/// only open the dropbox if you run the executable with the arg you provide.
/// 
/// The dropbox will open if the provided arg is *anywhere* in the arg vector, ie.
/// position doesn't matter. 
/// 
/// It's probably a good idea to put something that isn't obvious as the arg, perhaps
/// a password. That way, no one will accidentally (or malicously) opens the dropbox.
/// 
/// ```no_compile
/// // Must run the execuable with `my_grader open_sesame`
/// dropbox::open_with_arg(8080, "open_sesame");
/// ```
pub fn open_with_arg(arg: &str, port: u16) -> Option<LaunchError> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from(arg)) {
        return Some(open(port));
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::Client;
    use rocket::http::Header;

    fn client() -> Client {
        let rocket = new_rocket(8080);
        Client::new(rocket).expect("valid rocket instance")
    }

    #[test]
    fn test_server_runs() {
        let client = client();
        let resp = client.get("/").dispatch();

        assert_eq!(resp.status(), Status::Ok);
    }

    #[test]
    fn test_404() {
        let client = client();
        let resp = client.get("/not_a_route").dispatch();

        assert_eq!(resp.status(), Status::NotFound);
    }

    #[test]
    fn test_422_unprocessable_entity() {
        let client = client();
        let resp = client.post("/submit")
            // This route only accepts Submissions
            // This is not a submission
            .body(r#"{"key":"value"}"#)
            // This should be set automatically by body but it doesn't work lmao
            .header(Header::new("Content-Type", "application/json"))
            .dispatch();

        assert_eq!(resp.status(), Status::UnprocessableEntity)
    }

    #[test]
    fn test_accept_submission() {
        let client = client();
        let sub = Submission::new();
        let req = client.post("/submit")
            .body(serde_json::to_string(&sub).unwrap())
            .header(Header::new("Content-Type", "application/json"))
            .dispatch();

        assert_eq!(req.status(), Status::Accepted);
    }
}
