# Building the Batch
In [the first section](criteria.md) we defined our criteria in a `yaml` file. Now we need to load the `yaml` data into Rust and build a `Batch` from it.

## A Note about Errors
In Rust, the main way that errors are handled in through the [`Result`](https://doc.rust-lang.org/std/result/) and [`Option`](https://doc.rust-lang.org/std/option/) types. These are massively important to Rust, and you should read over them and learn how they work.

A very good guide is from the [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/error.html) book. The "Error Handling" section should take less than an hour to read over and will be very useful if you continue with Rust.

I'm going to be using the `expect` method, which is normally bad practice. It simply panics (aborts) with an error message if there's an error. Normally you would want to deal with the error in one way or another, but I'm going to use this anyway since it's just an example. `expect` should *not* be used in production code.

## Reading YAML Data
We can read YAML data with the `yaml!` macro. `yaml!` takes in a relative file path and returns the YAML data as a `String`.

This macro is very important for one reason. When you compile in debug mode (default), this macro will read from the file system as expected. However, when you compile for release (with the `--release` flag), it will read the file contents and embed the contents in the created executable. This means when you distribute the grader to your students, you don't need to provide the `yaml` file. The executable will run on it's own. Just be sure to compile in release mode before distributing.

We can go ahead and add this to the end of our `main` function

```rust ,noplaypen
let yaml = yaml!("../criteria/batch.yml").expect("Couldn't read file");
```

## Building a Batch
Now that we have our yaml data, we can build a `Batch` from it.

```rust ,noplaypen
let mut batch = Batch::from_yaml(yaml).expect("Bad yaml!");
```

Here we're using the `expect` method again, but it's probably a good idea in this case. This will crash if we have invalid YAML or missing items. Once you're done developing and compile for release, the YAML will be embedded and won't change, so it won't crash after that.

That's all there is to building a batch. Here's the complete `main.rs` file so far

```rust ,noplaypen
extern crate lab_grader;

use lab_grader::*;

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

Now we can move on to [writing the criteria tests](tests.md).
