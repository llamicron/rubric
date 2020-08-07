# Dropbox
A dropbox is a place to send submissions after they're done being graded. It recieves submissions in JSON format and writes them to a CSV file for review by the instructor.

The dropbox is a web server run by you, the instructor. It should be run on a publicly available server with a static IP or DNS name. The web server comes preconfigured, all you need to do is give it an environment to run it.

You can open the dropbox by calling the `dropbox::open(PORT)` method, where `PORT` is a valid port number.

```rust
extern crate rubric;
use rubric::dropbox;

fn main() {
    // Runs the dropbox on port 8080
    dropbox::open(8080);
}
```
You'll see the following output
```
Dropbox is open! accepting POST requests to /submit
�🔧 Configured for development.
    => address: 0.0.0.0
    => port: 8080
    => log: normal
    => workers: 24
    => secret key: generated
    => limits: forms = 32KiB
    => keep-alive: 5s
    => tls: disabled
🛰  Mounting /:
    => GET / (return_ok)
    => POST /submit application/json (accept_submission)
��� Rocket has launched from http://0.0.0.0:8080       
```

The home route (`/`) should return an OK status, but no content. If you visit the url of your webserver, you should get a blank web page. This is good, it means everything is working properly.

## Submitting to the dropbox
Submissions come with a `submit()` method meant to work with the dropbox. 

```rust
extern crate rubric;
use rubric::Submission;

fn main() {
    let submission = Submission::new();
    
    // grade...

    // assuming your dropbox is running at this url
    let url = "http://my.dns.name.or.ip.com:8080/submit";

    // Submit and give some feedback
    match submission.submit(&url) {
        Ok(_)  => println!("Submission recorded!"),
        Err(e) => println!("Something went wrong! {}", e),
    };
}
```



The `post_json()` method in the `helpers::web` module is made with the dropbox in mind. After creating and grading a Submission, just pass it and the url of your dropbox to send the submission.

```rust
extern crate rubric;
use rubric::{Submission, dropbox, helpers::web};

fn main() {
    let submission = Submission::new();
    
    // grade...

    // assuming your dropbox is running at this url
    let url = "http://my.dns.name.or.ip.com:8080/submit";

    // Submit and give some feedback
    match web::post_json(&url, &sub) {
        Ok(_)  => println!("Submission recorded!"),
        Err(e) => println!("Something went wrong! {}", e),
    };
}
```
