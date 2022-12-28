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

### Operator overloading

- as in Haskell, we can define operators for specific types, .e.g "Define how
  '+' works for a `Person` struct"
- operator traits expect a type, which allows one to define how an operator
  works between types:

  ```rust
  struct A;

  // implement A + B
  impl std::ops::Add<B> for A {
    //
  }

  struct B;

  // implement B + A
  impl std::ops::Add<A> for B {
  //
  }
  ```

- to implement operators without consuming the types, one must use lifetimes
  when creating the implementation:

  ```rust
  struct MyType {};

  impl<'a, 'b'> std::ops::Add<&'b MyType> for &'a MyType {
    // ...
  }

  let x = MyType {};
  let y = MyType {};
  let sum = &x + &y;
  ```

### Drop

- the `Drop` trait allows one to modify the implementation of `drop` for a
  given type

### Iterators

- the `Iterator` trait must be used to implement iterators over collections of
  values. The only required method is `.next`
- `for` can automatically turn a struct which implements `Iterator` into an
  iterator via `.into_iter()`:

  ```rust
  let xs: [i32; 3] = [1,2,3];

  for x in xs.iter() {
    println!("{x}");
  }
  ```

- structs that implement `Iterator` have a few useful methods available them,
  such as `.skip`, `.take`, `.sum`, etc.

### `impl Trait`

- similar to type hints in Python, we can specify that either an argument to a
  function, or its return value implement a specific trait

  - as function arguments:

    ```rust
    // using bounds
    fn my_func<T: MyTrait>(x: T) {
      // ...
    }

    // using impl Trait
    fn my_func(x impl MyTrait) {
      // ...
    }

    ```

  - as function return types:

    ```rust
    // using 'where'
    fn my_func() -> T where T: MyTrait {
      // ...
    }

    // using impl Trait
    fn my_func() -> T impl MyTrait {
      // ...
    }
    ```

- a limitation of using the `impl Trait` is that the turbofish syntax may not be
  used to coerce types, either when it is used for function arguments, or
  return types

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
- to consume an iterator without returning a value, use `.for_each(drop)`:

  ```rust
  { 0..10 }
    .map(|x| x + 1)
    .map(|x| println!("x is {x}"))
    .for_each(drop) // consume the iterator without a return value
  ```
