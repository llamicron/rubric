# Lab Grader [![Testing Suite](https://github.com/llamicron/lab_grader/workflows/Testing%20Suite/badge.svg)](https://github.com/llamicron/lab_grader/actions?query=workflow%3A%22Testing+Suite%22)

[Repository](https://github.com/llamicron/lab_grader) |
[User Documentation](https://github.com/llamicron/lab_grader/wiki) |
[Rust Documentation](https://docs.rs/crate/lab_grader)

This is a Rust package ("crate") that automates a lot of the necessary steps for testing criteria, originally created to grade labs in technology management classes.

To be specific, the author (you, probably) will define "criteria", which are essentially tasks to be run. You can compile your program and distribute it (probably to students), and they can run the program, which then runs all the tasks (criteria) you built into it. It then sends a report back to a server that you host. Then you collect the results when necessary. This package provides all the tools necessary (including a preconfigured web server) to accomplish these tasks.

See the [Example Grader](https://github.com/llamicron/lab_grader/wiki/Example-Grader) in the Github Wiki for a complete example. Links for the documentation above.
