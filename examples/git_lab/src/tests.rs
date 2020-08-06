use std::process::Command;
use rubric::{TestData, helpers::{fs, system, web}};

// Each of these functions corresponds to a criterion.
// The if function returns true, the criterion is passed.
// 
// Notice that every function *must* have the same signature, ie.
//      
//      fn foo(data: &TestData) -> bool
//
// The TestData here is the same data we defined when we built
// the Submission in `main.rs`.
//
// We're using a few helpers functions from the `helpers` modules


pub fn git_init(_: &TestData) -> bool {
    fs::file_exists(".git/")
}

pub fn git_installed(_: &TestData) -> bool {
    system::Program::Git.version().is_some()
}

pub fn commits_present(_: &TestData) -> bool {
    let output = Command::new("cmd")
        .args(&["/C", "git --version"])
        .output()
        .expect("Failed to execute process");
    output.stdout.len() > 0
}

pub fn repo_pushed(data: &TestData) -> bool {
    let url = format!("https://github.com/{}/{}/", data["gh_name"], data["repo"]);
    web::site_responds(&url)
}
