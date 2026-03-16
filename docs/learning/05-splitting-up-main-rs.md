## Goal
Split main.rs into multiple files to make it easier to navigate and understand.

## What I tried
I simply split the code into 2 groups: data structures and logic. I put the data structures in a new file called `schema.rs` and the logic in a new file called `csv_ops.rs`.

## What broke
Nothing.

## What I learned
 - Import syntax in Rust. One common pattern inside a library crate is to use `use crate::file::item` to import items from other modules in the same crate. The `crate` keyword refers to the current crate, and the `file` is the name of the file without the `.rs` extension. The `item` is the name of the item you want to import. If you want to import multiple items from the same file, you can use curly braces to group them together, like `use crate::file::{item1, item2}`. In contrast, a binary target like `main.rs` typically imports from the library crate using the crate/package name (in my case `oxidate`), for example `use oxidate::csv_ops::...` instead of `use crate::csv_ops::...`.
 - The `mod` keyword is used to declare a module. When you declare a module, Rust will look for a file with the same name as the module in the same directory. For example, if you declare `mod schema;` in `lib.rs`, Rust will look for a file called `schema.rs` in the same directory. If it finds it, it will include it as a module in your crate.
 - Copilot recommended I set up a `lib.rs` file because the roadmap for this project includes Python FFI. In Rust, a `lib.rs` file is used to define a library crate. A library crate is a collection of Rust code that can be shared and reused across multiple projects. By setting up a `lib.rs` file, I essentially expose which parts of my code are public and can be used by other projects. In `lib.rs`, I simply declared the modules that I want to expose as public.

## One snippet that I found interesting
```rust
//! Data structures for this project. This file should not contain any internal
//! dependencies.
```
A documentation convention in Rust is to use `//!` for documenting the enclosing item, i.e., the file that the documenting comment is in. This is different from `///` which is used to document the item that follows it.

## Next step
Implement a function that extracts the first n rows of a CSV file, and returns them as a vector of strings.
