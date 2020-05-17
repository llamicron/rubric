# Creating a Submission

We can combine what we know about about submissions, [`TestData`](./test_data.md), and [`prompt!`](./test_data.md#the-prompt-macro) to make a complete submission.

This snippet will make a new submission, ask the user for their name and ID, then wrap those into a `TestData` object and attach it to the submission.

```rust ,noplaypen
let sub = Submission::from_data(data! {
    "name" => prompt!("Name: ", String),
    "id" => prompt!("ID: ", String)
});
```
