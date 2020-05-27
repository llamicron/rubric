use std::process::Command;
use lab_grader::*;

// Naming the data parameter "_" because we don't need it in this case
pub fn confirm_git_installed(_: &TestData) -> bool {
    cli::Program::Git.version().is_some()
}

pub fn confirm_git_init(_: &TestData) -> bool {
    // This is a filesystem helper that this crate provides
    // also works on directories
    // This is *not* std::fs
    fs::file_exists(".git/")
}

pub fn confirm_enough_commits(_: &TestData) -> bool {
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
pub fn confirm_repo_pushed(data: &TestData) -> bool {
    // Format the url to check
    let url = format!("https://github.com/{}/{}/", data["gh_name"], data["repo"]);
    // Another helper function
    web::site_responds(&url)
}
