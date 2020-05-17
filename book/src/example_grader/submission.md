# Building a Submission
A submission is a bundle of data that represents a students work. A submission is graded against a batch, and then sent back to you. By default, it contains a timestamp, a numerical grade, and 2 lists of the criteria that the student passed/failed.

You can add any kind of data that you might want, for example the students name and ID, or information about their system like IP address.

Any data that is needed from inside a criteria test should also be here. This will make more sense when we write the criteria tests.

## Some Housekeeping
We need to do some housekeeping in our `main.rs`

```rust
extern crate lab_grader;

use lab_grader::*;

fn main() {
    // code will go here
}
```

We added an import to the top to bring in all the items we'll need from `lab_grader`. Then we just cleared our `main` function. In the next section, we'll add code into the `main` function.


## Build a Submission
Now we can build a submission, which the `Submission::new` function. Add the following to the beginning of your `main` function.
```rust
let mut sub = Submission::new();
```
We make it mutable to we can attach data later.

## Attach Data
We want some data to attach to the submission. In this case, we're going to want the student's name and ID, as well as their Github username and the name of the repository they create for the lab. We'll use this data a little later.

We're going to use two macros to make this data:
- `data!` - creates a bundle of key/value pairs
- `prompt!` - asks the user for input from the terminal

```rust
fn main() {
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
We can refactor the code above into this, using the `Submission::from_data` function.

```rust
fn main() {
    let mut sub = Submission::from_data(data! {
        "name" => prompt!("Name: ", String),
        "id" => prompt!("ID: ", String),
        "gh_name" => prompt!("Github Username: ", String),
        "repo" => prompt!("Repo name: ", String)
    });
}

```

## Test
Add the following line to the end of main and run the program with `cargo run` to see what it does so far.

```rust
println!("{:#?}", sub);
```

Now we can move on to [building the batch](batch.md).
