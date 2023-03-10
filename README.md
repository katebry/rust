# Rust

### Going through the exercises in the [rust lang book](https://doc.rust-lang.org/book/) 🦀📚

### Cargo commands

`cargo new <whatever>` creates a new project

`cargo run` compiles then runs the generated executable (path to exe is `./target/debug/<whatever>`)

`cargo check` checks if your code compiles but doesn't produce an executable

`cargo build --release` compiles your code with optimisations (path to exe is `./target/release/<whatever>`)

`cargo doc --open` to build documentation based on your dependencies

`cargo new <whatever> --lib` to create a library

`cargo test` to run tests

`cargo test --help` to display the options available when testing

`cargo test -- --test-threads=1` to manipulate the number of threads used (and in this case make it so tests run one at a time aka avoiding *parallelism*)

`cargo test -- --show-output` to see the printed values within a function during a test

`cargo test -- --ignored` to run only ignored tests

`cargo test -- --include-ignored` to run all tests


### Useful resources

[Rust standard library docs](https://doc.rust-lang.org/stable/std/prelude/index.html#modules)

[Crate registry](https://crates.io/)

[Keywords in Rust](https://doc.rust-lang.org/stable/book/appendix-01-keywords.html)

[Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)

[Ownership](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)

[Modules Cheat Sheet](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

[Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

[Vectors](https://doc.rust-lang.org/stable/nomicon/vec/vec.html)

[SipHash](https://en.wikipedia.org/wiki/SipHash)

[Buffer Overread](https://hashrust.com/blog/memory-safey-in-rust-part-1/)

[Generic Types](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html)

[Monomorphisation](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html)

[Lifetimes](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html)

[Lifetime Elision](https://doc.rust-lang.org/nomicon/lifetime-elision.html)

[Testing](https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html)

[Unstable Testing](https://doc.rust-lang.org/stable/unstable-book/library-features/test.html)

### Useful info 

- variables are immutable by default, to make a variable mutable you have to use `mut` 🐶
- `const`(ants) are always immutable, naming convention is to use all caps, e.g. `const USE_ALL_CAPS: String`
- variables declared with `let` can be shadowed 👻, e.g. `let x = 5; let x = x + 1` in this case, the end value of `x` is 6 
- rust is a statically typed language, which means it must know the types of all variables at compile time
- rust has 4 primary scalar types: integers, floating-point numbers, Booleans and characters
- unsigned numbers are positive 😊😄🤗 (and have a `u` in the typing)
- `char` literals use single quotes, whilst string literals use double: `let response = "they do indeed"`
- rust has two primitive compound types, `tuples` and `arrays` - tuples have a fixed length, once declared they cannot grow or shrink in size & they can contain a mixture of types
- arrays have a fixed length, every element of the array must be of the same type
- rust is an expression based language 👄💬 (expressions evaluate to a resultant value)
- expressions do not include ending semicolons - if you add a semicolon to the end of an expression, you turn it into a statement & it will not return a value
- a slice is a kind of reference, so it does not have ownership
- a struct is a custom data type that lets you package together and name multiple related values that make up a meaningful group - a struct is like an object's data attributes
- all functions defined within an `impl` block are called *associated functions* because they're associated with the type named after the `impl`
- *binary crates* are programs you can compile to an executable that you can run
- *library crates* don't have a main function and they don't compile to an executable (this type of crate is also known as a *library*)
- a *package* is a bundle of one or more crates, it contains a *Cargo.toml* file that describes how to build said crates
- code within a module is private by default
- you should use *idiomatic `use` paths* - that means specifying the parent module when calling the function to make it clear that the function being used isn't locally defined
- when bringing in `structs` and `enums` however, you can specify the full path, e.g. `use std::collections::HashMap;` then within your function you can call `HashMap::<whatever>`
- you can use the `glob` operator to bring all public items defined in a path into scope, e.g. `use std::collections::*;`
- when creating new Vectors, you can use the `vec!` macro, e.g. `let v = vec![1, 2, 3];`
- many of the same operations available with `Vec<T>` are available with `String` as well because `String` is implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions and capabilities
- you can use `contains` for searching in a string and `replace` for substituting parts of a string with another string
- all values and keys within a `hashmap` must have the same type
- rust has two types of errors: `Result<T, E>` for recoverable errors and the `panic!` macro for unrecoverable errors
- when a `panic` occurs the program starts `unwinding`, which means rust walks back up the stack and cleans up the data from each function it encounters; this a lot of work so you can circumvent it by setting `[profile.release]/n panic = 'abort'` in your `Cargo.toml` 
- *propagating errors*: when a function's implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do 
- generic types won't make your program run any slower than it would with concrete types - rust accomplishes this by performing *monomorphization* (the process of turning generic code into specific code by filling in the concrete types that are used when compiled)
- *traits* in rust are similar to *interfaces* in other languages
- every reference in rust has a *lifetime*, which is the scope for which that reference is valid - the main aim of lifetimes is to prevent *dangling references* (these cause a program to reference data other than what's intended)
- *lifetime syntax* is about connecting the lifetimes of various parameters and return values to functions; once they're connected, rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety
- rust has a *borrow checker* that compares scopes to determine whether all borrows are valid
- you can add an `#[ignore]` flag to tests to prevent them from running
- *unit tests* are small and focused, they test one module in isolation at a time and can test private interfaces
- *integration tests* are entirely external to your library and use your code in the same way any other external code would, using only the pulic interfaces and potentially exercising multiple modules per test
- when you use the `#[cfg(test)]` annotation you tell rust to compile and run the test code only when `cargo test` is run (not `cargo build`)
- only *library* crates expose functions that other crates can use; *binary* crates are meant to be run on their own

### Ownership

The main purpose of ownership is to manage heap data

1. Each value in Rust has an *owner*
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped
