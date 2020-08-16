#[macro_use] extern crate rubric;

use rubric::{Rubric, Submission, dropbox};

mod tests {
    use std::process::Command;
    use rubric::helpers::{fs, system, web};
    use rubric::TestData;

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
}


fn main() {
    dropbox::open_with_arg("open_sesame", 8080);

    let yaml_data = yaml!("minimal_git_lab.yml").unwrap();
    let mut rubric = Rubric::from_yaml(&yaml_data).unwrap();

    attach!(
        rubric,
        tests::git_init,
        tests::git_installed,
        tests::commits_present,
        tests::repo_pushed
    );

    // let mut sub = Submission::from_data(data! {
    //     "name" => prompt!("Name: ", String),
    //     "id" => prompt!("ID: ", String),
    //     "gh_name" => prompt!("Github Username: ", String),
    //     "repo" => prompt!("Repo name: ", String),
    //     "custom_data" => "my super secret data"
    // });
    let mut sub = Submission::from_data(data! {
        "name" => "Luke",
        "id" => "1001764631",
        "gh_name" => "llamicron",
        "repo" => "rubric",
        "custom_data" => "my super secret data"
    });

    sub.set_fingerprint("my secret key shhh don't tell anyone");

    sub.grade_against(&mut rubric);
    rubric.print_long();

    let url = format!("http://localhost:8080/submit");
    if sub.submit(&url).is_ok() {
        println!("Submitted!");
    } else {
        println!("Error! Couldn't submit");
    }
}
