# Formatted Print

- https://doc.rust-lang.org/stable/rust-by-example/hello/print.html
- https://doc.rust-lang.org/std/fmt/

## Takeaways

- Rust's `format!` macro is heavily inspired by, and similar to, Python's
  `str.format`
  - conventions for named parameters, indentation, and even keyword arguments
    are familiar to those who work with Python
- `format!` writes formatted text to a `String`
- `print!` prints to stdout
- `eprint!` prints to stderr
- `:` in formatted strings indicates the start of the format spec - see
  [syntax](https://doc.rust-lang.org/std/fmt/#syntax)
  - everything after the `:` in a `{...}` format block describes the trait used
    to interpet the block, the indentation, decimal precision, and
    transformations
- format characters after the `:` indicate the format to transform values to:
  - `{}` - `Display` trait
    - implementing `Display` for a struct is similar to implementing `__str__`
      for a class in Python
  - `{:?}` - `Debug` trait
    - implementing or deriving `Debug` is akin to implementing `__repr__` in
      Python
  - `{:b}` - binary
  - `{:o}` - octal
  - `{:x}` - hexadecimal lowercase
  - `{:X}` - hexadecimal uppercase
  - `{:#formatter$}` - where `formatter$` is one of the above - uses the
    formatter's alternate syntax to format the value
- named arguments in the format spec require a `$` appended to them:

  ```rust
  let width = 5;

  println!("{:width$}", 42); // => 00004
  ```

### Debug

- any type that wants to be formatted as a string needs to implement a trait
  from `std::fmt`
- all types can derive `std::fmt::Debug` automatically

### Display

- unlike `std::fmt::Debug`, `std::fmt::Display` needs to manually implemented
- `std::fmt::Display` gives full control over how a value is rendered
- implementing `Display` automatically implements the `ToString` trait on the
  type, adding the `to_string` method to instances:

  ```rust
  struct MyStruct {}

  impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "hello there", &self.bar)
      // [1][2] [           3          =

      // 1 - write the given formatted string at 3
      // 2 - to the given output stream, f
    }
  }

  let a = MyStruct {};
  let a_string = a.to_string();

  println!("{a}"); // => hello there
  ```

- Rust doesn't implement `Display` on any types in the `std` library - there's
  no way to implement `Display` consistently on generic types such as
  `Vec<T>`, thus it's not implemented for any type
- related to generics, `Display` can be implemented on any non-generic type
- as with `Display`, `{:b}` requires `fmt::Binary` to be manually implemented on
  the type

### Additional

- `#[allow(dead_code)]` allows for defining structs that aren't used:

  ```rust
  #[allow(dead_code)]
  struct UnusedStruct {}
  ```
