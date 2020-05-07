# YAML Spec
Here's the keys allowed when making a yaml file for a batch. Not all fields are required, but it's better to be specific.

Here's the [YAML syntax](https://docs.ansible.com/ansible/latest/reference_appendices/YAMLSyntax.html) so you can learn how to write valid YAML. This isn't the official specification, but it's the easiest guide I found.

When naming your yaml files, you can use `.yml` or `.yaml`. Honestly you can use whatever extension, I don't care, but I use `.yml` for mine.

Quotes around strings are usually not required.

*************

## Minimum Required
```yaml
name: Minimum Batch
criteria:
  Only criterion:
    stub: only-criterion
    worth: 10
```
## Everything
```yaml
# Required
name: Batch Name
# Optional
desc: A short description about your batch

# Required
# You need at least one criteria
criteria:

  # Required
  # Quotes are not required
  Criterion name:

    # Required
    # This is like a human readable ID.
    # Must be unique.
    # This can really be any string, but it's best to keep it short and whitespace-free
    stub: a-unique-stub

    # Required
    # Point value of the criterion
    # This is completely subjective, you give it worth
    # Can be negative
    worth: 15

    # Optional
    # Controls the order the criteria are run
    # Lowest first
    # Can be negative
    index: 5

    # Optional
    desc: More information about this criterion

    # Optional
    # Printed to the console when a criterion passes or fails
    # Defaults to these values
    messages: ["passed", "failed"]

    # Optional
    # if this is true, criterion cannot be printed
    # Defaults to false
    hide: false

  # Here's all the fields without the comments
  Second criterion:
    stub: second-criterion
    worth: 10
    index: 1
    desc: Here's some more about this criterion
    messages: ["passed", "failed"]
    hide: false
```
