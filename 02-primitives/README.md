# Primitives

- https://doc.rust-lang.org/stable/rust-by-example/primitives.html
- https://doc.rust-lang.org/std/#primitives

## Takeaways

- Rust has scalar types, and compound types
- scalar types are the most fundamental types, while compound types are types
  that are composed of scalar types
- scalar types:
  - signed integers - integers with the types `i8` to `i128`, and `isize`. Each `i8`, for example, is 8
    bites, or 1 byte
    - .e.g `i8` has 2^8 possible values, from `[-128..=127]`
    - integers are by default `i32`
  - unsigned intgers - non-negative integers `u8` to `u128`, and `usize`
    - e.g. `u8` contains `[0..=255]`
  - floating point numbers, `f32` and `f64`
    - floats are by default `f64`
  - characters - `char` type Unicode values that occupy 4 bits each
  - boolean values - `true` and `false`
  - unit - `()`
- compound types:
  - arrays, which are fixed in size, and may contain only a single type
  - tuples, also fixed in size, and may contain different types, but always the
    same type at each index

### Literals and operators

- as in Javascript, underscores can make numbers easier to read:

  ```rust
  let x = 200_000_000;
  let y = 0.000_001;
  ```

- numbers may be cast to types when declared:

  ```rust
  println!("{}", 42u32); // <= 42 is a u32
  ```

- boolean operations are the same as in Javascript:

  ```rust
  let AND = true && false;
  let OR = true || false;
  let NOT = !true;
  ```

- bitwise operations are similar to their Javascript counterparts:

  ```rust
  // the bits at each position are both 1
  println!("AND: 1101 & 1010: {:04b}" 0b1101 & 0b1010); // => 1000
  // a bit et each position is 1
  println!("OR: 1101 | 1010: {:04b}" 0b1101 | 0b1010); // => 1111
  // only 1 bit at each position is 1
  println!("XOR: 1101 | 1010: {:04b}" 0b1101 | 0b1010); // => 0111
  // left to the right
  println!("1 << 4: {}" 1u32 << 4);
  // right shfft
  println!("right shift: 0x80 >> 4: {}", 0x80u32 >> 4);
  ```
