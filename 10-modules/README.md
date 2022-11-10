# Modules

- https://doc.rust-lang.org/stable/rust-by-example/mod.html

## Takeaways

- a module is a collection of functions, structs, traits, `impl` blocks, and
  other modules

### Visibility

- modules are declared using the `mod` keyword
- as in Javascript, and in opposition to Python, everything in a module is
  private by default
- the `pub` keyword allows one to make an item public
- items in nested modules can be made available to the parent and other modules
  in a number of ways:
  - using `pub(self)` - no access outside of the module, the same as _not_
    using `pub`
  - using `pub(in crate::path)` - accessible to any other module in the crate
  - using `pub(super)` - accessible to the parent of the current module
- e.g.:

  ```rust
  mod foo_mod {
    fn private_fn() {
      println!("only accessible in foo_mod")
    }

    pub mod bar_mod {
      pub(self) fn private_fn() {
        println!("only accessible in foo_mod::bar_mod")
      }

      pub fn public_fn() {
        println!("accessible via foo_mod::bar_mod")
      }

      pub(super) fn parent_access() {
        println!("accessible via foo_mof")
      }

      pub(in crate::foo_mod) fn crate_access() {
        println!("accessible anywhere in crate")
      }
    }
  }
  ```

### Struct Visibility

- when structs are defined inside modules, access to fields of instances can be
  hidden when the struct is used outside of the module. This allows for
  encapsulation
- fields in structs within modules are by default private
- e.g.

  ```rust
  mod foo_mod {
    pub struct MyPrivateFieldsStruct<T> {
      contents: T,
    }

    impl<T> MyPrivateFieldsStruct<T> {
      pub fn new(contents: T) -> MyPrivateFieldsStruct<T> {
        MyPrivateFieldsStruct { contents }
      }
    }

    pub struct MyPublicFieldsStruct<T> {
      pub contents: T,
    }
  }
  ```
