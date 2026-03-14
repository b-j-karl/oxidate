## Goal
Set up a CI pipeline.

## What I tried
I implemented a very minimal CI pipeline. It just checks that the code compiles, runs the tests, checks that the code is formatted, and runs Clippy.

## What broke
My first pipeline failed! The main.rs file that I had filled with "Hello, World!" was not formatted according to rustfmt's default settings, so the `cargo fmt -- --check` step failed. I had to run `cargo fmt` locally to fix the formatting issues and then push the changes to get the pipeline to pass.

## What I learned
Basic come pre-installed with Rust, which is great. Linting, formatting, and testing tools were all simple `cargo` commands that I could easily add to the CI pipeline.
I also leanred about the CARGO_TERM_COLOR environment variable, which can be set to "always" to force Cargo to output colored text in CI logs. This makes it easier to read and understand the output of the CI pipeline. I decided to use this for my pipeline.

## One snippet that I found interesting
Claude suggested using the Swatinem/rust-cache GitHub Action to cache compiled dependencies between runs, which is a great idea to speed up the CI pipeline.
```yaml
- uses: Swatinem/rust-cache@v2
```
This GitHub Action caches compiled dependencies between runs. Very nice.

## Next step
I am itching to get my hands dirty with some actual Rust (I am so funny). My project idea is a command line tool for interacting with CSV files. I will start by writing a simple function that inspects a CSV file and prints out some basic information about it, like the number of rows and columns, the names of the columns, and the data types of each column.
