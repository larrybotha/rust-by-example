# Scoping Rules

- https://doc.rust-lang.org/stable/rust-by-example/scope.html

## Takeaways

- scoping rules define how borrowing and lifetimes of values work
- the compiler uses scoping rules to determine
  - the validity of borrows
  - when memory can be freed
  - when variables are created and destroyed

### RAII

- RAII means _resource allocation is initialisation_ - when a value goes out of
  scope, it is destroyed so that memory can be freed
- variables in Rust don't only hold data - they _own_ resources, e.g. `Box` owns
  memory in the heap
- _owned_ values are stored on the heap, and these resources must be freed once
  they are no longer needed so that other resources may have access to that
  memory
- when values go out of scope, their destructors are called
- `Drop` is the trait used to create destructors. Most types in Rust
  automatically implement `Drop`, but it may be implemented if custom drop
  logic is required:

  ```rust
  struct DropStruct {}

  impl Drop for DropStruct {
    fn drop(&mut self) {
      // do something before memory is freed
    }
  }
  ```

### Ownership and moves

- variables are in charge of freeing their own resources
- resources can have only have one owner, which prevents double-free errors
- not all variables can own resources - e.g. references are not owned
- ownership is moved during variable assignment or passing values into functions
  if the value the original variable holds _is not `Copy`_ - i.e. if the value
  does not implement `Copy`, it will be moved during assignment, otherwise it
  will be copied
- `let y = x` is referred to as _x is moved into y_
  - when values are copied, one says _x is copied into y_
- once a value is moved, the previous owner cannot be referenced - this prevents
  dangling pointers

#### Mutability

- mutability of data can be changed when ownership is transferred:

  ```rust
  let x = String::from("foo"); // immutable
  let mut y = x; // x moved into y, mutably

  *y += "bar";
  ```

## Additional notes

- [valgrind](https://valgrind.org/) is useful for profiling memory leaks on Linux
- `Box`'s methods change depending on the type of value it contains:

  ```rust
  let mut box_int = Box::new(42); // has .pow, .log, etc.
  let mut box_str = Box::new("foo".to_owned()); // has .chars, .bytes

  *box_int += 5;
  *box_str += "bar";
  ```