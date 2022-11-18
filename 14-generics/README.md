# Generics

- https://doc.rust-lang.org/stable/rust-by-example/generics.html

## Takeaways

- generics are the mechanism for generalising types and functionalities
- similarly to TypeScript, generic type parameters are defined in angled
  brackets:

  ```rust
  fn some_func<T>(x: T) { ... }
  ```

- any type that is not generic is considered _concrete_

## Functions

- generic function parameters may be specificed implicitly, by the type passed
  in, or explicitly when called:

  ```rust
  struct GenStruct<T>(T);

  fn generic<T>(x: GenStruct<T>) { ... }

  let my_gen_value = GenStruct("foo".to_string());

  generic(my_gen_value); // implicitly specified
  generic::<i8>(GenStruct(4)); // explicitly specified
  ```
