# Functions

- https://doc.rust-lang.org/stable/rust-by-example/fn.html
- https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html

## Takeaways

- the final expression of a function will be returned if it doesn't havea a
  trailing semi-colon
- `return` works as in other languages, and allows for returning from within
  loops etc. too

### Associated functions and methods

- associated functions are similar to static functions in OO languages - they
  are functions namespaced by the enum or struct they are implemented on
- methods are functions implemented on enums or structs that operate on
  instances of those enums or structs, and similarly to Python, use the `self`
  keyword to reference the instance
- `&self` is syntactic sugar for `self: &Self` in methods:

  ```rust
  struct MyStruct;

  impl MyStruct {
    // explicit
    fn another_method(self: &Self) {}
    // syntactic sugar
    fn some_method(&self) {}
  }
  ```

### Closures

- closures in Rust are more similar to closures in Haskell than Javascript
- blocks in closures are optional when there is only a single expression,
  otherwise they are mandatory

#### Capturing

- closures can capture variables in three ways:
  - by reference: `&T` - the default
  - by mutable reference: `&mut T`
  - by value: `T`
- Rust does this dynamically, based on the context of the lambda, but will
  maintain the type of capture for any subsequent executions of the closure
- capturing by reference has the least impact on values - only when required
  will Rust capture by mutable reference, and then by value
- a closure containing a mutable reference needs to be mutable itself, to
  account for the mutable captured value:

  ```rust
  let mut x = 3;
  let mut inc_x = || x += 1;
  ```

- as with other mutable references, only once a mutable reference has been
  consumed may another reference to the original value be created, except
  that. Once the closure has been executed, other references may be made to
  the value
- values that are referenced by value are moved into the closure, which appears
  to prevent the value from being referenced anywhere after the closure
  definition, even before the closure is executed:

  ```rust
  use std::mem;

  let x = Box::new(5);
  let my_closure = || mem::drop(x); // x is moved here

  // x may not be referenced after my_closure
  // println!("{x}"); // invalid
  ```

- if a value inside the closure is captured by value, and is not `Copy`, then the
  value will be moved, otherwise not

### Additional

- values can be manually cleaned up from memory using `std::mem::drop`:

  ```rust
  use std::mem;

  let x = Box::new(5);

  std::mem::drop(x); // x is no longer valid
  ```

- `std::mem::drop` requires values to be passed as `T` - i.e. not by reference /
  borrow
