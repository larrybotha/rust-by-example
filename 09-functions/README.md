# Functions

- https://doc.rust-lang.org/stable/rust-by-example/fn.html
- https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
- https://doc.rust-lang.org/stable/book/ch13-00-functional-features.html

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

#### Closures as input parameters

- closures by themselves may be optionally typed
- as arguments to other functions, Rust requires that closures be annotated,
  using one of three options:
  - `Fn` - a closure that captures values by reference, i.e. `&T`. Mutable
    references, and values passed in by value are not allowed
  - `FnMut` - a closure that captures values by mutable reference, by
    mutable reference, i.e. `&mut T`
  - `FnOnce` - a closure that captures values by value. Because borrowed or
    mutably borrowed values can be operated on in the same way that a value
    passed in by value can, the resulting type of capture is defined by how
    the closure is used
- these annotated closure capture values in the least restrictive manner for
  their type, with the compiler determining at compile-time what type the
  captured values will be
- defining a type signature for an input function of type `Fn` can be done as
  follows:

  ```rust
  fn my_func<F>(f: F) -> ()
    where F: Fn(i32) -> ()
  {
    f(5)
  }
  ```

### Additional

- values can be manually cleaned up from memory using `std::mem::drop`:

  ```rust
  use std::mem;

  let x = Box::new(5);

  std::mem::drop(x); // x is no longer valid
  ```

- `std::mem::drop` requires values to be passed as `T` - i.e. not by reference /
  borrow
- `std::mem::move` allows one to manually move ownership of values
- `Vec.contains` is similar to Javascript's `Array.indexOf`, except returning a
  boolean directly without further evaluation. It's more akin to Python's `x is in xs` syntax
- `std::any::type_name` can be used to get the type from a value:

  ```rust
  fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
  }
  ```

- the `where` syntax is used to describe generic parameters:

  ```rust
  let my_func<T>(value: T) -> i32 where T: i32 {
    // ...
  }
  ```
