# Web - `helpers::web`
These functions relate to making web requests. They use the [`reqwest`](https://docs.rs/reqwest/0.10.4/reqwest/) crate.

All of these functions use the `blocking` feature of reqwest, meaning they'll block until the request returns. You can make asynchronous requests with the reqwests crate, but that would involve more complicated code. I'm trying to keep it simple.

## `get`
Makes a GET request to the given URL, returning a [`Result<Response>`](https://docs.rs/reqwest/0.10.4/reqwest/blocking/struct.Response.html).

URL has to be valid or this will fail immediately.

Be cautious when making web requests; they take quite a bit of time. Only use them when necessary.

```rust ,noplaypen
let url = "https://postman-echo.com/get";
let result = web::get(url);

assert!( result.is_ok() );

if let Ok(response) = result {
    // Get the body from the response
    if let Ok(body) = response.text() {
        assert!( body.contains("postman-echo.com") );
    }
}
```

## `site_responds`
Returns true if a site responds with a success message. This will return false on 404 codes and other error codes.

```rust ,noplaypen
let url = "https://postman-echo.com/get";

assert!( web::site_responds(url) );
```

## `post_json`
This posts data in JSON format to the given URL. The parameter you pass in must implement the [`Serialize`](https://serde.rs/derive.html) trait from [`serde`](https://serde.rs/).

```rust ,noplaypen
let url = "https://postman-echo.com/post";

// Submission is serializable
let sub = Submission::new();

let result = web::post_json(url, &sub);

assert!( result.is_ok() );
```


## `post`
This is just like `post_json`, except it posts arbitrary string data. It sets the `CONTENT_TYPE` header to be `text/plain`.

```rust ,noplaypen
let url = "https://postman-echo.com/post";

let data = "here's some data to post, wonder where it will go";

let result = web::post(url, data);
assert!( result.is_ok() );
```


## `get_ip`
This retrieves the public IP of the machine. This makes a web request, so it may take more time than you would expect. Returns `None` if it couldn't be retrieved or doesn't exist.

```rust ,noplaypen
let IpAddr = web::get_ip();

assert!( IpAddr.is_some() );
if let Some(ip) = IpAddr {
    println!("My ip is: {}", ip);
}
```
