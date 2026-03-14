## Goal

Set up a linter and formatter for the Rust project.

## What I tried
I did some research on the state of linters in Rust and found that the most popular one is Clippy. It comes pre-installed with Rust via rustup so I didn't have to do anything to set it up. Clippy has multiple severity levels for its lints:
 - "allow" - the lint is ignored.
 - "warn" - the lint is reported as a warning, but it doesn't prevent compilation.
 - "deny" - the lint is reported as an error, and it prevents compilation.
 - "forbid" - the lint is reported as an error, and it prevents compilation. Additionally, it cannot be overridden by a lower severity level in a more specific scope. (to override a lint in rust you can use attributes like `#[allow(lint_name)]` or `#[warn(lint_name)]` on specific modules, functions, or even individual lines of code to change the severity level.)
 Clippy also has lint groups; a concept that is familiar to from ruff, the Python linter.

Rust also has a built-in formatter called rustfmt, which is also included with rustup. It can be configured using a rustfmt.toml file in the project directory. Most threads I read recommended minimal configuration for rustfmt, so I decided to only set the edition and max_width fields for now. The edition field specifies the Rust edition to format for, and the max_width field specifies the maximum width of formatted code before it starts breaking lines.

## What broke
Nothing, I hope.

## What I learned
I learned that Rust has a built-in linter and formatter that are both included with the standard Rust installation. This is great. It saved me from having to decide between, install, and configure additional tools. I also learned about the different severity levels for Clippy lints and how to configure them in the Cargo.toml file.

## One snippet that I found interesting

```toml
[lints.clippy]
# Deny these - they catch real bugs
suspicious = "deny" # Suspicious code that may be a bug.
perf = "deny" # Code that is not necessarily wrong, but could be written in a more efficient way.
correctness = "deny" # Code that is definitely outright wrong or useless.

# Warn on style issues
style = "warn" # Code that could be more idiomatic, but is not necessarily wrong.
```
This is the Clippy configuration I decided to start with. I want compilation to fail if I have any lints that are categorized as "suspicious", "perf", or "correctness". I want to be warned about style issues, but I don't want them to prevent compilation.


## Next step

Set up CI. I at least want the linter and formatter to run on every pull request, so I will set up GitHub Actions to do that. Once I have tests set up, I will also add a step to run the tests in CI.
