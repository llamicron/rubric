use std::process::Command;
use rubric::{TestData, helpers::{fs, system, web}};

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
