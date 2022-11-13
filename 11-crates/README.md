# Crates

- https://doc.rust-lang.org/stable/rust-by-example/crates.html

## Takeaways

- crates are _compilation units_ - modules are not compiled, instead, modules
  are inserted into crates at the locations they are imported at compile-time
- crates can be compiled into either libraries, or binaries. By default, `rustc`
  will compile to binaries, but can be configured to compile to a library
  using `--crate-type lib`

### Create a library

- use `cargo new [lib_name] --lib` to create a new library
- the entrypoint of libraries is src/lib.rs
- use `rustc --crate-type lib [input_file].rs` to compile a library
- use `rustc --crate-type lib [input_file].rs --crate-name [crate_name]` to use
  a name different from the file's name
- compiled library files are prefixed with `lib`
- for the example library:

  ```bash
  $ cd my_lib
  $ rustc --crate-type lib src/lib.rs --crate-name my_lib
  ```

### Create a binary

- library crates can be linked when using `rustc` to compile a binary by using
  the `--extern [crate_name]=[library_file_name]` flag
- for the example binary:

  ```bash
  $ cd my_binary
  $ rustc src/main.rs my_lib=../my_lib/libmy_lib.rlib
  $ ./main
  ```
