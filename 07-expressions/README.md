# Expressions

- https://doc.rust-lang.org/stable/rust-by-example/expression.html

## Takeaways

- Rust is primarily composed of statements
- two common types of statements are:

  - variable bindings
  - expressions followed by semicolons:

    ```rust
    // statement, containing the expression 7 + 2
    let x = 7 + 2;

    // x is an expression, the sem-colon makes the line a statement
    x;
    // similarly here, too
    5;

    // an expression - not a statement
    'X'
    ```

- this differs from Javasript where variable assignments are considered
  expressions - the assignment _resolves_ to a value, thus it is an
  expression. In Rust, a variable binding appears not to resolve to a value,
  perhaps because we're closer to the metal, and we're working closer with
  memory
- blocks are expressions in Rust, which means they can be used in variable
  assignment. If the last line of a block is an expression - i.e. is not
  followed by a semi-colon - it will be returned in a variable assignment,
  otherwise unit `()` will be returned
