# Error Handling

- https://doc.rust-lang.org/stable/rust-by-example/error.html
- https://doc.rust-lang.org/book/ch09-00-error-handling.html
- https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html

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

#### Unpacking `Option` with `?`

- similar to Javascript's optional chaining, one can unpack a value of type
  `Option` using `?`:

  ```rust
  fn maybe_plus_one(x: Option<i32>) -> Option<i32> {
    x? + 1
  }
  ```

  If `x` is `None` above, `None` is returned by `maybe_plus_one`. This is true
  for any function using `?` to unpack `Option` - the compiler requires that
  functions using `?` either return `Option` or `Result`

- `?` can also be used for chaining:

  ```rust
  struct A {
    b: Some(B)
  }

  struct B {
    c: Some(C)
  }

  struct C {
    value: i32
  }

  let x = A { b: Some(B { c: Some(C { value: 5 }) }) };
  let value = x.b?.c?.value; // Some(5), otherwise None
  ```

#### Combinators

- see [Rust's reference for combinators](https://doc.rust-lang.org/reference/glossary.html#combinator)
- `Option` is a functor, with `.map` operating in the same way as `Array.map` in
  Javascript, always returning an `Option`
- the type of `Option` can change between maps
- `and_then` is equivalent to `flatmap` in other languages, except that a
  handler is required. To flatten without manipulating the contents of the
  `Option`, one can use `.flatten`
- `.and_then` will unwrap the `Option`, pass the value into a function or
  closure, and then ensure that the resulting `Option` is not nested
- `Option::and_then` is equivalent to `Option::map().Option::flatten`

#### Unpacking options and defaults

- `Option` has a few methods that allows us to unpack values and deal with
  defaults:
  - `.or()` - eagerly unpack `None` with a default
  - `.or_else()` - lazily unpack `None` with a default, using a closure
  - `.get_or_insert()` - eagerly unpack the value, otherwise insert the given
    value into the option
  - `.get_or_insert_with()` - lazily unpack the value, otherwise insert a value
    into the option given a closure
- `.or()` does not mutate the original value, so it expects an `Option`
- `.get_or_insert()` mutates the original value, so it expects only the value
  that `Option` will hold

### `Result`

- `Result` is similar to `Option`, except that it deals primarily with _possible
  errors_ instead of the _absence of values_
- for `Result` we have:
  - `Ok(T)` which is analogous to `Some(T)`
  - `Err(E)` which is analogous to `None`
- `Result` has many methods and combinators that operate in a similar manner to
  methods and combinators of `Option. e.g. `.unwrap`will either yield a value, or `panic`
- a `Result` is returned by many methods and functions where a value cannot be
  processed, such as with `str::parse`:

  ```rust
  let ok_result = "4".parse();
  let err_result = std::panic::catch_unwind(|| "foo".parse()).unwrap();
  ```

- `main` can either return unit, or `Result`:

  ```rust
  use std::num::ParseIntError;

  fn main() -> Result<(), ParseIntError> {
    let x = "5";
    let number = match x.parse::<i32>() {
      Ok(n) => n,
      Err(e) => return Err(e)
    }

    println!("{}", number);

    Ok(())
  }
  ```

- as with `Option`, matching on `Ok` and `Err` for `Result` is cumbersome, and
  we can instead use `.map` and `.and_then` as a terse alternative
- early returns paired with `match` blocks can make code easier to read in some
  cases than using `.and_then`
- `?` allows us to attempt to extract the value out of a `Result` without
  `panic`ing. `?` works like `.unwrap`, but it prevents the application from
  `panic`ing:

  ```rust
  let result = "t".parse::<i32>()?; // => Result<i32, std::num::ParseIntError>
  ```

- before `?` there was the `try!` macro, which did the same thing:

  ```rust
  let result = try!("t".parse::<i32>());
  ```

  `try!` has been deprecated, and `?` is now the recommended approach

## Additional

- `not` can be used in attributes:

  ```rust
  #[cfg(not(target_os = "macos"))]
  fn do_the_thing() {}
  ```

- one can use `std::panic::catch_unwind` to catch panics
- as in Javascript, when mapping, and the value being operated on will be passed
  to a function or constructor directly, there's no need to use a closure:

  ```rust
  struct MyTuple(i32);

  // redundant closure
  let verbose = Some(6).map(|v| MyTuple(v));
  // passing the value directly into the constructor
  let terse = Some(6).map(MyTuple);
  ```

- one can deconstruct values at the argument level:

  ```rust
  struct Thing(i32);

  let x = Option<Thing(6)>;
  let x_doubled = x.map(|Thing(v)| => v * 2);
  ```

- Rust has a built-in `identity` function:

  ```rust
  let x = Some(6);
  let y = x.map(std::convert::identity);

  assert_eq!(x, y);
  ```

- `i32` implements `FromStr`, which
- aliases are convenient for describing verbose types:

  ```rust
  type ComplexResult = Result<i32, std::num::ParseIntError>;

  fn do_something() -> ComplexResult {
    // ...
  }
  ```
