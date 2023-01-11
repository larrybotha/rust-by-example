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

### Multiple error types

- it's convenient when `Option` interacts with `Option`, and `Result` with
  `Result`, but the need will quickly arise where we need to be able to
  interact between `Option` and `Result`, and `Result<T, Error1>` and
  `Result<T, Error2>`:

  ```rust
  let xs = vec![];

  // index error
  let invalid_x = xs.first().unwrap();

  // ParseIntError
  invalid_x.parse::<i32>().unwrap();
  ```

- the most basic way to handle an `Option` and a `Result` is to _nest_ one
  inside the other:

  ```rust
  fn do_the_nest(xs: Vec<&str>) -> Option<Result<i32, std::num::ParseIntError>> {
    // .first returns an Option
    xs.first().map(|x| {
      // .parse returns a result
      x.parse::<i32>().map(|n| n * 2)
    })
  }
  ```

- one can switch an `Option` and `Result` using `.map_or`, which is eagerly
  evaluated:

  ```rust
  let option_result = Some(Ok(5)); // Option<Result<i32, ...>>
  let result_from_option = option_result.map_or(
    // if the option contains None, wrap it in Ok
    Ok(None),
    // otherwise, map on the result, wrapping its value in an Option
    |result| result.map(Some)
  ); // Result<Option<i32>, ...>
  let option_from_result = result_from_option.map_or(
    // if the Result is Err, return None
    None,
    // otherwise, map on the Option, wrapping its value in Result
    |option| option.map(Ok)
  ); // Option<Result<i32, ...>>
  ```

- `.map_or` is similar to Python's `.get`, where an non-existent value in a `Dict`
  can be provided with a default

#### Defining an error type

- a good error in Rust:
  - represents different errors with the same type
  - has a good UX for users
  - is easy to compare to other error types
    - e.g. `Err(EmptyVec)` is preferred over `Err("Please use a vector with at least on element".to_owned())`
  - can hold information about the error
    - e.g. `Err(BadChar(c, position))` vs `Err("+ cannot be used here".to_owned())`
  - composes well with other errors
- a custom error in Rust is defined with a unit struct:
  ```rust
  #[derive(Debug, Clone)]
  struct MyCustomError;
  ```
- creating errors is a separate concern to displaying errors:

  ```rust
  #[derive(Debug, Clone)]
  struct MyCustomError;

  impl std::fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "invalid use of the thing")
    }
  }
  ```

- using `Result::ok_or` and `Option::map_err` we can change the error type while
  processing a value:
  ```rust
  let first_doubled = vec!["1", "2", "3"].first()
                .ok_or(MyCustomError)
                .and_then(|x| {
                  x.parse::<i32>()
                    .map_err(|_| MyCustomError)
                    .map(|n| n * 2)
                });
  ```
  In Haskell this is known as _type constructor flipping_ or _type constructor
  reversal_

#### Boxing errors

- code can be simplified by `Box`ing errors. This allows for preserving the
  original errors, but it comes at the expense of error types only being known
  at runtime, instead of being statically determined:

  ```rust
  // Create an alias of Result, where the error is wrapped by Box.
  // Any value that implements Error will be converted via Box::From
  type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

  #[derive(Debug, Clone)]
  struct MyError;

  // add error message
  impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Ooops, something ain't right")
    }
  }

  // implement error
  impl std::error::Error for MyError {}

  let my_result: Result<i32> = vec::<&str>![].first()
    // use our custom error, converting the error into Box<MyError>
    .map_or_else(|| MyError.into())
    .and_then(|s| {
      s.parse::<i32>()
        // convert whatever error str::parse raises to the Box'd version
        .map_or(|e| e.into())
        .map(|n| n * 2)
    });
  ```

#### Other uses of `?`

- `?` was previously described as `.unwrap` or `return Err(err)` - it's more
  accurately described by `.unwrap` or `return Err(From::from(err))` - it will
  convert any error to the expected return type
- instead of having to use `my_option.ok_or_else` or `my_result.map_err` with
  `.into()` to convert errors to the expected return types, one can use `?`
  to do the heavy lifting:

  ```rust
  fn option_thinger(xs: Vec<i32>) -> Result<i32, Box<dyn error::Error>> {
    // let first = xs.first().ok_or_else(|| MyCustomError);
    let first = xs.first()
      // convert to Result, and then unwrap, or return the error type as per
      // the function signature
      .ok_or(MyCustomError)?; // '?' will convert for us here

    Ok(first)
  }

  fn result_thinger(x: &str) -> Result<i32, Box<dyn error::Error>> {
    // '?' here will convert the parse error to the type of error as defined
    // by the function signature
    let n = x.parse::<i32>()?;

    Ok(n)
  }
  ```

#### Wrapping errors

- instead of boxing errors, we could return them in our own error type, defined
  by an enum
- to do this:
  - create an enum for the custom error, with variants for the different types
    of errors
  - implement `std::fmt::Display` for the enum
  - implement `std::error::Error` for the enum
    - for any of the variants that wrap another error, retain that error by
      returning it in a `Some`
  - in order for `.into` or `From::from(err)` to convert from some error into
    one of our error variants, implement `From<OriginalErrorType>` for each
    wrapped error contained in the enum

### Iterating over `Result`s

- `some_iter.map` may fail, in which case we have a few ways of handling the
  error:
  - use `.collect` and let the error occur
  - filter out the errors
  - store the errors
  - partition the iterator, returning a tuple containing the passed and failed
    values

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

- `A.into()`, where `A` is converted to type `B` can also be written
  `B::from(value)`. The convenience is that we don't need to know what `B` is -
  Rust will do the conversion for us, allowing `B` to be changed without having
  to change locations where `.into()` has been used - we know we have `A`, and
  it will be converted into _something_ else
- when aliasing `Result`, one needs to specify `std::result::Result` in order to
  prevent a circular dependency:

  ```rust
  type Result<T> = std::result::Result<T, SomeType>;
  ```

- `Option::ok_or` transforms an `Option` into a `Result`, so to extract a value,
  or return an `Err` from an `Option`:

  ```rust
  let get_result(x: Option<i32>) -> Result<i32> {
    let value = x.ok_or(MyError)?;

    value
  }
  ```

- `my_iterator.partition` accepts a predicate, and results in a tuple of the
  passing and failing values. In Javascript, it would likely be analogous to:

  ```javascript
  const [passing, failing] = xs.reduce(
    (acc, x) => {
      let [passes, failures] = acc;

      if (x) {
        passes = [...passes, x];
      } else {
        failures = [...failures, x];
      }

      return [passes, failures];
    },
    [[], []]
  );
  ```

- `Result::unwrap_err` is analogous to `Result::unwrap`, except that:
  - it unwraps the values contained in the `Err` variant
  - it `panic`s if it encounters an `Ok`
