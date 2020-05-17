# TestData
The `TestData` type is simply a type alias to the [`HashMap`](https://doc.rust-lang.org/beta/std/collections/struct.HashMap.html) type. Any methods or functionality that `HashMap` provides is also on `TestData`.

This field is part of a `Submission` and is meant to hold 2 things:
1. Data you need in the final submission, and
2. Data you need from within a [criteria test](../criteria/test.md).

The only restriction is that the `TestData` type is equivilent to a `HashMap<String, String>`, meaning both the keys and values must be a `String` type.

## Creating
The best way to create a `TestData` bundle is through the `data!` macro.

```rust ,noplaypen
let data = data! {
    "key" => "value",
    "key2" => "value 2"
};

assert_eq!( data["key"], "value" );
```

The `data!` and the `prompt!` macro work very well together. You can read about the `prompt!` macro in [the next section](./prompt.md).
