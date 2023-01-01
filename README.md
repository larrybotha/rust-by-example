# Rust By Example

Learning and annotations from the [Rust By Example book](https://doc.rust-lang.org/stable/rust-by-example/)

## Chapters

- [Hello World](./01-hello-world)
  - [Formatted Print](./01-hello-world/02-formatter-print)
- [Primitives](./02-primitives)
- [Custom Types](./03-custom-types)
- [Variable Bindings](./04-variable-bindings)
- [Types](./05-types)
- [Conversion](./06-conversion)
- [Expressions](./07-expressions)
- [Flow of Control](./08-flow-of-control)
- [Functions](./09-functions)
- [Modules](./10-modules)
- [Crates](./11-crates)
- [Cargo](./12-cargo)
- [Attributes](./13-attributes)
- [Generics](./14-generics)
- [Scoping Rules](./15-scoping-rules)
- [Traits](./16-traits)
- [macro_rules!](./17-macro-rules)

## Links and resources

- [A half hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
- [Learn Rust The Dangerous Way](http://cliffle.com/p/dangerust/)

## TODO

- move graph to chapter on ownership:

  ```mermaid
  stateDiagram-v2
    [*] --> NoValue
    NoValue --> Valid: assign value to variable

    state Valid {
        [*] --> HeapValue: /if dynamic length
        HeapValue --> HeapValue: assign to variable
        HeapValue --> HeapValue: pass to function
        note right of HeapValue
            variable assignment:
            values that implement Copy will result in
            a copy on the new variable
        end note
        --
        [*] --> StackValue: /if fixed length
        note left of StackValue
            variable assignment:
            moves ownership into new variable
        end note

        note right of StackValue
            passing variable to function:
            moves ownership info function scope
        end note
    }

    HeapValue --> Invalid: reach end of scope

    StackValue --> Invalid: assign to variable
    StackValue --> Invalid: pass to function
    StackValue --> Invalid: reach end of scope

    Invalid --> [*]
  ```
