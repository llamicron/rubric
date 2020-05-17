# Submission
A [`Submission`](https://docs.rs/lab_grader/0.10.0/lab_grader/submission/struct.Submission.html) represents one students work on a lab. It's a bundle of data that can have whatever you want in it. A `Submission` works together with a `Batch`. A `Submission` is graded against a `Batch` and assigned a grade.

After a `Submission` is graded, it can be sent back to you, as the professor/TA, for review.

Submissions come with a timestamp by default. This will be the timezone of the system it was submitted from, in [rfc3339](https://tools.ietf.org/html/rfc3339) format. It also has a numerical grade that will be changed when grading it. The `passed` and `failed` fields store which criteria this submission passed or failed.

The most important part of a submission is it's data, which you can read about in [the next section](./test_data.md).
