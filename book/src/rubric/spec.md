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
# -- Rubric Details --
# Required name
name: My rubric
# Optional description. Gets shown to the student when grading
desc: Description of my rubric
# Sanity check. If the sum of all criteria doesn't add to this number,
# an error message will be displayed. Just ensures that you give the correct
# worth to all criteria
total: 100





# -- Deadline verification --
# All of these are optional
#
# The optional submission deadline. Must be in this format
#   YYYY-MM-DD HH:MM:SS
# It will use the current local timezone.
# If a submission is created after this time, the late flag will be true.
# If you enter a deadline from the past, you will be warned during compilation, but
# compilation will continue.
deadline: 2020-05-21 23:59:59
# If this is set to false, the submission will always have a 
# grade of 0 unless manually changed. Defaults to true (allowing late submission).
# A submission *can* still be submitted if this is false, but its grade will be 0
# and the late flag will be true.
# If this is true, `deadline` functions exactly like `final_deadline`
allow_late: true
# This amount will be subtracted from the grade if the submission is late
late_penalty: 10
# This will subtract this many points per day after the deadline.
# One second after the deadline is the first late day, so -5 points.
# 24 hrs + 1 second after the deadline is the second late day, so -10.
# You probably shouldn't use both this and "late_penalty", as they will 
# both take effect if you provide both.
late_penalty_per_day: 5
# Same format as deadline.
# If this is provided, absolutely no submissions will be graded
# after this time. This is mostly used when you want to set a 
# soft deadline (with `deadline`), then late penalties per day, 
# then a hard deadline.
final_deadline: 2020-05-24 23:59:59




# -- Criteria --
criteria:
  "First criterion":
    # The name of the corresponding function. This should be unique.
    # If not specified, the function name will be the criterion's name,
    # lowercased and whitespace replaced with dashes. But it's best to be
    # explicit about this.
    # Should match the function name exactly.
    # Because this is unique, it is used to find criteria within a rubric.
    func: whatever_func
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
    func: second_criterion
    index: 100
    desc: ""
    worth: 0
    messages: ["passed", "failed"]
    hide: false
```
