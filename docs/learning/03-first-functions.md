## Goal
Write 2 functions: one that gets the number of rows in a CSV file, and another that gets the number of columns.

## What I tried
I started off by configuring `copilot-instructions.md` to be more focused on learning. This turned copilot into an incredibly useful learning tool. I then wrote the `get_num_rows` and learnt about the ownership model, Error handling with `Result` and `?`, and testing conventions in Rust. Copilot was a huge help in explaining these concepts to me. Then, I turned copilot off and wrote the `get_num_cols` function myself.

## What broke
I didn't run `cargo clippy` before committing, which caused the CI to fail. I found one warning repeated twice.
```
warning: this expression creates a reference which is immediately dereferenced by the compiler
 --> src\main.rs:5:33
  |
5 |     let num_rows = get_num_rows(&csv_path).unwrap();
  |                                 ^^^^^^^^^ help: change this to: `csv_path`
  |
```
This was because I was passing a reference to `csv_path` when calling `get_num_rows`, but `get_num_rows` already declared in its function header that it was borrowing `csv_path`. So I was effectively borrowing a reference, which is unnecessary. Basically, if the ampersand is already in the function header, you don't need to use it when calling the function.

## What I learned
Pass by reference using `&`. If a function takes a reference to an input variable, the function effectively borrows the value of that parameter. This allows the caller of the function to retain ownership of the variable. If an `&` is not used, the function takes ownership of the variable and the caller can no longer use it after calling the function. As a rule of thumb, I will always try to "borrow" input parameters to functions using `&` unless I have a specific reason to take ownership of the variable.

The concept of ownership is core to rust, and is what allows rust to have memory safety without a garbage collector.

`cargo-edit` is a super useful dependency management tool like `pip` or  `uv`. It allows you to add dependencies to your project using the command line e.g., `cargo add csv`.

Functions that can fail should return a `Result` type e.g., `Result<i32, std::io::Error>`. This makes the error explicit to the type system. When calling a function that returns a `Result`, you can use the `?` operator to propagate errors up the call stack. The alternative to using `Result` and `?` is to use `.unwrap()`. This causes the programme to panic if an error occurs i.e., it will crash.

The caller of the function that returns a `Result` can call `.unwrap()` on it to either get the successful value or cause a panic if an error occurred. This is only recommended for prototyping or when you genuinely want the programme to crash if an error occurs.

Rust's convention for unit tests is to put them in a `tests` module at the bottom of the file. The `#[cfg(test)]` is a conditional compilation attribute that tells the compiler to only compile the test code when running tests. The `#[test]` attribute marks a function as a test case.

`mod` declares a module, which is a file-like abstraction for grouping related code.

I originally set the return type of `get_num_rows` and `get_num_cols` to `i32`. However, Copilot recommended changing it to `usize`, an unsigned integer type. This makes sense because the number of rows and columns cannot be negative, and `usize` is what is returned by the underlying `csv` crate.

## One snippet that I found interesting
```rust
fn get_num_cols(csv_path: &Path) -> Result<usize, csv::Error> {
    let mut reader = csv::Reader::from_path(csv_path)?;
    // .headers() returns a reference to a StringRecord of the headers. A StringRecord
    // is basically a vector if strings representing a row of the CSV.
    Ok(reader.headers()?.len())
}
```
This is basically the first Rust function I have written alone. It uses a lot of concepts I have only just learned about. Here is a quick breakdown of the function:
 - `csv_path: &Path` tells the compiler this function is borrowing `csv_path` instead of taking ownership of it.
 - `-> Result<usize, csv::Error>` tells the compiler this function returns a `Result` type that is either a `usize` or a `csv::Error`.
- `csv::Reader::from_path(csv_path)?` creates a new CSV reader from the file path. The `?` operator propagates any errors that occur up the call stack.
- `reader.headers()?.len()` gets the headers of the CSV file, which is a `StringRecord`, and returns its length as a `usize`. The `?` operator again propagates any errors that occur when getting the headers.

## Next step
Implement function to extract CSV schema. The `.headers()` method which I discovered here will be useful.
