extern crate rubric;

use std::env;
use rubric::*;


fn main() {
    // Get command line args
    let args: Vec<String> = env::args().collect();

    // If you run this with the "server" arg, run the server
    if args.len() == 2 && args[1] == "server" {
        Submission::server(8080);
    }

    // Enforces they enter a u32, then covert to a String
    // Submission data must be a String
    let id = format!("{}", prompt!("ID: ", u32));

    // Make a submission
    // This is what will be sent to the submission server
    let mut sub = Submission::from_data(data! {
        "id" => id,
        "name" => prompt!("Name: ", String)
    });


    let yaml_data = yaml!("submission_server.yml").expect("Couldn't read yaml file");
    let mut rubric = Rubric::from_yaml(yaml_data).expect("Bad yaml");

    // We're not going to attach a test to the criteria,
    // meaning they'll all fail. We don't really care,
    // this is just to demonstrate sending the submission.
    println!("About to grade");
    sub.grade_against(&mut rubric);
    println!("{}", rubric);


    // Url of the submission server
    let url = "http://localhost:8080";
    // Be sure you have the server running before
    // grading and submitting
    if web::post_json(url, &sub).is_ok() {
        println!("Submission sent successfully!");
    } else {
        eprintln!("Error sending submission!");
    }
}
