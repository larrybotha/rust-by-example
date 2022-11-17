# Attributes

- https://doc.rust-lang.org/stable/rust-by-example/attribute.html
- https://doc.rust-lang.org/reference/conditional-compilation.html
- https://doc.rust-lang.org/std/macro.cfg.html

## Takeaways

- attributes add metadata to crates, modules, or items
- this metadata can:
  - disable linting rules
  - set conditional compilation of code
  - mark functions and unit tests or related to benchmarks
  - enable compiler features
  - set a crate name, version, and type
- attributes that affect crates are defined with a bang:

  ```rust
  #![some_attribute]
  ```

- attributes that affect modules or items are defined without a bang:

  ```rust
  #[some_attribute]
  ```

- attributes can accept values, and in a number of ways:

  ```rust
  // assignment
  #[some_attr = "some-value"]

  // as an argument
  #[some_attr("some-value")]

  // as a named argument
  #[some_attr(key = "some-value")]
  ```

### Crates

- the `crate_name` and `crate_type` attributes can be used to set the name and
  type of a crate, but cargo ignores these attributes, so they're not widely
  used
- using the `crate_type` attribute mitigates having to use `--crate-type` with
  `rustc`

### `cfg`

- configuration conditional checks can be done either via an attribute, or via a
  macro:

  ```rust
  // attribute
  #[cfg(...)]

  // or macro
  cfg!(...)
  ```

- the attribute variant results in conditional compilation of code, while the
  macro results in run-time evaluation of code
