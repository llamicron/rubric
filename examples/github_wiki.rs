extern crate lab_grader;

use lab_grader::*;

mod github_tests;
use github_tests::*;


fn main() {
    let mut sub = Submission::from_data(data! {
        "name"    => prompt!("Name: ", String),
        "id"      => prompt!("ID: ", String),
        "gh_name" => prompt!("Github username: ", String),
        "repo"    => prompt!("Repository name: ", String)
    });



    let yaml = yaml!("github.yml").expect("Couldn't get yaml from file");
    let mut batch = match Batch::from_yaml(yaml) {
        Ok(b) => b,
        Err(e) => panic!(format!("{}", e))
    };


    attach! {
        batch,
        "git-installed" => confirm_git_installed,
        "git-init" => confirm_git_init,
        "commits" => confirm_enough_commits,
        "pushed" => confirm_repo_pushed
    }

    sub.grade_against(&mut batch);
    println!("{}", batch);

    // let url = "http://myurl.whatever/submit";
    // if web::post_json(url, &sub).is_ok() {
    //     println!("Submission sent and recieved!");
    // } else {
    //     println!("Submission could not be sent.");
    // }
}
