# Rust

### Going through the exercises in the [rust lang book](https://doc.rust-lang.org/book/) 🦀📚



`cargo new <whatever>` creates a new project

`cargo run` compiles then runs the generated executable (path to exe is `./target/debug/<whatever>`)

`cargo check` checks if your code compiles but doesn't produce an executable

`cargo build --release` compiles your code with optimisations (path to exe is `./target/release/<whatever>`)

`cargo doc --open` to build documentation based on your dependencies


### Useful resources

[Rust standard library docs](https://doc.rust-lang.org/stable/std/prelude/index.html#modules)

[Crate registry](https://crates.io/)

### Useful extras

- variables are immutable by default, to make a variable mutable you have to use `mut` 🐶
