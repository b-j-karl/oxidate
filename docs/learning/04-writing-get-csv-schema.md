## Goal
Write a function that returns the schema of a CSV file, including the column names and their inferred types. I will need to investigate the best way of representing this output in Rust. I will also need to research how to infer the types of the columns.

## What I tried
First, I decided I would need a data structure to represent the column schema. I created a `ColumnSchema` struct with two fields: `name` and `dtype`. I used an `enum` to represent supported types.

## What broke
I had a bug due to PEBCAK: `.all()` on an empty iterator returns `true` (vacuous truth), so an empty string `""` was being classified as `Integer` instead of `String`.

## What I learned
I learnt the basic syntax for defining a `struct` and an `enum` in Rust. In writing these objects, I also learnt the proper way to document them using doc comments (`///`). Markdown formatting can be used in doc comments; love it. I decided to go ahead and document the other functions I had already written using doc comments as well. Not sure what the state of practice is in Rust for documentation, but I personally don't see why not.

I went to write a private function, and found at that everything is private by default. To declare a public function, you need to use the `pub` keyword e.g., `pub fn foo(...)`.

Sweet, `zip` exists in Rust too. Instead of `zip(a, b)` like in Python, you can do `a.iter().zip(b.iter())` in Rust.

I learned about 2 new concepts: traits and deriving traits. A trait is a collection of methods that types can implement. They are similar to interfaces in other languages. The `Debug` trait allows us to print the value of an object using the `{:?}` format specifier. The `PartialEq` trait allows us to compare two values for equality using the `==` operator. By deriving these traits for our `ColumnType` enum, we accept the default implementations of these traits provided by the Rust compiler.


## One snippet that I found interesting

Handling `Option<Result<T, E>>` from an iterator. `.next()` returns an `Option` which is Rust's way of representing nullable values. Rust forces us to handles every possibility, so we wrote the `match` expression below. It basically says: if `record` exists and is valid, use it; if there's an error, propagate it; if there's no record, use a default value.

```rust
let first_row = match reader.records().next() {
    Some(Ok(record)) => record,           // got a valid row
    Some(Err(e)) => return Err(e),        // parse error, propagate it
    None => csv::StringRecord::default(), // no rows, use empty default
};
```

An alternative is `.transpose()` which flips `Option<Result<T, E>>` into `Result<Option<T>, E>`, enabling use of `?` for error propagation. We can then call `.unwrap_or_default()` on the `Option` to handle the case where there are no records.

```rust
let first_row = reader.records().next().transpose()?.unwrap_or_default();
```

Although the second version is a cool one-liner, I went with the first version as it is more explicit and easier to understand for someone new to Rust.

---

Okay, 2 snippets that I found interesting.

Checking if all characters in a string are digits using an iterator chain with a closure:

```rust
if value.chars().all(|c| c.is_ascii_digit()) {
    return ColumnType::Integer;
}
```

- `.chars()` converts the `&str` into an iterator of characters.
- `.all(|c| ...)` returns `true` only if the closure returns `true` for every element.
- `|c|` is a closure (anonymous function), `c` is each character in turn. This is sort of like using a lambda function in `map` or `filter` in Python.
- `c.is_ascii_digit()` checks if the character is an ASCII digit.

Gotcha: `.all()` on an empty iterator returns `true`.

## Next step
My single `main.rs` file is getting pretty long and difficult to navigate. It is likely a good time to refactor the code into multiple files. I can take this opportunity to learn a bit about projectr structure in Rust.
