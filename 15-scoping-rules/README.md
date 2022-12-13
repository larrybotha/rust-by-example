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

## Additional notes

- [valgrid](https://valgrind.org/) is useful for profiling memory leaks on Linux
