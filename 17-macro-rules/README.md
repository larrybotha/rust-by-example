# `macro_rules!`

- https://doc.rust-lang.org/stable/rust-by-example/macros.html
- https://doc.rust-lang.org/reference/macros-by-example.html

## Takeaways

- macros allow for metaprogramming in Rust
- macros in Rust are expanded into ASTs, whereas in many other languages marcros
  are processed as string
- macros are created using the `macro_rules!` macro:

  ```rust
  macro_rules! my_macro {
    () => {
      // ...
    }
  }
  ```

- macros are useful for:
  - keeping code DRY - functionality across types may be encapsulated in a macro
  - writing DSLs
  - creating variadic interfaces (`println!` is variadic, but a normal rust
    function cannot be made variadic...?)

### Syntax

- the syntax for macros is described by three ideas:
  - patterns and designators
  - overloading
  - repetition
- designators - a macro's arguments:
  - are prefixed with a `$` sign
  - are annotated by a semicolon followed by a _designator_ - a description
    of the type of the argument
- overloading:
  - a single macro can be overloaded to accept different combinations of
    arguments, in which case it acts similarly to a `match` block
  - each arm of an overload must end with a semicolon
- repetition

  - as with regex, `*` in a macro's arguments indicates 0 or more, and `+`
    indicates 1 or more values

  ```rust
  macro_rules! variadic {
    ($x: expr) => {
      println!("one argument: {}", $x)
    }

    ($x: expr, $($xs: expr), +) => {
      println!("many arguments: {}, {}", $x,)
    }
  }
  ```

### DSLs

- [lazy_static](https://crates.io/crates/lazy_static) and
  [clap](https://crates.io/crates/clap) are good examples of DLSs built using
  macros

## Additional

- the `stringify!` macro will return a `str` representation of any tokens passed
  to it:

  ```rust
  let x = stringify!(2 + 2); // => "2 + 2"
  ```

- bools can be cast to integers

  ```rust
  let x = true as i32; // => 1
  let y = false as i32; // => 0
  ```

- because Rust doesn't support variadic functions, one can't spread a collection
  into a function call
- macros can be called using parens, braces, or square brackets
