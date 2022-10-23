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

### `for` loops

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

### Additional

- `String.is_empty()` is syntactic sugare for `String.len() > 0`
- concatenating `String`s is not as simple as adding them together - a `String`
  must be followed by `&str` values
- ternary operators in Rust are not like ternaries in Python or Javascript:

  ```rust
  let x = 5;
  let y = if x > 0 { true } else { false };
  ```
