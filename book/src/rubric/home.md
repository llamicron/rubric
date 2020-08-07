# Rubrics
Rubrics are the main component of this framework. A rubric is simply a list of criteria. They are the first thing you should write down when writing a grader.

Within this framework, rubrics are represented with `.yml` files in the `rubrics/` directory. "YAML" is a markup language that you can [read about here](https://docs.ansible.com/ansible/latest/reference_appendices/YAMLSyntax.html). It's pretty common within sysadmin tools and chances are you're already familiar with it. We can use YAML to write out the details of our criteria.

## A Practical Example
Rubrics are very easy to understand if you see one.

In the [`git_lab` example](https://github.com/llamicron/rubric/tree/master/examples/git_lab), we wrote a hypothetical assignment for our hypothetical students. This lab is meant to teach our students about git and how to use it. We came up with 4 criteria that will make up our assignment. These criteria are

1. The student must install git and have it available at the command line
1. The student must initialize git in a directory
1. The student must make at least 3 commits in that repository
1. The student must push the repository to Github and have it publicly accessible

This is a great start. All we need to do is formalize this into a rubric.

```yml
# Our assignments name
name: Git Lab
# An optional description
desc: A lab to teach git

# Set a deadline
deadline: 2023-06-07 23:59:59
# Don't allow late submissions
allow_late: false


# A list of our criteria
criteria:
  "Git installed":
    func: git_installed
    desc: Git should be installed and accessible
    worth: 25
    messages: ["installed", "not installed"]

  "Git initialized in repo":
    func: git_init
    desc: Current directory should have git initialized
    worth: 25
    messages: ["initialized", "not initialized"]

  "Commits present":
    func: commits_present
    worth: 25
    desc: Current git repository should have more than 2 commits

  "Repo pushed":
    func: repo_pushed
    worth: 25
    desc: Current git repository should be pushed to github
```

You can see that each criteria has a name, a point value, and some other configuration options. The only required fields are the name and worth, everything else has defaults.

Lets dissect one of the criteria
```yml
# ...
criteria:
  # The criteria name
  "Git installed":
    # This is the name of the corresponding function.
    # if you omit this, the function name will be the criterion's name,
    # lower-cased and whitespace replaced with dashes
    func: git_installed
    # Just a description. Students will see this when grading
    desc: Git should be installed and accessible
    # The point value, can be any number, even negative.
    worth: 25
    # Success/failure messages. These default to "passed" and "failed".
    # The're just some extra information for the student
    messages: ["installed", "not installed"]
```

There are a few more options that you can provide, which you can read about on the [Rubric Specification Page](spec.md).

## Loading a Rubric
After writing out a rubric in a `.yml` file, we can load it into our grader and use it.

The first step to loading a rubric is reading the file. This is done with the `yaml!` macro.

```rust ,noplaypen
let rubric_yaml = yaml!("../rubrics/main.yml").expect("Couldn't load file!");
```

`yaml!` loads a file relative to the current file. It returns a `Result`, so we can deal with the error if we want. It's usually a good idea to call `expect()` so the program crashes if the file couldn't be read.

`yaml!` is special because it embeds the `.yml` in the compiled executable. This means you don't have to distribute the rubric file with the executable. Your rubric can also be kept private if you want it to be.

Next, we just have to pass the YAML data to the `Rubric` struct like this

```rust ,noplaypen
let rubric = Rubric::from_yaml(&rubric_yaml).expect("Bad YAML!");
```

> Note: We're using `expect()` again here. We want the program to crash at *compile-time* when we're working on the grader, not at *run-time* when the students are using it. Better for us to deal with the error than them.

## Writing The Tests
We have a rubric loaded, but it has no way to actually verify that the criteria have been fulfilled. We're going to write one function for each of the criteria. The function (called a "criteria test" or just "test") has the responsibility of ensuring the criteria was actually fulfilled by the student.

Assignments obviously vary greatly in scope and material, so writing these tests is the bulk of the work to be done when writing a grader. It all depends on what the student should be doing.

Things like installing Git or running a web server are easy to verify, but other tasks might not be. Because every criteria has a function, the full force of Rust is on your side. You may have to get creative in writing your tests. You can always look at the [examples on Github](https://github.com/llamicron/rubric/tree/master/examples) for inspiration.

See the [Criteria Tests](tests.md) page for more information on this topic.
