## Goal

Set up a new Rust project called "oxidate."

## What I tried

First, I installed Rust using the official installer, rustup. Then, I followed the instructions on the Rust website to create a new project using Cargo, Rust's package manager. I ran the command `cargo hello-rust` in my terminal, which created a new directory with the necessary files for a "Hello, World" Rust project. I renamed the project to "oxidate" by changing the name field in the Cargo.toml file and renaming the directory accordingly.

## What broke

I used the wrong command to create the project, `cargo hello-rust`, instead of the proper `cargo new <project_name>`. I had to manually rename things afterward.

## What I learned

The proper way to create a new project is to run `cargo new <project_name>`, which sets up the project with the correct name from the start. So far, I have been impressed by Rust's documentation. Let's see if this impression holds up!

## One snippet that I found interesting

```sh
cargo new oxidate
```

This single command scaffolds the entire project with the right name, directory structure, and a default `Cargo.toml`.

## Next step

Add formatting and linting tools to the project. I want to do this early on, so I can get used to writing code that adheres to Rust's style guidelines from the beginning.
