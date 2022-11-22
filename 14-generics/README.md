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

### Functions

- generic function parameters may be specificed implicitly, by the type passed
  in, or explicitly when called:

  ```rust
  struct GenStruct<T>(T);

  fn generic<T>(x: GenStruct<T>) { ... }

  let my_gen_value = GenStruct("foo".to_string());

  generic(my_gen_value); // implicitly specified
  generic::<i8>(GenStruct(4)); // explicitly specified
  ```

### Implementation

- implementations for generic structs require a similar `<T>` syntax:

  ```rust
  struct GenStruct<T>(T);

  impl<T> GenStruct<T> { ... }
  ```

### Traits

- traits can be generic, with the generics defined _after_ the name of the
  trait:

  ```rust
  trait MyTrait<T> {
    fn method(self, _T);
  }

  // implement MyTrait generically for any caller U
  impl<T, U> MyTrait<T> for U {
    ...
  }
  ```

### Bounds

- generic type parameters may define _bounds_ - traits that the type must
  implement for it to be passed in

  ```rust
  fn do_the_print<T: Display>(value: T) {
  //                   [1]
  // 1 - a bound on the generic type T
    println!("{value}");
  }
  ```

- structs may also define bounds:

  ```rust
  struct MyType<T: Display>;
  ```

- generic values passed into functions that have a defined bound may safely
  access any methods defined on the trait

## Additional

- types can be cast using:
  - turbofish syntax: `SomeGeneric::<u8> { ... }`
  - `as`: `SomeGeneric { value: 5 as i32 }`
  - appending the type: `SomeGeneric { value: 5i32 }`
