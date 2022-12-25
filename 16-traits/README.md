# Traits

- https://doc.rust-lang.org/stable/rust-by-example/trait.html

## Takeaways

- traits are collections of methods defined for an unknown type, `Self`
- methods within a trait have access to other methods within the trait
- an instance of a struct that implements a trait may be instantiated via the
  trait and a type annotation:

  ```rust
  trait MyTrait {
    fn new() -> Self;
  }

  struct MyStruct {}

  impl MyTrait for MyStruct {
    fn new() -> Self {
        MyStruct {}
    }
  }

  let instance: MyStruct = MyTrait::new();
  ```

### Deriving traits

- as in Haskell, there are traits that can be derived for types without
  additional work
- these derivable traits may also be manually implemented if need be
- `Clone` is derivable, and will create a cloned instance of a type given a
  reference to the original instance
- `Copy` replaces a type's _copy semantics_ with _move semantics_
- `Default` adds a `::default` associated function which returns an empty
  instance of a type by default, which may also be useful to override:

  ```rust
  #[derive(Debug)]
  struct Point {
      x: i32,
      y: i32
  }

  impl std::default::Default for Point {
      fn default() -> Self {
          Self { x: 0, y: 0 }
      }
  }
  ```

- as in Haskell, evaluating equality of types requires implementing traits
  associated with equality:

  ```rust
  #[derive(Debug, PartialEq)]
  struct Int(i32);

  let x = Int(5);
  let y = Int(6);

  assert_ne!(x, y);
  assert!(x.eq(&y));
  ```

  Deriving `PartialEq` adds `.eq` and `.ne` methods to the instance

- `Eq` may only be applied to types that implement `PartialEq`, and where every
  value of the type is equal to itself. An example of where this fails is for
  floating point numbers, where `NaN` does not equal itself

- as in Haskell, implementing `Ord`-like traits is required to compare values in
  an ordinal fashion. Equality

  ```rust
    use std::cmp::Ordering;

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Int(i32);

    let x = Int(6);
    let y = Int(5);

    assert!(x > y);
    assert!(x.gt(&y));
    assert_eq!(x.cmp(&y), Ordering::Greater);
  ```

  `PartialOrd` adds `.gt`, `.lt`, `.le`, `.ge`, and `.cmp` methods to the instance.
  `.cmp` returns a variant from the `std::cmp::Ordering` enum

### Returning traits with `dyn`

- the [Rust docs on trait objects](http://doc.rust-lang.org/1.65.0/reference/types/trait-object.html)
  has a succinct example on using `dyn` to return a trait when the concrete
  type is not known
- generally, Rust requires a concrete type to be returned from functions, as
  Rust requires at compile time a way to determine the memory allocation of a
  type. Using `dyn` along with `Box` allows for an escape hatch, where one can
  specify that something that is heap allocated and implements a specific
  trait is going to be returned

## Additional

- functions in traits are all called associated functions - they're associated
  with the type after `impl`

  - associated functions may or may not be methods. Functions that have a
    reference to the instance, as in other languages, are methods. Functions
    that don't have a reference to the instance don't appear to be given a
    special name, but are akin to static methods in other languages:

    ```rust
    trait MyTrait {
        fn my_static_method();
    }
    ```

    ```python
    from abc.collections import ABCMeta

    class MyBaseClass {
        @staticmethod
        def my_static_method():
            pass
    }
    ```

- prefer `const` over `static` for constants: [Static
  items](http://doc.rust-lang.org/1.65.0/reference/items/static-items.html#using-statics-or-consts)
