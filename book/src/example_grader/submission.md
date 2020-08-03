# Building a Submission
Let's take a break from our rubric for a minute.

A "submission" is a bundle of data that represents a students work. A submission is graded against a rubric, and then sent back to you. By default, it contains a timestamp, a numerical grade, and which criteria the student passed or failed.

You can add any kind of data that you might want, for example the students name and ID, or information about their system like IP address. Different situations call for different data, so it's kept as flexible as possible.

Sometimes you'll also need certain data to test a criterion in our rubric. You can include that data in the submission. This will become clear later.

## Some Housekeeping
We need to do some housekeeping in our `main.rs`

```rust ,noplaypen
extern crate rubric;

use lab_grader::*;

fn main() {
    // code will go here
}
```

We added an import to the top to bring in all the items we'll need from `lab_grader`. Then we just cleared our `main` function. In the next section, we'll add code into the `main` function.

> Note: glob importing with `*` is usually bad practice because you can't explicitly see what is brought into scope vs. what was already there. I'd rather not specify every import in this example, so we'll leave it as a glob. After you're done you can reduce it to only the items you need.


## Build a Submission
Now we can build a submission, which the `Submission::new` function. Add the following to the beginning of your `main` function.
```rust ,noplaypen
let mut sub = Submission::new();
```
We make it mutable to we can attach data later.

## Attach Data
We want some data to attach to the submission. In this case, we're going to want the student's name and ID, as well as their Github username and the name of the repository they create for the lab. We'll use this data a little later.

We're going to use two macros to make this data:
- `data!` - creates a bundle of key/value pairs
- `prompt!` - asks the user for input from the terminal

```rust ,noplaypen
fn main() {
    // Create submission
    let mut sub = Submission::new();

    // Create data
    let data = data! {
        "name" => prompt!("Name: ", String),
        "id" => prompt!("ID: ", String),
        "gh_name" => prompt!("Github Username: ", String),
        "repo" => prompt!("Repo name: ", String)
    };

    // Attach data to submission
    sub.use_data(data);
}
```

## Refactor
We can refactor the code above into this, using the `Submission::from_data` function instead of `new`.

```rust ,noplaypen
fn main() {
    // Create submission with data
    let mut sub = Submission::from_data(data! {
        "name" => prompt!("Name: ", String),
        "id" => prompt!("ID: ", String),
        "gh_name" => prompt!("Github Username: ", String),
        "repo" => prompt!("Repo name: ", String)
    });
}

```

## Test
Add the following temporary line to the end of `main` and run the program with `cargo run` to see what it does so far.

```rust ,noplaypen
println!("{:#?}", sub);
```

Now we can move on to [building the rubric](rubric.md).
