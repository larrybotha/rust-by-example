# Custom Types

- https://doc.rust-lang.org/stable/rust-by-example/custom_types.html
- https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/static_lifetime.html

## Takeaways

- custom types in Rust are created primarily using enums and structs
- constants can be created using the `const` and `static` keywords

### structs

- `structs` is short for _structures_
- there are 3 main types of structs:
  - tuple structs - these are basically named tuples - i.e. a tuple that is
    defined by a type, _not_ like `namedtuple` in Python
  - C structs, which are similar to interfaces in TypeScript
  - unit structs, which are useful when creating generic types. Unit structs
    have no fields
- accessing values in a tuple struct is done the same way as in normal tuples -
  via 0-indexed keys
- tuple structs may also be instantiated with keys:

  ```rust
  struct MyTuple(i32, i32, i32);

  let my_tuple = MyTuple {0: 3, 1: 5, 2: 7};
  ```

- similar to objects in Javascript, structs can be instantiated with shorthand:

  ```rust
  struct Thing {
    my_field: String,
  };

  let my_field = String::from("foo");
  let thing = Thing { my_field }
  ```

- _struct update syntax_ is the name given to spreading values into other
  structs:

  ```rust
  struct MyStruct {
    a: i32,
    b: i32,
    c: i32,
  };

  let first = MyStruct { a: 1, b: 2, c: 3 };
  let second = MyStruct { b: 3, ..first };
  ```

  _struct update syntax_ can only be used after other fields have been
  specified

- structs can be destructured in the same way they can in Javascript and Python,
  except that that the name of the struct is required when destructuring:

  ```rust
  struct MyStruct {
    foo: i32,
    bar: i32,
  };

  let thing = MyStruct { foo: 1, bar: 2 };
  let MyStruct { foo: not_foo, bar } = thing;
  ```

  Destructuring can be nested - see the `struct_exercise` function in
  [./src/main.rs](./src/main.rs)

### Enums

- enums in Rust are similar to enums in TypeScript, except that:
  - enums may contain structs, and not only the C-like variants that TypeScript
    uses (i.e. implicitly-indexed enums, and valued enums)
  - enums may have methods defined using `impl` in the same way as structs do
- variants in enums may be, unit structs, tuple structs, or C-like structs:

  ```rust
  enum MyEnum {
    Unit,
    Tuple(i32),
    CLike { foo: String, }
  }

  let x = MyEnum::UnitVariant;
  ```

- enum variants can be accessed through type aliases

  ```rust
  enum MyReallyLongEnumName {
    A,
    B,
    C
  }

  type Letter = MyReallyLongEnumName;

  let a = Letter::A;
  ```

- `Self` can be used to reference an enum or struct when using `impl`:

  ```rust
  enum Thing {
    A,
    B,
    C
  }

  impl Thing {
    fn print(&self) -> () {
      match &self {
        Self::A => println!("was A"),
        Self::B => println!("was B"),
        Self::C => println!("was C"),
      }
    }
  }

  let thing = Thing::A;

  thing.print();
  ```

  `Self` is a type alias within `impl` referring to the struct or enum being
  extended

- manual scoping of variants can be avoided with `use`:

  ```rust
  enum One {
    A,
    B
  }
  enum Two {
    C,
    D
  }

  fn my_func() {
    use crate::One::{A, B as Bee};
    use crate::Two::*;

    let a = A;
    let b = Bee;
    let c = C;
    let d = D;
  }
  ```

  - `use` allows for importing specific modules using braces
  - `use` allows for wildcard imports, similar to Python's `import *` syntax

- enums may be defined with implicit or explicit discriminators - the 'value'
  the unit structs may be bound to. These seem to only be allowed to be
  `isize` values:

  ```rust
  enum Implicit {
    A, // <= 0
    B, // <= 1
  }

  enum Explicit {
    A = 32,
    B = 101,
  }
  ```

### Constants

- Rust has two types of constants; those defined using `const`, and those
  defined using `'static`
  - `const` is unchangeable
  - `'static` is possible mutable with a `'static` lifetime
    - static lifetimes are inferred, and need not be specified
    - accessing or modifying mutable static variables is `unsafe`
- constants may be defined in any scope, including the global scope
- constants must be defined with an explicit type

### Additional

- `#[allow(dead_code)]` will silence dead-code notifications
- `assert_eq!` can be used to assert equality of two values, as syntactic sugar
  for `assert!(a == b)`
- type aliases are defined using the `type` keyword
- `use crate::` appears to refer to the current file / module
- types can be cast using `as`

  ```rust
  let x = 3.2 as i32;
  ```

- when matching on a reference inside an implementation, Rust by Example seems
  to favour dereferencing the instance to work on a concrete type instead of
  on a reference... not sure why yet...
- the linked list example uses `Cons` and `Nil` - these variants are used in
  [Fantas, Eels, and Specification][fantas-eels] for defining many functional
  constructs

<!-- Links -->

[fantas-eels]: http://www.tomharding.me/2017/03/03/fantas-eel-and-specification/ "Fantas, Eels, and Specification by Tom Harding"
