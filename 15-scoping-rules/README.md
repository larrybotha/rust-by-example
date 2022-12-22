# Scoping Rules

- https://doc.rust-lang.org/stable/rust-by-example/scope.html
- https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision

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

- variables are in charge of freeing their own resources
- resources can have only have one owner, which prevents double-free errors
- not all variables can own resources - e.g. references are not owned
- ownership is moved during variable assignment or passing values into functions
  if the value the original variable holds _is not `Copy`_ - i.e. if the value
  does not implement `Copy`, it will be moved during assignment, otherwise it
  will be copied
- `let y = x` is referred to as _x is moved into y_
  - when values are copied, one says _x is copied into y_
- once a value is moved, the previous owner cannot be referenced - this prevents
  dangling pointers

#### Mutability

- mutability of data can be changed when ownership is transferred:

  ```rust
  let x = String::from("foo"); // immutable
  let mut y = x; // x moved into y, mutably

  *y += "bar";
  ```

#### Partial moves

- destructuring of complex values, such as tuples and structs, can be done in
  two ways:

  - by value
  - by reference:

    ```rust
    struct Person {
      name: String,
      age: Box<i32>,
    }

    let sam = Person { name: String::from("sam"), age: Box::<i32>::new(42) };
    let Person { name, ref age } = sam; // name is destructured by value
                                        // age is destructured by reference
    ```

    This results in a partial move - `name` has been moved, but `age` has

- in addition to accessing the values that have been moved out of the original
  value, the original value, which is now partially moved, may not be referenced
- if either of the values were stored on the stack, the move would be
  unnecessary, as destructuring would result in copying

### Borrowing

- _Rust by example_ indicates that we generally want to access data without
  taking ownership - this is where borrowing and references come in
- while a value is borrowed, we are guaranteed that it cannot be destroyed

#### Aliasing

- immutably borrowed data can be borrowed an arbitrary number of times
- only a single mutable borrow may be used at a time, and no other references
  may be used until the last use of the mutable borrow
- any references created before the mutable borrow become invalid once the
  mutable borrow has been defined

#### The `ref` pattern

- `ref` can be used to create references to fields during `let` destructuring or
  pattern matching
- using `ref` during variable assignment is equivalent to using `&`:

  ```rust
  let x = 42;
  let ref ref_x_a = x;
  let ref_x_b = x;

  println!("refs point to same value?: {}" *ref_x_a == *ref_x_b);
  ```

- fields can be destructured as immutable references:

  ```rust
  struct Thing {
    a: i32,
    b: i32
  }

  let thing = Thing { a: 1, b: 1 };
  let Thing { a: ref a_ref, b: _ } = thing;

  println!("a_ref is {}", a_ref);
  ```

- fields can be destructured as mutable refs:

  ```rust
  struct Thing {
    a: i32,
    b: i32
  }

  let mut thing = Thing {a:1, b: 1};
  let Thing { a: ref mut a_mutable_ref, b: _ } = thing;

  *a += a; // => a == 2
  ```

### Lifetimes

- a variable's lifetime begins when it is created, and ends when it is destroyed
- the borrow checker uses lifetimes to determine the validity of borrows /
  references
- a borrow is declared using `&` - it is valid under the condition that it is
  destroyed before the value it is borrowing is destroyed - i.e. the borrow
  checker will not allow for us to create a borrow that may outlive the value
  it is borrowing

#### Explicit annotation

- all references have lifetimes
- there are rules which determine whether a lifetime needs to be explicit or
  not this - _elision of lifetime parameters_ referes to this and the rules
- for the following: `foo<'a, 'b>` - the item `foo` may not outlive either of
  the two lifetimes `'a` or `'b`
- lifetimes are applicable to borrows in trait implementations, function definitions,
  and variable declarations
- when lifetimes are ambiguous, say when a function accepts multiple borrowed
  values, and then returns some borrowed value, we need to be explicit to
  indicate to the compiler which lifetime is being returned

#### Structs and traits

- fields in structs that contain references must include explicit lifetime
  parameters
- an instance of a struct with lifetime parameters may not outlive the lifetime
  of the values it references
- `impl` may also have lifetime parameters if the struct that is implementing
  the trait has lifetime parameters

#### Bounds

- lifetimes are generic
- bounds can specify that all references in a type may not outlive a given
  lifetime parameter:

  ```rust
  // All references in T may not outlive 'a
  fn bounded_with_lifetime<'a, T: 'a>(x: T) {
    // ...
  }

  // T must implement Trait, and
  // all references in T may not outlive 'a
  fn bounded_with_trait_and_lifetime<'a, T: Trait + 'a>(x: T) {
    // ...
  }
  ```

## Additional notes

- [valgrind](https://valgrind.org/) is useful for profiling memory leaks on Linux
- `Box`'s methods change depending on the type of value it contains:

  ```rust
  let mut box_int = Box::new(42); // has .pow, .log, etc.
  let mut box_str = Box::new("foo".to_owned()); // has .chars, .bytes

  *box_int += 5;
  *box_str += "bar";
  ```

- `ref` can be used to destructure a value _by-reference_:

  ```rust
  struct MyThing {
    thing_a: String,
    thing_b: String,
  }

  let thing = MyThing {
    thing_a: String::from("foo"),
    thing_b: String::from("bar")
  };
  // thing_a is destructured by-value
  // thing_b is destructured by-reference
  let MyThing { thing_a, ref thing_b } = thing;
  ```

- a variable that is mutable may be passed through to functions with the most
  flexibility:
  - by value
  - by immutable reference
  - by mutable reference
- `str` variables are included in the resulting binary after compiling - they
  have lifetimes that span the lifetime of the application, and thus have the
  special `'static` lifetime:

  ```rust
  let x: &'static str = "foo";
  ```

- one can either deconstruct a tuple struct, or access the value directly in the
  struct:

  ```rust
  struct TupleStruct(i32);

  let value = TupleStruct(42);
  let TupleStruct(x) = value;
  let y = value.0;

  assert_eq!(x, y);
  ```
