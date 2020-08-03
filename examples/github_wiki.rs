#[macro_use] extern crate rubric;


use rubric::{Submission, Rubric};

fn create_submission() -> Submission {
    Submission::from_data(data! {
        "name" => prompt!("Name: ", String),
        "id" => prompt!("ID: ", String),
        "gh_name" => prompt!("Github Username: ", String),
        "repo" => prompt!("Repo Name: ", String)
    })
}

fn load_rubric() -> Rubric {
    let yaml = yaml!("github.yml").expect("Couldn't load file");
    Rubric::from_yaml(&yaml).expect("Bad yaml!")
}


mod tests {
    use std::process::Command;
    use rubric::{TestData, helpers::{fs, system, web}};

    pub fn confirm_git_init(_: &TestData) -> bool {
        fs::file_exists(".git/")
    }

    pub fn confirm_git_installed(_: &TestData) -> bool {
        system::Program::Git.version().is_some()
    }

    pub fn confirm_commits_present(_: &TestData) -> bool {
        let output = Command::new("cmd")
            .args(&["/C", "git --version"])
            .output()
            .expect("Failed to execute process");
        output.stdout.len() > 0
    }

    pub fn confirm_repo_pushed(data: &TestData) -> bool {
        let url = format!("https://github.com/{}/{}/", data["gh_name"], data["repo"]);
        web::site_responds(&url)
    }
}

use tests::*;
fn main() {
    let mut sub = create_submission();
    let mut rubric = load_rubric();
    
    
    attach! {
        rubric,
        "git-init" => confirm_git_init,
        "git-installed" => confirm_git_installed,
        "commits" => confirm_commits_present,
        "pushed" => confirm_repo_pushed
    };

    sub.grade_against(&mut rubric);
    println!("{}", rubric);
}
