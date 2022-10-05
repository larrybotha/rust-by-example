# Rust By Example

Learning and annotations from the [Rust By Example book](https://doc.rust-lang.org/stable/rust-by-example/) 

## Chapters

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
