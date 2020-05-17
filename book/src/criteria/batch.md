# A Batch
A `Batch` is a collection of criteria. It is represented in Rust as the [`Batch`](https://docs.rs/lab_grader/0.10.0/lab_grader/batch/struct.Batch.html) struct.

A batch is the primary way that you, as the programmer, interact with a collection of criteria. Batches are created by parsing YAML data into criteria, then attaching tests to those criteria.

Detailed `Batch` documentation [here on docs.rs](https://docs.rs/lab_grader/0.10.0/lab_grader/batch/struct.Batch.html).

## Creating a Batch
See the [YAML specification](./yaml_spec.md) for more information on what is allowed and required in YAML. Here's a short example

```yaml
name: My First Batch
desc: An optional description about what this batch does

criteria:
  "First criterion"
    stub: first-crit
    worth: 25

  "Second criterion"
    stub: second-crit
    worth: 25
```

Now we can deserialize that YAML into a [`Batch`](https://docs.rs/lab_grader/0.10.0/lab_grader/batch/struct.Batch.html) struct.

```rust ,noplaypen
let yaml = yaml!("path/to/that/yaml.yml").unwrap();
let mut batch = Batch::from_yaml(yaml).expect("Bad yaml!");
```


## Using a Batch
Batches can do a few things on their own, but they're meant to be used with a `Submission`. This is what a Submission is graded against.

Here's a few code examples of what you can do with a `Batch` alone.
```rust ,noplaypen
// These print the batch and all the criteria contained in it
println!("{}", batch); // Print a batch in full
batch.print_short();   // Print a shorter version

// This gets the total points possible
println!("Total points possible: {}", batch.total_points());
// This gets the total points earned. You should grade before running this
println!("Points earned: {}", batch.points());

// Attaches a function to a particular criteria
fn my_func(_: &TestData) -> bool { true };
batch.attach("first-crit", Box::new(my_func));

// Adds a criterion to the list
batch.add(/* some criterion */);

// Gets a criterion by stub, if any
if let Some(crit) = batch.get("first-crit") {
    println!("criterion name: {}", crit.name);
}
```

## The `attach!` macro
The `attach!` macro is provided to easily attach many tests to criteria. In the code example above, you can see how to define a function and then attach it to a criterion through the `Batch::attach` function. The `attach!` macro allows you to do many of these at once, and you don't have to `Box` it either.

```rust ,noplayplen
fn test_func1(_: &TestData) -> bool { true };
fn test_func2(_: &TestData) -> bool { false };

fn main() {
    // create the batch from yaml as above...
    attach! {
        batch,
        "first-crit" => test_func1,
        "second-crit" => test_func2
    };
}
```

This is the recommended way to attach functions to criteria.
