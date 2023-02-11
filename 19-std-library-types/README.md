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

### Strings

- Rust has two types of strings:
  - `String`, which is heap-allocated, and represents a `Vec<u8>`
  - `&str`, which is a reference to stack-allocated `[u8]`, which is also known
    an as a string slice
- `String` and `&str` are always guaranteed to contain UTF-8 characters
- `String` and `&str` have various `.split_` methods, which convert the strings
  into iterators
- `str::chars` will return a `Char` which is an iterator over `char`
- backslashes are used to escape characters in strings. The [Tokens](https://doc.rust-lang.org/reference/tokens.html)
  in the book covers techniques for escaping

### `Option`

- `Option` allows for capturing errors in a program without calling `panic!`

### `Result`

- `Option` helps in showing _that_ there was an error, but `Result` allows for
  us to describe _how_ it errored using the `Err` enum
- chaining a result with `?` will unwrap the result and do one of two things:
  - if the result is `Ok`:
    - unwrap the value
    - allow the function to continue executing
  - if the result is `Err`, the function will bail with that `Err`

### `panic!`

- the `panic!` macro can be used to generate a panic and start unwinding the
  stack, freeing any memory allocated by the thread where the panic was
  generated. Each object that was allocated will have its own destructor called,
  until all allocations are freed
- single-threaded applications that panic will report the panic and exit
- [valgrind](https://valgrind.org/) can be used to evaluate whether an
  executable has freed memory or not once it has exited

### `HashMap`

- `HashMap`s store values by key, similar to dicts in Python, and objects in
  Javascript
- a key in a `HashMap` may be any type that implements the `Eq` and `Hash`
  traits
- `HashMaps` are heap-allocated, and thus growable, like vectors
- `HashMaps`, unlike vectors, can also shrink when they have excess capacity
- one can define a `HashMap` with a starting capacity, but the docs recommend
  allowing capacity to be managed dynamically:

  ```rust
  use std::collections::HashMap;

  let defined_cap = HashMap::with_capacity(6);
  let dynamic_cap = HashMap::new(); // better
  ```

- retrieving values from a `HashMap` returns an `Option`:

  ```rust
  use std::collections::HashMap;

  let hash_map = HashMap::new();

  hash_map.insert("a", "b");

  match hash_map.get(&"a") {
    Some(v) => println!("got {v}"),
    None => println!("got nothing")
  }
  ```

- removing values works similarly:

  ```rust
  use std::collections::HashMap;

  let hash_map = HashMap::new();

  hash_map.insert(1, "a");

  let removed_value = hash_map.remove(&1);

  assert_eq!(removed_value, Some("a"));
  ```

- `HashMap::iter` can be used to iterate over a `HashMap`, and similarly to
  Python's dicts, iterates over tuples of key-value pairs

#### Custom keys

- any value that implements `Eq` and `Hash` can be used as a key for a
  `HashMap`:
  - `bool`
  - integers
  - `&str` and `String`
- `f32` and `f64` don't implement `Hash`, and so can't be used as keys (unless
  one implements `Hash`...?)
- all collections whose types implement `PartialEq`, `Eq` and `Hash` are
  hashable, e.g.:

  ```rust
  use std::collections::HashMap;

  let xs = vec![1,2,3];
  let mut hash_map = HashMap::new();

  hash_map.insert(xs, 0);
  // => {[1,2,3]: 0}
  ```

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

- `Vec::sort` and `Vec::dedup` are useful methods on `Vec`
- assigning `None` to a variable requires the type to be explicitly annotated:

  ```rust
  let none_int = None::<i32>;
  ```

- Rust discourages manually dereferencing values in an iterator by providing a
  `Iterator::copied` method that will perform the same task:

  ```rust
  let xs = vec±[1,2,3];
  let sum_imperative = xs.iter()
    // manuualy dereference - discouraged
    .map(|x| *x)
    .reduce(|acc, x| acc + x);
  let sum_declarative = xs.iter()
    // let Rust dereference
    .copied()
    .reduce(|acc, x| acc + x);

  ```

- `Iterator::fold` is analogous to `Array.reduce` in Javascript -
  `Iterator::reduce`, however, returns an `Option`:

  ```rust
  let xs = vec![1,2,3];
  let maybe_sum: Option<i32> = xs.iter().copied().reduce(|acc, x| acc + x);
  // Rust suggests using `sum` here, too
  let sum = xs.iter().fold(0, |acc, x| acc + x);

  assert_eq!(Some(sum), maybe_sum);
  ```
