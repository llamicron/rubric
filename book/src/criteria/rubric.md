# A Rubric
A `Rubric` is a collection of criteria. It is represented in Rust as the [`Rubric`](https://docs.rs/lab_grader/0.11.0/lab_grader/rubric/struct.Rubric.html) struct.

A rubric is the primary way that you, as the programmer, interact with a collection of criteria. Rubrices are created by parsing YAML data into criteria, then attaching tests to those criteria.

Detailed `Rubric` documentation [here on docs.rs](https://docs.rs/lab_grader/0.11.0/lab_grader/rubric/struct.Rubric.html).

## Creating a Rubric
See the [YAML specification](./yaml_spec.md) for more information on what is allowed and required in YAML. Here's a short example

```yaml
name: My First Rubric
desc: An optional description about what this rubric does

criteria:
  "First criterion"
    stub: first-crit
    worth: 25

  "Second criterion"
    stub: second-crit
    worth: 25
```

Now we can deserialize that YAML into a [`Rubric`](https://docs.rs/lab_grader/0.10.0/lab_grader/rubric/struct.Rubric.html) struct.

```rust ,noplaypen
let yaml = yaml!("path/to/that/yaml.yml").unwrap();
let mut rubric = Rubric::from_yaml(yaml).expect("Bad yaml!");
```


## Using a Rubric
Rubrices can do a few things on their own, but they're meant to be used with a `Submission`. This is what a Submission is graded against.

Here's a few code examples of what you can do with a `Rubric` alone.
```rust ,noplaypen
// These print the rubric and all the criteria contained in it
println!("{}", rubric);  // Print a rubric in full
rubric.print_short();    // Print a shorter version
rubric.print_table();    // print a full table

// This gets the total points possible
println!("Total points possible: {}", rubric.total_points());
// This gets the total points earned. You should grade before running this
println!("Points earned: {}", rubric.points());

// Adds a criterion to the list
rubric.add(/* some criterion */);

// Gets a criterion by stub, if any
if let Some(crit) = rubric.get("first-crit") {
    println!("criterion name: {}", crit.name);
}


// Attaches a function to a particular criteria
fn my_func(_: &TestData) -> bool { true };
if let Some(crit) = rubric.get("first-crit") {
    crit.attach(Box::new(my_func));
}
```

## The `attach!` macro
The `attach!` macro is provided to easily attach many tests to criteria. In the code example above, you can see how to define a function and then attach it to a criterion through the `Rubric::attach` function. The `attach!` macro allows you to do many of these at once, and you don't have to `Box` it either.

```rust ,noplayplen
fn test_func1(_: &TestData) -> bool { true };
fn test_func2(_: &TestData) -> bool { false };

fn main() {
    // create the rubric from yaml as above...
    attach! {
        rubric,
        "first-crit" => test_func1,
        "second-crit" => test_func2
    };
}
```

This is the recommended way to attach functions to criteria.
