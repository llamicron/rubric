# Rubric Specification
If you're looking for the syntax of the YAML language itself, then [look here](https://docs.ansible.com/ansible/latest/reference_appendices/YAMLSyntax.html). This page is for the allowed items in a rubric's `.yml` file.


## Minimal Rubric
Here is a rubric with as few items as possible. Everything here is required.

```yml
name: My rubric

criteria:
  "first and only criterion":
    worth: 10
```

## All Items
Here is a full rubric with everything specified, with comments for more information about each key.


```yml
# Required name
name: My rubric
# Optional description. Gets shown to the student when grading
desc: Description of my rubric
# Sanity check. If the sum of all criteria doesn't add to this number,
# an error message will be displayed. Just ensures that you give the correct
# worth to all criteria
total: 100

criteria:
  "First criterion":
    # Can be any string, as long as it's unique.
    # If not specified, the stub will be the criterion's name,
    # lowercased and whitespace replaced with dashes
    stub: whatever-stub
    # Any number (even negative). Lowest number is run first.
    # Criteria without indices do not have consistent order
    index: 1
    # A description
    desc: You should do this to fulfil this criterion
    # required point value
    # can be negative
    worth: 50
    # success and failure messages
    # default to "passed" and "failed"
    messages: ["Passed!", "not passed"]
    # This will prevent the criterion from being displayed
    # to the student. Useful if you want hidden requirements 
    # or are grading a test
    # true or false
    hide: false

  # This criterion has all default values
  "Second criterion":
    stub: second-criterion
    index: 100
    desc: ""
    worth: 0
    messages: ["passed", "failed"]
    hide: false
```
