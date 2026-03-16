## Goal
Implement a function that extracts the first n rows of a CSV file, and returns them as a nested vector of strings.

## What I tried
I simply wrote and tested the function this time around, with some help from Copilot for finding the relevant APIs to use.

## What broke
Initially, I forgot the make the new function public. I have to do this for it to be accessible from `main.rs`.

## What I learned
- `.take(n)` is a method that works on iterators, which allows you to take the first `n` items from the iterator.
- `.collect()` is a method that transforms an iterator into a collection, such as a vector.
- Slicing syntax in Rust is `[start..end]`, where `start` is the index of the first item you want to include, and `end` is the index of the first item you want to exclude. So if you want to include the first 5 items, you would use `[0..5]`. If you want to include all items from index 1 onwards, you would use `[1..]`.

## One snippet that I found interesting
```rust
head.push(record.iter().map(|s| s.to_string()).collect());
```
Here is a breakdown of this line of code. So, `record` is a `StringRecord`, which is basically a vector of strings representing a row of the CSV file. I call `.iter()` on it to get an iterator where each item is a string slice (`&str`) representing a value in the row. I use a closure with `.map()` to convert each string slice into a `String` and finally, I call `.collect()` to transform the iterator of `String`s into a `Vec<String>`. This feels like a list comprehension in Python, but with more explicit steps.

## Next step
Put them all together and write an `inspect` CLI command.
