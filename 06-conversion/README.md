# Conversion

- https://doc.rust-lang.org/stable/rust-by-example/conversion.html

## Takeaways

- primitive types can be converted to other primitive types using casting
- structs and enums (custom types) are converted to each other via traits

### `From` and `Into`

- the `From` and `Into` traits are linked, the idea being that if a type can be
  converted into another, it can be converted from that type, too
- `From` allows for creating a specific type from another. `String::from` is an
  example of converting from `&str` to `String`:

  ```rust
  let stack_str = "foo";
  let heap_string = String::from(stack_str);
  ```

- to create a type from another type, implement `From` on the target type for
  each type explicitly:

  ```rust
  struct MyType {
    foo: i32
  }

  impl std::convert::From<i32> for MyType {
    fn from(value: i32) -> Self {
      MyType {
        foo: value
      }
    }
  }

  impl std::convert::From<&str> for MyType {
    fn from(value: &str) -> Self {
      MyType {
        foo: value.parse().unwrap_or(0)
      }
    }
  }
  ```

- `Into` is the reciprocal of `From`, and is available via the original type's
  `.into` method when an annotation is provided:

  ```rust
  let x: MyType = 6.into();
  // => MyType { foo: 6 }
  ```

### `TryFrom` and `TryInto`

- `TryFrom` and `TryInto` are similar to the `Try`-less counterparts, except
  that they allow for fallible conversions, returning a `Result`:

  ```rust
  struct LongEnough(bool);

  impl std::convert::TryFrom<&str> for LongEnough {
    type Error = bool;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
      match value.len() >= 5 {
        true => Ok(LongEnough(true)),
        _ => Err(false),
      }
    }
  }

  let short: Result<LongEnough, bool> = "nope".try_into();
  let long: Result<LongEnough, bool> = LongEnough::from("hell yes!");
  ```

### To and From Strings

- custom types can implement `ToString` to make the type convertible to a String
- as in [chapter 2](../02-primitives), however, one should implement `Display`,
  as it implements `ToString` under the hood, while allowing for the type to
  be `print!`ed
- converting to numeric literals from strings is typically done using `parse`:

  ```rust
  let x = "32".parse().unwrap();
  ```

- values may also be cast using the turbofish syntax:

  ```rust
  let x = "12".parse::<i8>().unwrap();
  //          [1]      [2]
  // 1 - parse the string to a numeric type...
  // 2 - using the turbofish syntax to specify the resulting type
  ```

- for parsing custom types into numeric literals, one needs to implement
  `FromStr` for the type

### Additional

- `#[derive(PartialEq)]` allows for evaluating partial equality of values,
  i.e. for types that do not have equality across their entire set, such as in
  Javascript where `NaN !== NaN` for `Number` - Rust has the same for floating
  point decimals. Thus, floating point decimals implement `PartialEq` instead
  of `Eq`

  See http://doc.rust-lang.org/1.64.0/core/cmp/trait.PartialEq.html for
  details

- `sum()` is available as a convenience for `xs.fold(0, |x, acc| x + acc)`
