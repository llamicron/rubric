# Introduction
small change. This Rust crate automates a lot of the necessary steps for testing criteria, originally created to grade labs in technology management classes.

Detailed documentation can be [found here on docs.rs](https://docs.rs/lab_grader/0.10.0/lab_grader/index.html).

> Note: this documentation is still very much a work in progress, and some sections are just missing. If you have any questions, send me an email at `llamicron@gmail.com` or on discord at `CoconutCake#3161`.
## Use Case
You should know that this program was intended for a technology management class. During the labs, the students would be managing Azure VMs, using software like Docker, Git and Github, etc. They were acting as SysAdmins, doing all the regular SysAdmin stuff. We needed a way to check each (60+) student's work on their VM or development machines, and report that back to us.

This poses a unique challenge. How do you get proof that every student has the right version of Python installed, or has a web server running on the right port? There's lots of different possibilities.

My original solution was to write a Rust program that checked everything and sent the results back to us. It wasn't modular or reusable; it was very much a bodge that was intended to work once. But after I wrote the fourth lab grader and prepared to write one to grade the final exam, I realized that I needed to set this in stone.

This library was the outcome. It's a glorified task-runner, but it's actually kind of cool. If you have any need of running a lot of commands, checking web sites and APIs, examining the file system, or performing anything across a lot of machines and getting a report from each one, then this library is for you.

## Process
The general process for writing a grader is this

- Make a new rust project - you can do this with `cargo new`

- Define your criteria - You'll write all the details about each criteria in yaml. Things like name and description, point value, etc.

- Build a Batch - from within Rust, you'll load your yaml data into a `Batch`. Then you'll write a function (a *test*) for each criterion and "attach" each test where it belongs.

- Define a Submission - Any data you want sent back to you, as well as any data you may need inside the criteria tests should be stored in a submission.

- Grade the Submission against the Batch - this is done with one function call.

- Submit the Submission - this is done by POSTing the data (with the web helper module). Again, one function call.

That might seem like a lot, but it's pretty easy once you do it. A good place to get started is the example grader exercise. It will walk you through a complete grader.


## Terminology

Here's a list of the terminology that I use throughout the program. This will help you understand exactly what's going on. It's pretty straightforward anyway but I'd rather be explicit about this.

If a term is formatted like `this`, that means it's represented in code as a struct or module of the same name.

- Grader - The program you're writing when you define criteria. "Grader" may not be the best term for your use case, but it's how I use this library.

- `Criterion` - a bit of clerical data, and a "test", which is just a function that returns true or false. The Criterion's test is the heart of the application. It's a function that you write. Data can be passed into the Criterions test, but the criterion itself doesn't store it.

- `Batch` - a batch is basically just a wrapper around a collection of criteria. This is the highest level of abstraction. You may want to run your tests in phases, or batches. Each yaml file that you define later on is a single batch. 1 yaml file = 1 Batch.

- Submission Server - just a web server that accepts Submissions as json data, then writes them to a file. You can start this server in one function call, all you need is a machine to run it on.

- `Submission` - This is a bundle of data that represents the results of grading the criteria. The data it carries is defined by you. It is sent back to the submission server (that you run). A Submission is graded "against" a set of Criteria. Any data that you need in any criterion should be in here.

- `helpers` - this is pretty self evident. It's a module (with submodules) that contains functions that easily accomplish tasks that you'll probably run into. Currently there are 3 helper modules, but more will be added:
  - `cli` - handles terminal operations like getting user input
  - `fs` - file system operations, like ensuring a file exists or contains something
  - `web` - make GET and POST web requests in one function call

As I said in the "Use Case" section, this was originally intended to grade TCMG labs. As a consequence, there may be some academia-oriented terminology in here. Originally, the Submission type had a mandatory student ID and name field, but I've removed those to make it more flexible. I'm trying to remove anything that would limit this application to just that use case. You may be using this for something other that what it was intended, which is fantastic.
