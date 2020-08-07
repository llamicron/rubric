#[macro_use] extern crate rubric;

mod tests;

use rubric::{Rubric, Submission, dropbox};


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "dropbox" {
        dropbox::open(8080);
    }

    let yaml_data = yaml!("../rubrics/main.yml").unwrap();
    let mut rubric = Rubric::from_yaml(&yaml_data).unwrap();
    
    for crit in rubric.sorted() {
        println!("{}: {}", crit.name, crit.func);
    }

    attach!(
        rubric,
        tests::git_init,
        tests::git_installed,
        tests::commits_present,
        tests::repo_pushed
    );

    let mut sub = Submission::from_data(data! {
        "name" => prompt!("Name: ", String),
        "id" => prompt!("ID: ", String),
        "gh_name" => prompt!("Github Username: ", String),
        "repo" => prompt!("Repo name: ", String),
        "custom_data" => "my super secret data"
    });


    sub.grade_against(&mut rubric);
    println!("{}", rubric);

    let url = format!("http://localhost:8080/submit");
    if sub.submit(&url).is_ok() {
        println!("Submitted!");
    } else {
        println!("Error! Couldn't submit");
    }
}
