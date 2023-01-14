# Std library types

- https://doc.rust-lang.org/stable/rust-by-example/std.html
- https://doc.rust-lang.org/std/

## Takeaways

- in addition to primitives, i.e. scalar types and compound types, Rust's `std`
  library provides many custom types which expand on the primitives

### `Box`, stack, and heap

- in Rust, all values are stack-allocated by default
- `Box` allows one to heap-allocate a value
- when a `Box`d value goes out of scope:
  - the `Box`s destructor is called
  - the internal value is destroyed
  - the memory on the heap is freed
- `Box<T>` is a smart pointer to a heap-allocated value `T`
- a `Box`d value can be dereferenced using `*`, which removes only 1 layer of
  indirection

## Additional

- to determine the number of bytes a value occupies on the stack, one can use
  `std::mem::size_of_val(x)`:

  ```rust
  use std::mem;

  let x = 5;

  println!("x occupies {} bytes on the stack", mem::size_of_val(&x));
  ```
