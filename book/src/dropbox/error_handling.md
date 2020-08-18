# Error Handling

When sending a submission to the dropbox, it's very important to provide feedback to the student. They should know if the submission went through successfully.

Submitting with the `Submission::submit()` function returns a `Result` that you can use to handle any possible errors. The `Err` variant of the `Result` is a [`reqwest::Error`](https://docs.rs/reqwest/0.10.7/reqwest/struct.Error.html).

```rust
// some code omitted
fn main() {
    // Open the dropbox somewhere else
    dropbox::open_with_arg("open_sesame", 8080);

    let mut sub = Submission::new();
    
    // Grading, etc goes here...

    // using `match` gives us basic error handling.
    // here, we're just printing the error. This can give more insight
    // as to what went wrong.
    match sub.submit("http://localhost:8080/submit") {
        Ok(_) => println!("Submission recorded!"),
        Err(e) => println!("Error! Couldn't record submission.\n{}", e);
    }
}
```
