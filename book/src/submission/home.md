# Submission
A submission represents one students work on an assignment. It contains any data you may want about the student, like name and ID, their grade, which criteria they passed and failed, information about their system, information they provide, etc.

A submission is the other half to a rubric. While rubrics are identical among all students, every student has their own unique submission. The stucture of a submission is all the same, but the values contained in it are unique to each student.

A submission is designed to be constructed during the grading process and sent back to you, the instructor. You'll end up with a list of submissions from every student containing their grade and all the data you specified.

Let's take a gander at a blank `Submission` struct to see what's in it:

```rust ,noplaypen
Submission {
    time: 2020-08-04 Tue 21:13:34 -05:00,
    grade: 0,
    data: {},
    passed: [],
    failed: [],
}
```

There's a few default values like a timestamp, grade, and which criteria the submission passed and failed, but it's pretty empty. The important bit is the `data` `HashMap`. This is where all of the data you specify will live. All of this data will be collected when the student runs the grader, and will be sent back as part of the grade report.

## The `data!` macro
I mentioned that the `data` field on a submission is a `HashMap`. This is sort of true. It's technically a `TestData`, which is an alias to `HashMap<String, String>`.

If you read the [Criteria Tests](../rubric/tests.md) section, you might remember `TestData` as the value that criteria tests must accept as a parameter. The `TestData` on a submission is what will be passed into these tests.

Rust does not have object literals the same way a language like Python does. Instead, this crate provides a `data!` macro that will create a `TestData` for you. Here's how to use it.

```rust ,noplaypen
let data = data! {
    "some_key" => "some value",
    "other_key" => "other value"
};
```

Since `TestData` is an alias to `HashMap<String, String>`, the keys and values must be strings.


## Creating a Submission
Creating a submission is easy
```rust ,noplaypen
// creates a blank submission like the one above
let sub = Submission::new();
```

You will most likely want to create a submission with some data. You can do that like this.
```rust ,noplaypen
let data = data! {
    "key" => "value"
};

let sub = Submission::from_data(data);

// This will create the following
Submission {
    time: 2020-08-04 Tue 21:13:34 -05:00,
    grade: 0,
    data: {
        "key": "value",
    },
    passed: [],
    failed: [],
}
```
