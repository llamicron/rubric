# Submitting
When a student runs the program, it should grade their submission and then submit to a location you, as the professor/TA, can access. There's two parts to this.

## Submission Server
The submission server is simply a web server with a single route: it accepts POST requests on the `/submit` route. You can run it with a single function.

You'll want to set up a publicly accessible server to run this server. I use a Microsoft Azure VM because they're pretty easy to set up and provide DNS services.

Let's add a little bit of code to the *beginning* of our `main` function. It will read the command line arguments and run the server if you run the program with the "`server`" argument.

```rust
fn main() {
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();

    // If the second one is "server"
    if args.len() == 2 && args[1] == "server" {
        // Run the server on port 8080
        Submission::server(8080);
    }

    // ...
}
```

Now you can run the program with the "`server`" argument to start up your web server. Open another terminal and run this server in the background while we finish the grader.

## Submitting
Now that the submission server is running, we can submit. Let's add this to the end of the `main` function.

```rust
fn main() {
    // ...
    let url = "http://localhost:8080/submit";
    let res = web::post_json(url, &sub);
    if let Ok(response) = res {
        println!("Submission sent, response {}", response.status());
    } else {
        println!("Error sending submission! {}", res.unwrap_err());
    }
}
```

This will submit the submission to the server that you should have running, and print a success or error message. Of course, you'll want to put the url for your server instead of `localhost`.
