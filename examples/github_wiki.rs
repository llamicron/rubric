extern crate lab_grader;

use std::process::Command;
use lab_grader::*;


// Naming the data parameter "_" because we don't need it in this case
fn confirm_git_installed(_: &TestData) -> bool {
    cli::Program::Git.version().is_some()
}

fn confirm_git_init(_: &TestData) -> bool {
    // This is a filesystem helper that this crate provides
    // also works on directories
    // This is *not* std::fs
    fs::file_exists(".git/")
}

fn confirm_enough_commits(_: &TestData) -> bool {
    // Run the git command to list commit count
    let out = Command::new("sh")
        .arg("-c")
        .arg("git rev-list --all --count")
        .output()
        .expect("Couldn't run subcommand");

    // If the command returns something
    if let Ok(string) = String::from_utf8(out.stdout) {
        // And if we could parse a number from it
        if let Ok(num) = string.trim().parse::<u64>() {
            return num > 2;
        }
    }

    false
}

// We do need the data this time
fn confirm_repo_pushed(data: &TestData) -> bool {
    // Format the url to check
    let url = format!("https://github.com/{}/{}/", data["gh_name"], data["repo"]);
    // Another helper function
    web::site_responds(&url)
}

fn main() {
    let mut sub = Submission::from_data(data! {
        "name"    => prompt!("Name: ", String),
        "id"      => prompt!("ID: ", String),
        "gh_name" => prompt!("Github username: ", String),
        "repo"    => prompt!("Repository name: ", String)
    });


    let mut batch = match yaml!("criteria/github.yml") {
        Ok(text) => Batch::from_yaml(text),
        Err(_) => panic!("Something went wrong! Couldn't read yaml data and build a batch")
    };


    batch.attach("git-installed", Box::new(confirm_git_installed));
    batch.attach("git-init", Box::new(confirm_git_init));
    batch.attach("commits", Box::new(confirm_enough_commits));
    batch.attach("pushed", Box::new(confirm_repo_pushed));

    sub.grade_against(&mut batch.criteria);
    println!("{}", batch);

    // let url = format!("http://myurl.whatever/submit");
    // if web::post_json(&url, &sub).is_ok() {
    //     println!("Submission sent and recieved!");
    // } else {
    //     println!("Submission could not be sent.");
    // }
}
