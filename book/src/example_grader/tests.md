# Writing the Tests
Now comes the most important part of writing the application. We've build our rubric of criteria, but they currently don't have a way to be tested. How do we know if they actually have Git installed or not?

The way we determine this is by writing a "test", which is just a function, for each of our criteria. Then we'll "attach" the function to the criteria, then we can grade the submission.

## A Single Test
Each test needs to have the same signature, meaning it has to accept the same parameters and return the same thing. We need this consistency between tests to make grading possible.

Every test must accept a reference to a `TestData` object, and return a boolean. We created a `TestData` object with the `data!` macro when we made a submission. In fact, the exact data we put in the submission will be the data passed into each of our criteria tests. This is why we put the users Github username and repository name in the data; we'll need it inside one of our tests.

## Helpers
Before we write any tests, you should know about the [helper](../helpers/home.md) modules. These modules are a collection of functions that do common tasks in criteria tests. They may save you some times. See the documentation linked above for more info on each module.

## The First Test
Let's write a test for our first criteria, which checks if Git is installed or not. Remember that a test is just a function with a specific signature.

You can write your tests anywhere, but I'll make a `tests.rs` to keep the tests separate from the rest of our code.

```rust ,noplaypen
// tests.rs
use lab_grader::*;

fn confirm_git_installed(_: &TestData) -> bool {
    cli::Program::Git.version().is_some()
}
```

We added a function called `confirm_git_installed`. It takes in a parameter of type `&TestData`, but in this case we don't need it so we'll name the parameter `_`. In the function body, we used the `cli` helper module to get the version of Git, and returned true if it's a `Some` value (this is an `Option` type, it would be `None` if Git wasn't installed).

And that's it for the first test. There's still one step to go, but we'll do that after the other tests.


## The Rest of the Tests
I'm going to write some functions to serve as the tests for the remaining criteria. I won't explain what each one is doing in detail, but it should be pretty self explanatory. I'll put the entire `tests.rs` file here.


```rust ,noplaypen
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
```

In the `confirm_repo_pushed`, we're actually using the data attached to the submission. We can do that with bracket syntax (`data["key"]`) or through the [`get`](https://doc.rust-lang.org/beta/std/collections/struct.HashMap.html#method.get) method. Using `get` is recommended over bracket syntax.


## Attaching the Tests
We need to import the tests we wrote in `main.rs`. Add this import to the top of `main.rs`

```rust ,noplaypen
// Declare the tests mod (tests.rs)
mod tests;
// Import all the test functions
use tests::*;
```

Now that we have our tests, we need to attach them to the appropriate criteria. We can do that with the `attach!` macro.

```rust ,noplaypen
fn main() {
    // ...
    attach! {
        rubric,
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
