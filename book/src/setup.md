# Rust Setup
You'll need the Rust language installed, and it's build tool "cargo". You can [read more about that here](https://www.rust-lang.org/tools/install). It's pretty easy to install on any platform.

You'll also want to switch over to the `nightly` channel. This crate uses experimental features only available in night. Do that with
```
$ rustup default nightly
```

## Grader Setup
Once you have Rust installed, you'll want to make a new project to serve as your grading application.

You can create a new Rust project with
```
cargo new --bin my_grader
```

This is a Rust crate and can be installed like any other Rust crate. In the `Cargo.toml` file that cargo creates for you, add this crate as a dependency.

```toml
# ...
[dependencies]
rubric = "0.14" # or latest version
```

Now is a good time to create a `rubrics/` directory alongside your `src/` directory. You'll use that later.

In your `main.rs`, delcare the rubric crate and import the items you need.
```rust ,noplaypen
#[macro_use] extern crate rubric;

// import what you need
use rubric::{Rubric, Submission, dropbox};

fn main() {
    // ...
}
```

It's also recommended that you create a `tests.rs` beside `main.rs` to hold your [criteria tests](./rubric/tests.md).

You may need to access certain items or functions in the `rubric` crate. You can reference the [docs on docs.rs](https://docs.rs/rubric) for specific information.
