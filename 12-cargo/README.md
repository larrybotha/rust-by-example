# Cargo

- https://doc.rust-lang.org/stable/rust-by-example/cargo.html
- https://doc.rust-lang.org/cargo/
- https://doc.rust-lang.org/book/ch11-00-testing.html

## Takeaways

- cargo is Rust's package management tool, analogous to Javascript's npm, and
  Python's PyPi
- Rust's package registry is crates.io
- cargo offers additional features, in that it is both test-aware and benchmark
  aware

## Dependencies

- `cargo new [name]` by default creates a binary project
- `cargo new [name] --lib` will create a library project
- the manifest file, `Cargo.toml`, allows for dependencies to be defined from
  different locations:
  - crates.io
  - remote URLs, such as GitHub
  - local paths
- running `cargo run` or `cargo build` will download any dependencies if not
  already present

## Conventions

- cargo allows for multiple binaries in a single project, by placing additional
  binaries into a `bin` folder, and referencing them explicitly when compiling

## Testing

- `cargo` supports unit and integration tests out of the box
- unit tests are written in the same files as the modules they test
- integration tests are placed in a `tests/` folder
- tests are run using `cargo test`

  - similarly to Jest, a specific test can be run given the test's name:

    ```shell
    $ cargo test test_foo
    ```

- cargo runs tests in parallel
- the `#[test]` macro turns a function into a unit test

## Build Scripts

- for builds that require additional work, such as compiling other code, cargo
  can run a build script
- there are two ways to allow cargo to run a build script:
  - add a `[package]build = my_file.rs` entry to `Cargo.toml`
  - add a `build.rs` in the project directory
- if a build script exists, it will be run before the project is built
