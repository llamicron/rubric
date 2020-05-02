use rocket::Config;
use rocket::http::Status;
use rocket::config::Environment;
use rocket_contrib::json::Json;

use crate::submission::Submission;
use crate::ResultsFile;


/// Just a test route so you can make sure the server is running
#[get("/")]
fn return_ok() -> Status {
    Status::Ok
}

/// Accepts a submission and writes it to the results file
#[post("/submit", format = "application/json", data = "<submission>")]
fn accept_submission(submission: Json<Submission>) -> Status {
    let sub = submission.into_inner();
    let mut rf = ResultsFile::for_item(&sub).expect("Could not open results file");

    if rf.write_csv(&sub).is_ok() {
        return Status::Accepted;
    } else {
        eprintln!("Error: Could not write following submission");
        eprintln!("{:#?}", sub);
        return Status::InternalServerError;
    }
}

/// Spins up a submission server on the given port.
///
/// You probably shouldn't call this directly, instead call
/// the `server` method on [`Submission`](crate::submission::Submission).
pub fn run(port: u16) {
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
        .expect("Could not build submission server");
    rocket::custom(config).mount("/", routes![return_ok, accept_submission]).launch();
}


#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::Client;
    use rocket::http::Header;

    fn client() -> Client {
        let rocket = rocket::ignite().mount("/", routes![return_ok, accept_submission]);
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
        let sub = Submission::new("test", 7777);
        let req = client.post("/submit")
            .body(serde_json::to_string(&sub).unwrap())
            .header(Header::new("Content-Type", "application/json"))
            .dispatch();

        assert_eq!(req.status(), Status::Accepted);
    }
}
