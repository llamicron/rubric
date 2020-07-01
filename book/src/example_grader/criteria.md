# Defining the Criteria
Once we have our project set up, the first step is to define our criteria. Criteria are contained within a "Rubric". A Rubric has a name, description, and a list of criteria. It's represented in Rust by the `Rubric` structure. We're going to write a `yaml` file, and all the data we put in there will be serialized into a `Rubric`.

Let's make a directory called `criteria/`, and inside there we'll make a file called `rubric.yml`:

```yaml
# criteria/rubric.yml
name: Git Lab
desc: Install and use Git
```

Here we've put a name for our rubric, and then an (optional) description.

Next we'll add our criteria. You can see the [YAML specification](../criteria/yaml_spec.md) to see all the keys that are available here. As a reminder, here's the criteria we wrote out in the last section:
- Installed Git
- Initialized Git in a repository
- Made at least 3 commits in the repository
- Pushed the repository to Github


```yaml
# criteria/rubric.yml
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

We've put 4 criteria in the `yaml` file. Each one has a name, a stub (an identifier), a point value, index, and some success/failure messages. The name and worth are the only required fields.

Once again, see the [YAML specification]() for more info on what you can put in this file.

Now we have our criteria defined, we can move on to [writing some Rust](submission.md).
