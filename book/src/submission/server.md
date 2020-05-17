# Submission Server
The "Submission Server" is the other half to [submitting](./submit.md). It's a preconfigured web server built to handle submissions.

You can run it through the `Submission` type like this

```rust ,noplaypen
// Running on port 8080
Submission::server(8080);
```

This will run a web server with two routes:
1. `GET /` - returns an `OK` status with no other content
2. `POST /submit` - Accepts a JSON body of a `Submission`, returns an `Accepted` code on success, and an `InternalServerError` on failure.


## Running the server
I like to set up a Microsoft Azure server to run this web server on, but you can do whatever you like. It just needs to be publicly accessible with a static IP or DNS name. You should put the address to your server (including the `/submit` bit) as the url to POST the submission to.

Be sure your server has the proper ports exposed. If you want to run on port 80 and omit the port in the url, be sure to run with root permission.

I like to run the server from within my grader through a command line parameter. Here's an example.

```rust ,noplaypen
use std::env;

// ...

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "server" {
        Submission::server(8080);
    }
}
```

## The Results
When the server accepts the first submission, it will make a file (in the directory you run the server in) called `submissions.csv`. It will then write the submission (including all the data you put in it) to the csv file.

It's important to note that the header for the csv file will be written according to the *first* submission it accepts. If, for some reason, different submissions has different data keys, the csv file will break. This is because one submission should have more columns and values than another. You should be sure not to conditionally add data to a submission.
