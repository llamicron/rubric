# Writing the Tests
Now comes the most important part of writing the application. We've build our batch of criteria, but they currently don't have a way to be tested. How do we know if they actually have Git installed or not?

The way we determine this is by writing a "test", which is just a function, for each of our criteria. Then we'll "attach" the function to the criteria, then we can grade the submission.

## A Single Test
Each test needs to have the same signature, meaning it has to accept the same parameters and return the same thing. We need this consistency between tests to make grading possible.

Every test must accept a reference to a `TestData` object, and return a boolean. We created a `TestData` object with the `data!` macro when we made a submission. In fact, the exact data we put in the submission will be the data passed into each of our criteria tests. This is why we put the users Github username and repository name in the data; we'll need it inside one of our tests.

## Helpers
<!-- TODO: Add a link here -->
Before we write any tests, you should know about the [helpers]() modules. These modules are a collection of functions that do common tasks in criteria tests. They may save you some times. See the documentation linked above for more info on each module.

## The First Test
Let's write a test for our first criteria, which checks if Git is installed or not. Remember that a test is just a function with a specific signature. I'll add this outside our `main` function.

```rust ,noplaypen
fn confirm_git_installed(_: &TestData) -> bool {
    cli::Program::Git.version().is_some()
}
```

We added a function called `confirm_git_installed`. It takes in a parameter of type `&TestData`, but in this case we don't need it so we'll name the parameter `_`. In the function body, we used the `cli` helper module to get the version of Git, and returned true if it's a `Some` value (this is an `Option` type, it would be `None` if Git wasn't installed).

And that's it for the first test. There's still one step to go, but we'll do that after the other tests.


## The Rest of the Tests
I'm going to write some functions to serve as the tests for the remaining criteria. I won't explain what each one is doing in detail, but it should be pretty self explanatory. I'll put the entire `main.rs` file here.


```rust ,noplaypen
extern crate lab_grader;

use std::process::Command;
use lab_grader::*;

fn confirm_git_installed(_: &TestData) -> bool {
    cli::Program::Git.version().is_some()
}

fn confirm_git_init(_: &TestData) -> bool {
    // This is a filesystem helper that this crate provides
    // also works on directories
    helpers::fs::file_exists(".git/")
}


fn confirm_enough_commits(_: &TestData) -> bool {
    // Run the git command to list commit count
    let command = "git rev-list --all --count";
    let out = if cfg!(target_os = "windows") {
        Command::new("cmd")
                    .args(&["/C", command])
                    .output()
                    .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("failed to execute process")
    };

    // If the command returns something
    if let Ok(string) = String::from_utf8(out.stdout) {
        // And if we could parse a number from it
        if let Ok(num) = string.trim().parse::<u64>() {
            return num > 2;
        }
    }

    false
}


// We do need the data this time, so we'll name it `data`
fn confirm_repo_pushed(data: &TestData) -> bool {
    // Format the url to check
    let url = format!("https://github.com/{}/{}/", data["gh_name"], data["repo"]);
    // Another helper function
    web::site_responds(&url)
}

fn main() {
    let mut sub = Submission::from_data(data! {
        "name" => prompt!("Name: ", String),
        "id" => prompt!("ID: ", String),
        "gh_name" => prompt!("Github Username: ", String),
        "repo" => prompt!("Repo name: ", String)
    });

    let yaml = yaml!("../criteria/batch.yml").expect("Couldn't read file");
    let mut batch = Batch::from_yaml(yaml).expect("Bad yaml!");
}
```

In the `confirm_repo_pushed`, we're actually using the data attached to the submission. We can do that with bracket syntax (`data["key"]`) or through the [`get`](https://doc.rust-lang.org/beta/std/collections/struct.HashMap.html#method.get) method. Using `get` is recommended over bracket syntax.


## Attaching the Tests
Now that we have our tests, we need to attach them to the appropriate criteria. We can do that with the `attach!` macro.

```rust ,noplaypen
fn main() {
    // ...
    attach! {
        batch,
        "git-installed" => confirm_git_installed,
        "git-init" => confirm_git_init,
        "commits-present" => confirm_enough_commits,
        "repo-pushed" => confirm_repo_pushed
    };
}
```

This attached each of our functions to the criteria with the given stub. This is why we needed to specify a stub in yaml.

> Note: If you don't provide the stub in yaml, it will be created by lowercasing the name and replacing whitespace with a dash. ie. `My First Criterion => my-first-criterion`.

Now that the tests are attached, we're [ready to grade](grade.md).
