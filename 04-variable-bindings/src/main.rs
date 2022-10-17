fn mutable_binding() {
    let mut x = 1;

    println!("before: {x}");
    x += 1;
    println!("after: {x}");
    println!();
}

fn variable_scope() {
    let x = "outer";

    {
        let x = "inner";
        println!("x in block: {x}");
    }

    println!("x outside of block: {x}");
    println!();
}

fn variable_shadowing() {
    let x = "outer";

    {
        let x = "inner";
        println!("x in block: {x}");
    }
    println!("x outside of block: {x}");

    let x = "outer shadowed";

    println!("x outside of block: {x}");
    println!();
}

fn main() {
    mutable_binding();
    variable_scope();
    variable_shadowing();
}
