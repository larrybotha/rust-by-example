# Custom Types

- https://doc.rust-lang.org/stable/rust-by-example/custom_types.html

## Takeaways

- custom types in Rust are created primarily using enums and structs
- constants can be created using the `const` and `static` keywords

### structs

- `structs` is short for _structures_
- there are 3 main types of structs:
  - tuple structs - these are basically named tuples - i.e. a tuple that is
    defined by a type, _not_ like `namedtuple` in Python
  - C structs, which are similar to interfaces in Typescript
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

### Additional

- `#[allow(dead_code)]` will silence dead-code notifications
- `assert_eq!` can be used to assert equality of two values, as syntactic sugar
  for `assert!(a == b)`
