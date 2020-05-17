# Example Grader
This chapter will walk you through building a complete grader from start to finish. I'm writing this as I publish version `0.10.0`. There will no doubt be changes in later versions.

## Scenario
Let's say we wrote a lab to teach the basics of Git, and we need to ensure that the sudents have done the following:
- Installed Git
- Initialized Git in a repository
- Made at least 3 commits in the repository
- Pushed the repository to Github

These are our "criteria", which is an important term. Because we have 4 criteria, let's say each is worth 25 points for a total of 100 points.

Let's write a grader program that the student will run. The grader will check these criteria and send a report back to us.

## Setup
Our grader will be written in [Rust](https://www.rust-lang.org/). Before we get started, be sure you have all the necessary tools to write a Rust application, including `cargo`. You can [install it here](https://www.rust-lang.org/learn/get-started) if you don't already have it. Be sure you're running on the "nightly" release of Rust. You can switch to nightly with `rustup default nightly`.

We'll make a new Rust project using `cargo`
```
$ cargo new my_grader
```


This will make 3 files for us. `Cargo.toml` is where we specify details about our application (called a "crate"). You can leave most of it alone, as we won't be publishing this crate, but you need to add this crate (`lab_grader`) as a dependency.

```toml
[dependencies]
lab_grader = "0.10.0"
```

`main.rs` contains a hello world function, so you can go ahead and compile and run your program with

```
$ cargo run
```

After it compiles you should see "Hello, world!". Now we can move on to [defining the criteria](./criteria.md).
