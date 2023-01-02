# Error Handling

- https://doc.rust-lang.org/stable/rust-by-example/error.html
- https://doc.rust-lang.org/book/ch09-00-error-handling.html

## Takeaways

- there are 2 types of errors in Rust:
  - recoverable, and non-recoverable
- recoverable errors are the `Err` type in `Result`
- non-recoverable errors are `panic`s
- `panic` is useful for exiting tests
- when a value is optional, or a missing value is not an error, `Option` is
  useful
  - `Option::unwrap` is fine for prototyping, or when you are absolutely
    guaranteed to have a value
  - generally, `expect` is better to use than `unwrap`, as it allows one to
    explicitly deal with the error case
- when something can go wrong, and it's the caller's responsibility to deal with
  it, use `Result`

### `panic`

- `panic`ing will usually result in the following:
  - the stack is unwound (freeing any allocated memory)
  - an error message is printed
  - the program is usually exited

```rust
fn gimme_even(x: i32) {
  if x % 2 != 0 {
    panic!("I want an even number!")
  }

  println!("thanks for the even number")
}
```

### `abort` and `unwind`

- Rust has two `panic` strategies - `abort`, and `unwind`
  - `abort` will exit the program without cleaning up memory
  - `unwind` is the default, and will clean allocated memory before exiting
- `panic` strategies can be set via the command line when compiling:

  ```shell
  rustc my_file.rs -C panic=abort
  ```

- the command line flag must be used in conjunction with code, and can either be
  used by evaluating the config directly, or setting an attribute:

  - by evaluating the config:

    ```rust
    fn do_it() {
      if cfg!(panic="abort") {
        println!("I'm aborting!")
      } else {
        println!("I'm unwinding")
      }
    }
    ```

    This will be built into the binary, and evaluated at runtime

  - by compiling it into/out of the binary:

    ```rust
    #[cfg(panic="abort")]
    fn do_the_thang() {
      // do things when panic aborts
    }

    #[cfg(not(panic="abort"))]
    fn do_the_thang() {
      // do things when panic unwinds
    }
    ```

- to run `cargo` with `panic` set to the _abort_ strategy:

  ```shell
  cargo --config 'profile.dev.panic = "abort"' run
  ```

### `Option` and `unwrap`

- `Option` is an enum which encapsulates the possibility of absence of a value
- A value of `Option` is either `Some`, or `None`, and these variants can be
  handled in one of two ways:
  - explicitly using `match`
  - implicitly using `unwrap` or `expect`
- using `unwrap` will either return the value, or `panic`
- `expect` operates similarly to `unwrap`, with the different that the `panic`
  message can be modified

## Additional

- `not` can be used in attributes:

  ```rust
  #[cfg(not(target_os = "macos"))]
  fn do_the_thing() {}
  ```

- one can use `std::panic::catch_unwind` to catch panics
