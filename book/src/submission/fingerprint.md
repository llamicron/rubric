# Fingerprinting and Security
Security is important for graders. We don't want students grades to be visible, and we don't want students to be able to fake submissions.

## Closed Source
You should consider keeping your graders closed source. While having the source available isn't explicitly dangerous, it would probably be best to keep your grader from your students. Instead, just compile and distribute the executable.

When I write graders, I set them to open a dropbox when I run the grader with a certain command line argument. While technically your students could open the dropbox, this would only start an empty dropbox on their machine. It would be able to accept submission on the off chance someone submitted to it. I would recommend setting the argument to start the dropbox to something not easily guessable. I use `open_sesame` to start the dropbox.

If you want to be absolutely sure that a student doesn't open the dropbox, you can compile a separate grader that only opens the dropbox, separate from the grading bit.

## Fingerprints
When creating a new submission, you can enable a `Fingerprint` for extra verification. You provide a "secret key" (just a random string that you keep secret) that will be attached to the fingerprint. The fingerprint will also collect some basic system information (like platform). When the submission is sent to the dropbox, the fingerprint data will also be sent.

```rust ,noplaypen
let mut submission = Submission::new();
submission.set_fingerprint("secret key. Keep this quiet!");
```

If fairly straightforward to POST a web request with a JSON body. Providing a secret key in the fingerprint helps protect the dropbox from fake submissions. If you recieve a submission that does not contain the secret key, it's probably fake. If you keep the secret key in the source code, be sure the source code is private. The student will not be notified about the fingerprint, so they won't know about the secret key.

The fingerprint will also collect some system information, which is more passive protection. If half of the students submission are from a Linux machine, then it switches to Windows, that may be suspicious. If the assignment is meant to be performed on one system, this might be a red flag. It's ultimately up to you as to what you do with the data.
