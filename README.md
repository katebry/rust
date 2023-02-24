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

[Keywords in Rust](https://doc.rust-lang.org/stable/book/appendix-01-keywords.html)

[Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)

### Useful extras

- variables are immutable by default, to make a variable mutable you have to use `mut` 🐶
- `const`(ants) are always immutable, naming convention is to use all caps, e.g. `const USE_ALL_CAPS: String`
- variables declared with `let` can be shadowed 👻, e.g. `let x = 5; let x = x + 1` in this case, the end value of `x` is 6 
- rust is a statically typed language, which means it must know the types of all variables at compile time
- rust has 4 primary scalar types: integers, floating-point numbers, Booleans and characters
- unsigned numbers are positive 😊😄🤗 (and have a `u` in the typing)
