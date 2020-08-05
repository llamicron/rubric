# Prompting for Data
When creating a submission, you'll almost always want to ask the student for some information. Usually their name and ID, but maybe some other information as well.

The `prompt!` macro does this easily. 

```rust
fn main() {
    // Asks for something, enforces the correct type
    let name = prompt!("Name: ", String);
    // Will loop until they enter a number
    let age = prompt!("Age: ", usize);
    // Will loop until they enter a valid IPv4 address
    let ip = prompt!("IP Addr: ", std::net::Ipv4Addr);
}
```

You can combine this with the `data!` macro to easily collect information from the user and encapsulate it in a submission.

```rust ,noplaypen
fn main() {
    let data = data! {
        "name" => prompt!("Name: ", String),
        "id"   => prompt!("ID: ", String)
    }
}
```

> Note: because `TestData` must contain string values, you lose out on the type enforcement that `prompt!` provides. This is an unfortunate side effect of the `TestData` type; all values must be strings.
