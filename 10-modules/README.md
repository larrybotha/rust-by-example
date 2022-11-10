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
