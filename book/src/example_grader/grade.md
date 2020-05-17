# Grading
We now have a complete `Batch` and a `Submission`, which is all we need to grade.

When we grade the submission, we'll run each criteria test in the batch. The submission's data will be passed into each of the tests. If the test passes, the submissions `grade` field will increase by the worth of the criterion.

```rust ,noplaypen
fn main() {
    // ...
    // Grade the submission
    sub.grade_against(&mut batch);

    // Print the batch results
    println("{}", batch);
}
```

We've graded the submission and then printed the batch. Printing the batch will show the student all the criteria and let them know what they passed or failed. Of course, you don't have to do this. You may want to keep one or all of your criteria private. You can hide individual criteria with the `hide` field in yaml, and you can always just not print the batch.

That's all there is to grading. The next step is to submit to a submission server, which we'll do in the [next section](submit.md).
