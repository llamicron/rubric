# File System - `helpers::fs`
File system operations. Don't get this confused with `std::fs`. It's not the same.

## `file_exists`
Returns true if the file (or directory!) exists. Relative to the file that the compiled executable is being run in. If you're paranoid, use [`std::fs::canonicalize`](https://doc.rust-lang.org/std/fs/fn.canonicalize.html) to get a full path.
```rust ,noplaypen
assert!( fs::file_exists("Cargo.toml") );
```

## `file_contains`
Returns true if a file contains the provided string. Case sensitive.

```rust ,noplaypen
assert!( fs::file_contains("Cargo.toml", "version") );
```
