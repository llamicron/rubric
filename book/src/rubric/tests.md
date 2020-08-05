# Criteria Tests
As stated on the [Rubrics](home.md) page, a criteria tests is a Rust function that verifies if a criteria was actually fulfilled. They will vary widely. This page will show how to write a test and a few examples to get you started.

## A Single Test
Tests are just functions, but they must have a specific signature. They must always accept the same parameters and return the same type of value. Here's the signature

```rust ,noplaypen
// Be sure TestData is imported
use rubric::TestData;

fn my_test(data: &TestData) -> bool {
    // Test code goes here...
}
```

Every test must accept a reference to a `TestData` struct. This `TestData` is stored on a [`Submission`](../submission/home.md), which I'll cover in a different section. What you should know now is that it's *an alias to* `HashMap<String, String>`.

Sometimes you won't need `TestData` in a test, in which case you can just name the parameter `_` and Rust won't complain.

Tests must also return a boolean. `true` if it passes, `false` otherwise. If a test returns `true`, then the associated criteria's worth will be added to the point total. If all the criteria tests return true, the maximum score is achieved.

### Using `TestData`
Remember that a `TestData` struct is really just a `HashMap`. It will contains keys and values that *you specify* when setting up a [`Submission`](../submission/home.md). You can use any of the [methods that HashMap's have](https://doc.rust-lang.org/beta/std/collections/struct.HashMap.html). 90% of the time, you'll just want to read a value from the `TestData`. There's 2 ways to do that.

```rust ,noplaypen

fn some_test(data: &TestData) -> bool {
    // The easy but dangerous way to get a value
    // this will *crash* if the key doesn't exist
    let my_value = data["my_key"];

    // The safe way to get a value
    if let Some(value) = data.get("my_key") {
        // Key exists, now we have the value
        println!("Value is {}", value);
    } else {
        // Key doesn't exist, something went wrong, handle error
        println!("Value doesn't exist!");
    }

    // ...
}
```

It's important that you take precautions when writing a grader. You really don't want it to crash while your students are running it. The two examples above to the same thing, but the second method won't crash if the key doesn't exist.


## Organization
I strongly recommend making a `test.rs` file alongside `main.rs` to keep your tests in. Of course, you don't have to. You could keep your tests as loose functions in `main.rs`, or maybe have a submodule in `main.rs`.

Again, I recommend making a `tests.rs` file and keeping them in there. Here's how I set things up.

```rust ,noplaypen
// tests.rs;
use rubric::TestData;

fn test_from_tests_rs(_: &TestData) -> bool {
    // test code goes here
    return true;
}

// more tests here...
```

```rust ,noplaypen
// main.rs
extern crate rubric;

// declare tests.rs as a module
mod tests;
// bring all test functions into scope
use tests::*;
// ...
```



## Attaching Tests
Each criteria has an associated test, but we need to tell our program which test goes with which criteria. After we've written our tests and loaded our rubric, we can use the `attach!` macro to assign them. Just point the criteria's `stub` that you specified in the `.yml` file to the function name.

```rust ,noplaypen

fn my_criteria_test(_: &TestData) -> bool {
    // test code goes here...
    return true;
}


fn main() {
    // load rubric
    // code omitted
    // be sure it's mutable
    let mut rubric = //...

    attach! {
        rubric,
        "criteria-stub" => my_criteria_test
    };
}
```

## Helpers
There are a few helper modules and functions that perform some common tasks. Sometimes your tests will be one-liners from the helper modules. See the [`helpers`](https://docs.rs/rubric/0.11.1/rubric/helpers/index.html) module documentation on docs.rs for more info.


## Examples
Some basic examples can be found in the [`examples` directory on Github](https://github.com/llamicron/rubric/tree/master/examples), specifically in [this file](https://github.com/llamicron/rubric/blob/master/examples/git_lab/src/tests.rs) in the `git_lab` example.
