# A Single Criterion
A `Criterion` is the bread and butter of this application. To help us understand it a bit better, I'm going to put the definition of the `Criterion` struct here so we can take it apart.

Detailed `Criterion` documentation available [here on docs.rs](https://docs.rs/lab_grader/0.10.0/lab_grader/criterion/struct.Criterion.html).

```rust ,noplaypen
// Comments elided
pub struct Criterion {
    pub stub: String,
    pub name: String,
    pub worth: i16,
    pub index: i64,
    pub messages: (String, String),
    pub desc: Option<String>,
    pub test: Box<dyn Fn(&TestData) -> bool>,
    pub status: Option<bool>,
    pub hide: bool,
}
```

If you look through these fields, you might recognize them as the fields allowed in the [YAML specification](./yaml_spec.md). In fact, the only field that isn't present in the YAML spec is the `test` field. This is because `test` is a function, and we can't write Rust functions from inside YAML.

You should pretty much always create criteria from YAML, and parse it into a `Rubric` type. A single criteria alone isn't really useful. Nonetheless, there is a [`CriterionBuilder`](https://docs.rs/lab_grader/0.10.0/lab_grader/criterion_builder/struct.CriterionBuilder.html) struct that helps you create them individually. If you want you can create a list of criteria that way then throw them into a rubric. But this is much more work than through YAML.

The best reason I can think of for creating a `Criterion` manually is if you want to programmatically change a piece of data like the name or worth. But, you can always define it in YAML and change just that field later.

Really all a `Criterion` can do is run it's own test with or without data, and return a result. Again, they aren't very useful alone.
