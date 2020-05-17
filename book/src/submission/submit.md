# Submitting
You can send a submission with a POST request. The easiest way to do this is through the [`post_json`](https://docs.rs/lab_grader/0.10.0/lab_grader/helpers/web/fn.post_json.html) method from the `helpers::web` module.

The `Submission` type is JSON serializable, so we don't have to do any extra work before sending.

```rust ,noplaypen
let sub = Submission::from_data(/* data here */);

// application code here...

let url = "http://localhost:8080/submit";
web::post_json(url, &sub);
```

Here, I'm POSTing the submission to a server running on `localhost:8080`. There's a special web server meant to handle these requests, which you can read about in the [submission server](./server.md) section.

## Error Handling
The `web::post_json` method returns a `Result` with a [`Response`](https://docs.rs/reqwest/0.10.4/reqwest/blocking/struct.Response.html) type inside, which can be used to handle errors. There's 101 ways you could do this, but here's an example.

```rust ,noplaypen
let url = "http://localhost:8080/submit";

if let Ok(resp) = web::post_json(url, &sub) {
    if resp.status().is_success() {
        // if the response has a success code
        println!("Success! server responded with {}", resp.status());
    } else {
        // otherwise it came back with an error
        println!("Error! the server responded with {}", resp.status());
    }
} else {
    // This means the response couldn't even be completed
    println!("Error! The request could not be performed!");
}
```
