# The `prompt!` macro
You're probably going to want to get some user input and put that into the submission's data. For instance, a submission isn't very useful if you don't know who sent it. You'll want a name and possibly ID.

The [`prompt!`](https://docs.rs/lab_grader/0.10.0/lab_grader/macro.prompt.html) macro will ask for user input and try to cast it to the given type.

```rust ,noplaypen
let name = prompt!("Name: ", String);
println!("Your name is {}", name);
```

This macro can also enfore that the user enters a certain type. If it can't cast what they entered into the given type, it will crash with an error message.

```rust ,noplaypen
let number = prompt!("Enter a number: ", isize);
println!("{} is definitely a number.", number);
```

> Note: the values of a `TestData` object must be a string, so you'll need to cast whatever they entered *back to a `String`* if you want to include it in the `TestData` object.
