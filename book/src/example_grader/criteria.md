# Defining the Criteria
Once we have our project set up, the first step is to define our criteria. Criteria are contained within a "Rubric". A Rubric has a name, description, and a list of criteria. It's represented in Rust by the `Rubric` struct.

We need to write out all the details of each of our criteria so our grader program can use them. We'll write the details in a `yaml` file, which our grader will automatically turn into a gradable `Rubric`.

You can put this `yaml` file wherever you want, but I like to keep things organized. Let's make a directory called `rubrics/`, and inside there we'll make a file called `main.yml`:

```yaml
# rubrics/main.yml
name: Git Lab
desc: Install and use Git
```

Here we've put a name for our rubric, and then an (optional) description.

Next we'll add our criteria. You can reference the [Rubric YAML specification](../criteria/yaml_spec.md) to see all the keys that are available here. 

As a reminder, here's the criteria we wrote out in the last section:
1. Install Git
1. Initialize Git in a repository
1. Make at least 3 commits in the repository
1. Push the repository to Github


```yaml
# rubrics/main.yml
name: Git Lab
desc: Install and use Git

criteria:
  "Git installed":
    stub: git-installed
    index: 1
    worth: 25
    messages: ["installed", "not installed"]

  "Git initialized":
    stub: git-init
    index: 2
    worth: 25
    messages: ["initialized", "uninitialized"]

  "Commits present":
    stub: commits-present
    index: 3
    worth: 25
    messages: [">= 3 commits found", "< 3 commits found"]

  "Repo pushed":
    stub: repo-pushed
    index: 4
    worth: 25
    messages: ["pushed", "not pushed"]
```

Here, we've described our 4 criteria. Each one has
  - A descriptive `name`
  - A `stub`, which is a unique identifier
  - `worth`, it's point value
  - and `messages`, which are success/failure messages that the student will get when grading.

`name` and `worth` are the only required fields, but it's good to be explicit.

Once again, see the [YAML specification]() for more info on what you can put in this file.

Now we have our criteria defined, we can move on to [writing some Rust](submission.md).
