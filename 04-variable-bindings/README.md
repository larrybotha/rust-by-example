# Variable Bindings

- https://doc.rust-lang.org/stable/rust-by-example/variable_bindings.html

## Takeaways

- Rust can infer many types without type annotations
- `let` is used to define variable bindings
- mutable bindings use `let mut`:

  ```rust
  let mut x = 1;

  x += 1;
  ```

- to prevent the compiler from complaining about unused variables, prefix the
  variable with an underscore
- to prevent the compiler from complaining about using specific variable names,
  such as `foo`, append an underscore
- variables bindings have scopes which are defined by open and closing braces. A
  pair of braces is called a _block_:

  ```rust
  fn main() {
    // main 'block'
    let x = "inner"; // scoped to 'main'

    {
      let x = "outer"; // scoped to this block
    } // x inside the block is no longer in scope
  }
  ```

- variable shadowing is allowed in Rust. This is aided by ownership - once a
  non-mutable heap-allocated value is used, it's variable is no longer valid,
  and the binding is free to allocate to a new value. Inside a block,
  shadowing of an outer variable does not mutate the outer variable - the
  block-variable's value is only valid within its block:

  ```rust
  fn some_func() {
    let x = "outer";

    {
      let x = "inner"; // shadowing allowed, does not affect outside x
      println!("{x}");
    }
    println!("{x}");

    let x = "outer again"; // shadowing of x is allowed, as the previous x is
                           // no longer valid
    println!("{x}");
  }
  ```

- variables may be initialised later:

  ```rust
  let x;

  {
    let y = 3;
    x = 4;
  }
  ```

  Unitialised variables are not allowed

- shadowing a mutable variable with a non-mutable variable, and then attempting
  to modify it, will not compile:

  ```rust
  let mut x = 5;

  {
    let x = x;
    x = 5; // <= does not compile
  }
  ```
