# Functions

- https://doc.rust-lang.org/stable/rust-by-example/fn.html
- https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
- https://doc.rust-lang.org/stable/book/ch13-00-functional-features.html
- https://huonw.github.io/blog/2015/05/finding-closure-in-rust/

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

- `Fn` is the most restrictive, and expects a captured value to be immutably
  borrowed
- `FnMut` expects a captured value to be mutably borrowed. It requires
  annotation where it is used as an input parameter:

  ```rust
  fn apply<F>(mut f: F)
    where F: FnMut()
  {
    f();
  }

  let mut x = 5;
  let my_func = || {
    *x += 1;
  }
  ```

- `FnOnce` expects is the most permissive, but is required when a value must be
  captured by value, e.g. when moving / dropping

#### Type anonymity

- when closures are defined, Rust implicitly creates an anonymous struct to
  store captured variables inside, and implements one of `Fn`, `FnMut`, or
  `FnOnce` for the closure

  - the type of closure is only know when it is called, so when the closure is
    used as an input type, it must be defined as a generic
  - furthermore, an anonymous type `<T>` _still_ doesn't provide enough
    information to the type signature (...what is the reason for this?)
  - to address this, the generic value, i.e. the closure, should be indicated as
    having to implement one of the closure traits

    e.g.

    ```rust
    // f is a closure which takes no arguments, returns nothing,
    // and must implement Fn - i.e. it captures variables by reference
    fn apply<F>(f: F) // f is a generic, F
      where F: Fn()   // and F must implement Fn
    {
      f()
    }

    let x = 4;
    // capture 'z' into an anonymous type that must implment Fn,
    // and bind that type to my_func
    let my_func = || println!("{x}");

    apply(my_func)
    ```

#### Input functions

- as in other languages that support a functional style of programming, Rust has
  full support for higher order functions, whether they are lambdas /
  closures, or named functions
- `Fn`, `FnMut`, and `FnOnce` all apply to the function accepting the other
  function as an input parameter, regardless of whether it's a closure or not

#### As output parameters

- returning a closure directly from another function would mean that we're
  returning an unknown type. To address this, we need to specify that the
  closure is one of `Fn`, `FnMut`, or `FnOnce`. This is done by indicating
  that the containging function _implements_ the trait;

  ```rust
  fn i_really_love_closuring() -> impl Fn() {
    move || println!("called!")
  }
  ```

- `move` is required to ensure that any captured values are owned by the closure
  before it moves out of scope. This prevents dangling references inside the
  closure after its ownership moves to where it's called

#### Examples in `std`

##### `Iterator::any`

- `Iterator::any` operates in a similar manner to Javascript's `Array.some`
  ```rust
  let xs = vec![1,2,3];
  // .iter requires each value to be destructured from a ref
  let result = xs.iter().any(|&x| x > 2);
  // .into_iter requires no destructuring, but xs is then consumed
  let result = xs.into_iter().any(|x| x > 2);
  ```
- for a struct to implement `Iterator`, it must implement `Iterator::next` at
  the minimum
- `.iter()` passes values by reference, while `.into_iter` moves ownership of
  values, dropping the value that was turned into an iter

#### Searching through iterators

- `Iterator::find` is analogous to Javascript's `Array.find`, and similar to
  Python's `next`, except that the result is `Option`:

  ```rust
  let xs = vec![1,2,3];
  let x = xs.iter().find(|&x| x % 2 == 0);

  if let Some(n) = x {
    println!("even number exists and is {n}");
  }
  ```

- depending on whether the iterator is consumable (`.into_iter`), or uses
  references to access values, the methods on the iterator require different
  ways of accessing elements in the iterator

  - `.iter()` yields `&T`, i.e. _yield_ in terms of generators in Python and
    Javascript.

    The `.find` predicate accepts values by mutable reference, i.e. `&mut T`.

    To destructure values in the predicate, we need to account for both levels
    of reference using a double ampersand:

    ```rust
    let xs = vec![1,2,3];
    let result = xs.iter().find(|&&x| x == 2);
    //               ^           ^^
    //               |           ||
    //               ]          /  \
    //              [1]     [2]-    -[3]
    //
    // 1 - create a non-consuming iterator, yielding &i32
    // 2 - this ampersand destructures the iterator's reference (...?)
    // 3 - this ampersand destructures the predicate's reference (...?)
    ```

    See the resulting type of `x` each time an ampersand is added or removed,
    see the type signature for `Iterator::find` - the values passed to the
    predicate are annotated as `&Self::Item`

    Interestingly, the double ampersand is not needed when evaluation with
    modulo arithmetic:

    ```rust
    let xs = vec![1,2,3];
    let first_even = xs.iter()      .find(| &x| x % 2 == 0);
    let also_first_even = xs.iter() .find(|&&x| x % 2 == 0);
    ```

    `Rem` appears to return `i32` regardless of whether it's operating on `&i32`
    or `i32`

  - `.into_iter()` turns a collection into a consuming iterator - values are
    passed to our predicate by value. The predicate accepts values by mutable
    reference, as with `.iter`, so our predicate needs to destructure with a
    single ampersand, in the same way we do with borrowed function parameters in
    general:

    ```rust
    let xs = vec![1,2,3];
    let result = xs.into_iter().find(|&x| x == 2);
    // xs is no longer valid here
    ```

- the index of a value in a collection can be retrieved using
  `Iterator::position`:

  ```rust
  let xs = vec!["foo".to_string(), "bar".to_string()];
  let index = xs.iter().position(|x| x == "foo");
  ```

  In this example, `x` is `&String` - it's already

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

- `Iterator::find` is the equivalent of first filtering, and then getting the
  first element in the iterator:

  ```rust
  let xs = vec![1,2,3];
  let result = xs.iter().filter(|&x| x % 2 == 0).next();
  ```
