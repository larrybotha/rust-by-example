# Flow of Control

- https://doc.rust-lang.org/stable/rust-by-example/flow_control.html
- https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html

## Takeaways

### if / else

- conditions in if / else statements in Rust don't need to be contained in
  parens, similar to Python, and different to Javascript
- if / else conditionals are expressions - they can evaluate to a result, and
  thus be used for assignment to variables. This mitigates the hell of nested
  ternaries in Javascript:

  ```rust
  let x = 5;
  let result = if x >= 0 && x < 1 {
      "foo"
  } else if x >= 1 && x < 2  {
      "bar"
  } else if x >= 2 && x < 3 {
      "baz"
  } else {
      "quux"
  };
  ```

- all branches in if / else conditionals must evaluate to the same type

### `loop`

- `loop` allows for looping infinitely in Rust - it is equivalent to `while true`
- as in other languages, `break` will exit a loop, and `continue` will move
  execution to the next iteration

#### Nesting and labels

- as in Javascript, one can label loops to specify which loop should be affected
  when breaking or continuing:

  ```rust
  let mut count_outer = 0;

  'outer: loop {
      count_outer += 1;

      let mut count_inner = 0;

      'inner: loop {
          count_inner += 1;

          if count_outer % 2 == 0 {
              continue 'outer;
          }

          if count_inner >= 3 {
              break 'inner;
          }
      }

      if count_outer >= 10 {
          break 'outer;
      }
  }
  ```

#### Returning from loops

- as with if / else conditionals, a loop can be used to resolve to a value, and
  thus for variable assignment, by providing a value after `break`:

  ```rust
  let count = 0;
  let result = loop {
    count += 1;

    if count > 5 {
        break count;
    };
  };
  ```

### `while`

- `while` in Rust is similar to other languages, with the same syntax as `loop`

### `for` and range

- the `for in` syntax, similar to Python's, is used to iterate over `Iterator`s
- a range is an `Iterator`:

  ```rust
  // 0,1,2,3,4,5
  for x in 0..6 {
      // ...
  }
  ```

- collections, such as arrays, slices, and vectors, can be converted to
  `Iterator`s using a few methods:
  - `.into_iter` - will move ownership of each value into the loop,
    invalidating the original iterated value
  - `.iter` - will process each item in the iterator as a borrowed value,
    leaving the original collection intact
  - `.iter_mut` - will process each item in the iterator as a mutable borrowed
    value, allowing for mutating the original collection
- by default, `for in` uses `.into_iter` on collections

### `match`

- as with matching in Haskell, all branches in a `match` statement in Rust must
  be provided
- as in Haskell, `_` is the catch-all

#### Destructuring

- `match` blocks can destructure a variety of items:
  - tuples
  - arrays and slices
  - enums
  - pointers
  - structures
- matched values in `match` blocks can be named for access inside the the match:

  ```rust
  let x = 2;

  match x {
    z @ 2 => println!("z: {z}"),
    _ => println!("no match")
  }
  ```

#### Guards

- guards in `match` blocks work similarly to guards in Haskell; a value that
  matches multiple branches conditions will be evaluated against the guards
  for those branches

  ```rust
  enum Number {
    Integer(i32),
    Decimal(float32)
  }

  let x = Number::Integer(4);

  match x {
    Number::Integer(z) if z > 5 => println!("x is an integer greater than 5: {z}"),
    Number::Integer(z) => println!("x is an integer not greater than 5: {z}"),
    _ => println!("x is a decimal")
  }
  ```

- Rust's compiler won't infer whether a catch-all is optional for `match` blocks
  containing guards - the catch-all is required

#### Binding

- values in match branches can be bound to names so that they can be used inside
  the block:

  ```rust

  let x = 5;

  match x {
    n @ 1..=10 => println!("x is in the range 1-9: {n}"),
    _ => println!("x is not in range")
  }
  ```

### `if let`

- matching on a single value is awkward, so Rust provides syntactic sugar via
  `if let` to mitigate having to match on the catch-all branch:

  ```rust
  let x: Option<i32> = 5;

  match x {
    Some(n) => println!("we've got something!"),
    _ => {}
  }

  // vs

  if let Some(n) = x {
    println!("we've got something!")
  }
  ```

### Additional

- `String.is_empty()` is syntactic sugare for `String.len() > 0`
- concatenating `String`s is not as simple as adding them together - a `String`
  must be followed by `&str` values
- ternary operators in Rust are not like ternaries in Python or Javascript:

  ```rust
  let x = 5;
  let y = if x > 0 { true } else { false };
  ```

- references are dereferenced with `*`
- destructuring of references can be done using `&`, `ref`, or `ref mut`
- variables can be assigned as references, as opposed to a reference being
  created from another value:

  ```rust
  let x = 4;
  let ref_x = &4;

  // vs
  let ref_x = &4; // <= a reference at assignment
  // or
  let ref ref_x = 4; // <= a reference at assignment
  ```

- values in `match` blocks can be matched against ranges:

  ```rust
  let x: Option<i32> = 5;
  let y = 'n';

  match x {
    Some(n @ 1..=10) => println!("x is in range"),
    _ => println!("x is not in range"),
  }

  match y {
    n @ 'a'..='z' => println!("y is lowercase"),
    _ => println!("y is not lowercase"),
  }
  ```
