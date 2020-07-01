extern crate lab_grader;

use lab_grader::*;
// Bring all the test functions into scope
use tests::*;

// The example in the guide uses an external module
// this keeps it in the same file so it will work as an example
// mod tests;

mod tests {
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

}


fn main() {
    let mut sub = Submission::from_data(data! {
        "name"    => prompt!("Name: ", String),
        "id"      => prompt!("ID: ", String),
        "gh_name" => prompt!("Github username: ", String),
        "repo"    => prompt!("Repository name: ", String)
    });



    let yaml = yaml!("github.yml").expect("Couldn't get yaml from file");
    let mut rubric = match Rubric::from_yaml(yaml) {
        Ok(b) => b,
        Err(e) => panic!(format!("{}", e))
    };


    attach! {
        rubric,
        "git-installed" => confirm_git_installed,
        "git-init" => confirm_git_init,
        "commits" => confirm_enough_commits,
        "pushed" => confirm_repo_pushed
    }

    sub.grade_against(&mut rubric);
    // println!("{}", rubric);
    rubric.print_short();

    let url = "https://postman-echo.com/post";
    match web::post_json(url, &sub) {
        Ok(resp) => println!("Submission sent and recieved! Status {}", resp.status()),
        Err(_) => println!("Submission could not be sent.")
    }
}
