//! Functions to easily make web requests
//!
//! These use the [`reqwest`](https://docs.rs/reqwest/0.10.4/reqwest/) crate
//! to make requests.
use serde::Serialize;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};

use std::time::Duration;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

// Constructs some header, this is mostly used for POST requests
fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(APP_USER_AGENT));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
}

/// Performs a GET request
///
/// If there is an error in sending the request, this will return with
/// and `Err` variant.
///
/// If the request is performed, an `Ok` variant will return containing
/// a [`Response`](https://docs.rs/reqwest/0.10.4/reqwest/blocking/struct.Response.html).
/// This `Response` could have an error code like `500` or `404`. This is different from
/// the request not being performed.
///
/// ## Example
/// ```rust
/// use lab_grader::helpers::web::get;
///
/// let result = get("https://postman-echo.com/get");
///
/// if let Ok(resp) = result {
///     // Request was successful, deal with the response
///     assert!(resp.status().is_success());
/// } else {
///     // The request failed to go through, deal with that
/// }
/// ```
/// **Get the body of data returned from a GET request**
/// ```rust
/// # use lab_grader::helpers::web::get;
///
/// let result = get("https://postman-echo.com/get");
///
/// // If the request went through and returned
/// if let Ok(resp) = result {
///     // If the request contains a body of text
///     if let Ok(body) = resp.text() {
///         assert!(body.contains("postman-echo.com"));
///     } else {
///         // Couldn't get the body from the request
///     }
/// }
/// ```
pub fn get(url: &str) -> Result<Response, reqwest::Error> {
    let client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .timeout(Duration::from_secs(4))
        .build()
        .expect("Couldn't build reqwest client. This shouldn't happen.");

    client.get(url).send()
}


/// Just calls [`get`](crate::helpers::web::get) and asserts that
/// the response was successful. Just saves a few lines of code.
///
/// ```rust
/// # use lab_grader::web;
///
/// let url = "https://postman-echo.com/get";
/// assert!(web::site_responds(url));
///
/// let bad_url = "https://probablynotawebsite.com/";
/// assert!(!web::site_responds(bad_url));
/// ```
pub fn site_responds(url: &str) -> bool {
    if let Ok(resp) = get(url) {
        return resp.status().is_success();
    }
    false
}

/// Sends a POST request to the url with the given body
///
/// `body` must be JSON serializable with `serde`
///
/// ## Example
/// ```rust
/// use lab_grader::helpers::web::post_json;
///
/// // We'll use a HashMap because it's similar to json
/// use std::collections::HashMap;
///
/// let mut data = HashMap::new();
/// data.insert("key", "value");
///
/// // This url just returns whatever we send it
/// let result = post_json("https://postman-echo.com/post", data);
///
/// // If the result went through
/// if let Ok(resp) = result {
///     // If the result contains a body
///     if let Ok(text) = resp.text() {
///         assert!(text.contains("value"));
///     }
/// }
/// ```
pub fn post_json<B: Serialize>(url: &str, body: B) -> Result<Response, reqwest::Error> {
    let client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("Couldn't build reqwest client. This shouldn't happen.");

    client.post(url)
        .headers(construct_headers())
        .json(&body)
        .send()
}
