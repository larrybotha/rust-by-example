# Crates

- https://doc.rust-lang.org/stable/rust-by-example/crates.html

## Takeaways

- crates are _compilation units_ - modules are not compiled, instead, modules
  are inserted into crates at the locations they are imported at compile-time
- crates can be compiled into either libraries, or binaries. By default, `rustc`
  will compile to binaries, but can be configured to compile to a library
  using `--crate-type lib`
