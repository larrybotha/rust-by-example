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
