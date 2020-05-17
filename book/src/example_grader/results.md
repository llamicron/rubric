# The Results
Once a submission is accepted by the submission server, it will create a file called `submissions.csv`. Every submission accepted will be written to this file. This is a simple csv file, and you can open it in Excel or another program to process the data in any way you see fit.

A few warnings:
- The header of the csv file will be written for *the first* submission recieved. If, for some reason, submissions have different data fields, the csv values and header won't match up. You should be sure that submissions all have the same data fields.
- You should try to avoid having commas in criteria names/descriptions, or in your data. You can't really prevent users from entering commas though. When submitting, any commas found will be replaced with semicolons.
- You need to compile the grader on the platform that your students will be running it on. If you want to provide a version for Windows, Linux, and MacOS, I recommend using a [Github workflow](https://help.github.com/en/actions/configuring-and-managing-workflows/configuring-a-workflow) to build for each platform.

## Wrapping up
This has been a very simple example of a very simple grader. The last thing to do is distribute the program to your students. when you compile for release mode, it will generate an executable in `target/release/[program_name]`

```
$ cargo build --release
```

We usually write our labs as repositories on Github that the student can clone and follow, so we put the grader in the repo.
