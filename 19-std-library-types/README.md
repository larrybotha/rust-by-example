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

### Vectors

- like slices, the size of a vector is not known at compile time
- unlike slices, a vector can grow dynamically
- a vector is described by 3 pieces of information:
  - a pointer to the data
  - capacity - how many values the vector can hold, i.e. how much memory is
    reserved for the vector
  - size - how many values the vector is currently holding
- when a vector grows beyond its capacity, it is assigned a larger capacity and
  all of its items may be moved to a new location in memory to maintain
  contiguity of their location in memory
- a vector can be collected from an iterator:
  ```rust
  let range = 0..10;
  let xs: Vec<i32> = range.collect();
  ```
- a _mutable_ vector may be pushed to and popped from
- `Vec::pop` returns an `Option`. If the vector is empty, `.pop` will return `None`
- vectors can be iterated over using `Vec::iter`:

  ```rust
  let xs = vec![1,2,3];

  for x in xs.iter() {
    println!("x: {x}");
  }
  ```

- as in Python, one can use `.enumerate` to get a tuple of the value and its
  index:

  ```rust
  let xs = vec![1,2,3];

  for (i, x) in xs.iter().enumerate() {
    println!("x at {}: {}", i, x);
  }

  ```

- one can mutably iterate over a mutable vector using `Vec::iter_mut`
  - `Vec::iter_mut` is not necessary when mutating via `Iterator::map`, as
    `Iterator::map` returns a new value, and does not mutate the original
    value
- [`Vec` vs slice](https://stackoverflow.com/questions/32571441/what-is-the-difference-between-storing-a-vec-vs-a-slice)
- [Arrays, Vectors, and Slices](https://hashrust.com/blog/arrays-vectors-and-slices-in-rust/)

## Additional

- to determine the number of bytes a value occupies on the stack, one can use
  `std::mem::size_of_val(x)`:

  ```rust
  use std::mem;

  let x = 5;

  println!("x occupies {} bytes on the stack", mem::size_of_val(&x));
  ```

- the address of values can be inspected using `{:p}`:

  ```rust
  let xs = vec![1,2,3];

  println!("address of xs[0]: {:p}", &xs[0]);
  ```
