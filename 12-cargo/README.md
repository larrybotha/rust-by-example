# Cargo

- https://doc.rust-lang.org/stable/rust-by-example/cargo.html
- https://doc.rust-lang.org/cargo/

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
