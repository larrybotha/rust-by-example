# Primitives

- https://doc.rust-lang.org/stable/rust-by-example/primitives.html
- https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
- https://doc.rust-lang.org/std/#primitives

## Takeaways

- Rust has scalar types, and compound types
- scalar types are the most fundamental types, while compound types are types
  that are composed of scalar types
- scalar types:

  - signed integers - integers with the types `i8` to `i128`, and `isize`.
    Each `i8`, for example, is 8 bits, or 1 byte
    - .e.g `i8` has 2^8 possible values, from `[-128..=127]`
    - integers are by default `i32`
  - unsigned intgers - non-negative integers `u8` to `u128`, and `usize`
    - e.g. `u8` contains `[0..=255]`
  - `isize` and `usize` are numbers with memory allocations based on the
    architecture the application is running on - 32bit vs 64bit machines

    These types are used primarily when indexing a collection:

    ```rust
    let xs = [1,2,3];
    let index: usize = 1;
    let value = &xs[index]; // => 2
    ```

  - floating point numbers, `f32` and `f64`
    - floats are by default `f64`
    - all floating point numbers are signed
  - characters - `char` type Unicode values that occupy 4 bits each
  - boolean values - `true` and `false`
  - unit - `()`

- compound types:
  - arrays, which are fixed in size, and may contain only a single type
  - tuples, also fixed in size, and may contain different types, but always the
    same type at each index
- when running `carbo build --release`, if there is an integer overflow, i.e.
  you have specified a `u8` but a value ends up being 256, Rust will silently
  use _two's complement wrapping_ instead of panicking - it'll wrap the value
  back around to the beginning of valid values. e.g. 256 -> 0

  There are various methods to deal with this - see the Rust book's chapter on
  data types to handle these situations

- division behaves differently depenping on whether you're working with integers
  or floating point numbers:

  ```rust
  println!("integer division: 2 / 3 = {}", 2 / 3); // => floors the result
  println!("floating point division: 2.0 / 3.0 = {}", 2.0 / 3.0); // => returns a decimal
  ```

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

### Tuples

- tuples can be destructured in the same way that they can be in Python
- values in tuples are accessed in the same way as in Python; via zero-indexed
  accessors
- the empty tuples is called unit (`()`), and is implicitly returned by
  functions that don't return anything, much like Javascript's `void` and
  Python's `None`
- Rust can return tuples in a similar manner to Python and Go, but the parens
  are required

  ```rust
  fn my_func() -> (i32, String) {
    (1, String::from("foo"))
  }
  ```

- tuples containing more than 12 values cannot be printed
- tuples with a single value must have a trailing comma to differentiate them
  from literals inside parens:

  ```rust
  let tuple = (1,);
  let
  ```

### Arrays

- arrays are useful when you want data allocated on the stack, or have a fixed
  number of elements to work with
- arrays are stored in contiguous memory, which can be expensive to locate when
  arrays become large
- the length of an array is obtained using `xs.len()`
- array annotations must specify the type and length of the array:

  ```rust
  let xs: [i8; 3] = [1,2,3];
  ```

- because the size of an array is fixed, it can be defined without initially
  binding it:

  ```rust
  let xs: [i8; 3];

  xs = [1,2,3];
  ```

- arrays may also be initialised using an alternative syntax, where the default
  value and size are provided when creating the binding:

  ```rust
  let xs = [3; 5];
  // => [3,3,3,3,3
  ```

- a runtime error will kill the application if one attempts to access elements
  of an array outside of the bounds of the array. This is different from many
  other low-level languages where the application will continue running with
  invalid memory access
- similar to Python's dict lookups, arrays can be safely referenced using
  `[].get` which returns an `Option` which can be matched against:

  ```rust
  let xs = [0; 3];
  let value = match xs.get(100)  {
    Some(x) => {x},
    None => {-1}
  };
  ```

  An alternate syntax to unwrap `Some` is available when `Option`s are returned:

  ```rust
  let xs = [1,2,3];
  let value = xs.get(4).unwrap_or(-1);
  ```

### Slices

- unlike arrays, the length of a slice is not known at compile time, which means
  it is _allocated_ - i.e. allocated to the heap
- the stack representation of a slice is composed of two things - a pointer to
  the data on the heap, and the the size of the data, which is a `usize`
- slices can borrow sections of ararys, and have the signature `&[T]`
  - i.e. when creating a slice, we always borrow from the referenced array

### Additional

- destructuring a tuple struct requires using that type in the destructure:

  ```rust
  struct MyType (i32, i32);

  let x = MyType(1,2);
  let MyType(y,z) = x;

  println!("{x} {y}");
  ```

- `std::mem` can be used to retrieve information about values in memory:

  ```rust
  use std::mem;

  let xs = [0; 10];

  println!("xs in bytes: {}", mem::size_of_val(&xs));
  ```

- Rust's `[].map` doesn't appear to have an index - the index needs to be
  managed via a value outside of the lambda
