# Types

- https://doc.rust-lang.org/stable/rust-by-example/types.html

## Takeaways

### Casting

- Rust does not support implicit type casting / coercion, as Javascript does
- explicit casting can be done using the `as` keyword:

  ```rust
  let x = 5_i8; // <= a suffix may be used to specify a value's type, instead of
                // a type annotation
  let y = 5 as f32;
  ```

- decimals and floats can be cast between each other without issue
- only `u8` may be cast to char
- casting to unsigned types will add or subtract `T::MAX + 1` to the type until
  the value fits in the new type:

  ```rust
  println!("300 as u8: {}", 300_i32 as u8); // subtract 256 until <= 255
  // => 44 = 300 - 256, or 300 % 256
  println!("-300 as u8: {}", -300_i32 as u8); // add 256 until >= 0
  // => 212 = -300 + 256 + 256
  ```

- casting `NAN` to integers appears to always return 0

### Additional

- the min and max of numeric literals can be obtained as values on the types:

  ```rust
  println!("min i8: {}", i8::MIN);
  ```

- `NAN` exists on floating point numeric literals:

  ```rust
  println!("f32::NAN: {}", f32::NAN);
  println!("f64::NAN: {}", f64::NAN);
  ```

- the size of a value in bytes can be retrieved using `std::mem::size_of_val`:

  ```rust
  println!("size of i8: {}", std::mem::size_of_val(i8)); // => 1
  //                         [1]  [2]     [3]
  // 1 - crate
  // 2 - module
  // 3 - function
  ```

- attributes can be scoped to functions:

  ```rust
  struct Unused; // <= raises linting error

  fn scoped_attribute() {
    #![allow(dead_code)]

    struct LocalUnused; // <= no linting error raised
  }
  ```
