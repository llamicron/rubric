# Lab Grader
[![Testing Suite](https://img.shields.io/github/workflow/status/llamicron/lab_grader/Testing%20Suite?label=Testing%20Suite&style=for-the-badge)](https://github.com/llamicron/lab_grader/actions?query=workflow%3A%22Testing+Suite%22)
[![Latest Version](https://img.shields.io/crates/v/lab_grader?style=for-the-badge)](https://crates.io/crates/lab_grader)
[![License](https://img.shields.io/crates/l/lab_grader?style=for-the-badge)](https://crates.io/crates/lab_grader)


[![Repostiroy](https://img.shields.io/badge/%20-Repository-informational?style=for-the-badge)](https://github.com/llamicron/lab_grader)
[![User Documentation](https://img.shields.io/badge/%20-User%20Documentation-informational?style=for-the-badge)](https://github.com/llamicron/lab_grader/wiki)
[![Rust Documentation](https://img.shields.io/badge/%20-Rust%20Documentation-informational?style=for-the-badge)](https://docs.rs/crate/lab_grader)

This is a Rust package ("crate") that automates a lot of the necessary steps for testing criteria, originally created to grade labs in technology management classes.

To be specific, the author (you, probably) will define "criteria", which are essentially tasks to be run. You can compile your program and distribute it (probably to students), and they can run the program, which then runs all the tasks (criteria) you built into it. It then sends a report back to a server that you host. Then you collect the results when necessary. This package provides all the tools necessary (including a preconfigured web server) to accomplish these tasks.

See the [Example Grader](https://github.com/llamicron/lab_grader/wiki/Example-Grader) in the Github Wiki for a complete example. Links for the documentation above.
