# Introduction




# Setup
This is a Rust crate and can be installed like any other Rust crate. You'll need Rust and it's build tool, `cargo` installed. [Read more about that here](https://www.rust-lang.org/tools/install).

You can create a new Rust project with
```
cargo new --bin my_grader
```

In the `Cargo.toml` file that cargo creates for you, add this crate as a dependency.

```toml
# ...
[dependencies]
rubric = "0.12" # or latest version
```

In your `main.rs`, delcare the crate with the `#[macro_use]` configuration flag. You can import just the items you need (recommended), or import everything with `*`.
```rust ,noplaypen
#[macro_use] extern crate rubric;

// import what you need
use rubric::{Rubric, Submission, TestData, /* ... */};
// or everything
use rubric::*;
```

I would also recommend creating a `rubrics/` directory alongside your `src/` directory to store your rubrics.
