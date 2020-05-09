# Lab Grader [![Testing Suite](https://github.com/llamicron/lab_grader/workflows/Testing%20Suite/badge.svg)](https://github.com/llamicron/lab_grader/actions?query=workflow%3A%22Testing+Suite%22)

[Repository](https://github.com/llamicron/lab_grader) |
[User Documentation](https://github.com/llamicron/lab_grader/wiki) |
[Rust Documentation](https://docs.rs/crate/lab_grader)

This is a Rust package that automates a lot of the necessary steps for grading labs in technology management classes.

This package will provide a semi-framework, but the author must provide the criteria. This way, a lot of the boiler plate is reduced and the author can focus only on what is relevant to grading the assignment.

To be specific, the author (you, probably) will defined "criteria", which are essentially tasks to be run. You can compile your program and distribute it (probably to students), and they can run the program, which then runs all the tasks (criteria) you built into it. It then sends a report along with a bundle of data that you define back to a server that you host. Then you collect the results when necessary. This package provides all the tools necessary (including a preconfigured webserver) to accomplish these tasks.

**Note:** The wiki that is linked to above hasn't been finished yet, so it may be innacurate at the moment.
