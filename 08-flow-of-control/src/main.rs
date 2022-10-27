fn if_else_no_parens() {
    let x = 5;

    if x > 0 {
        println!("greater than 0");
    } else {
        println!("not greater than 0");
    }

    println!();
}

fn if_else_expressions() {
    let x = 5;
    let result = if x > 0 && x < 5 {
        "less than"
    } else if x == 5 {
        "equal to"
    } else {
        "greater than"
    };

    println!("x is '{result}' 5");
    println!();
}

fn loop_example() {
    let mut count = 0;
    let mut xs: Vec<i32> = vec![];

    loop {
        count += 1;

        if count % 2 != 0 {
            println!("skipping odd number");
            // continue to next iteration
            continue;
        }

        if count >= 10 {
            println!("breaking at {count}");
            // break out of the loop
            break;
        }

        xs.push(count);
    }

    println!("xs: {xs:?}");
    println!();
}

fn nested_loops_and_labels() {
    let mut count = 0;
    let mut xs: Vec<Vec<i32>> = vec![];

    'loop_outer: loop {
        count += 1;
        let mut zs: Vec<i32> = vec![];
        let mut inner_count = 0;

        'loop_inner: loop {
            inner_count += 1;

            if zs.iter().sum::<i32>() > 10 {
                break 'loop_inner;
            }

            if inner_count == 5 {
                println!("continuing 'loop_outer from 'loop_inner");
                continue 'loop_outer;
            }

            zs.push(count + inner_count);
        }

        xs.push(zs);

        if count >= 5 {
            break 'loop_outer;
        }
    }

    println!("xs: {xs:#?}");
    println!();
}

fn return_from_loop() {
    let mut count = 0;
    let result = loop {
        count += 1;

        if count >= 5 {
            break count;
        };
    };

    assert_eq!(result, 5);
    println!("result: {result}");
    println!();
}

fn while_example() {
    let mut count = 0;

    while count < 5 {
        println!("count is {count}");

        count += 1;

        if count >= 5 {
            break;
        }
    }

    println!();
}

fn for_example() {
    // a range from 0 to 31 exclusive
    for x in 0..31 {
        // ternary's require blocks to evaluate to
        let fizz = if x % 3 == 0 { "fizz" } else { "" };
        let buzz = if x % 5 == 0 { "buzz" } else { "" };
        let xs = vec![fizz, buzz]
            .iter()
            .filter(|z| !z.is_empty()) // the type here is &&&str... blegh..?
            .fold(String::new(), |acc, s| acc + s);

        if !xs.is_empty() {
            println!("{x:>2}: {xs}");
        }
    }

    println!();
}

fn for_into_iter_mut() {
    let xs = vec![1, 2, 3]; // get a heap allocated value

    println!("for loop, implicit .into_iter");
    // implicitlry uses xs.into_iter, invalidating xs
    for x in xs {
        println!("x: {x}");
    }
    println!();

    // the following will not compile - xs is invalid here
    //println!("{xs:?}");

    let xs = vec![1, 2, 3];

    println!("for loop, explicit .into_iter");
    // same as previous loop
    for x in xs.into_iter() {
        println!("x: {x}");
    }
    println!();

    // will also not compile
    //println!("{xs:?}");

    let xs = vec![1, 2, 3];

    println!("for loop, explicit .into_iter");
    // same as previous loop
    for x in xs.iter() {
        println!("x: {x}");
    }

    println!("xs is still valid: {xs:?}");
    println!();

    let mut xs = vec![1, 2, 3];

    for x in xs.iter_mut() {
        println!("squaring {x}");
        //*x = *x * *x; // we need to dereference x before mutating it...
        // for some reason
        let squared = *x * *x;
        *x = squared;
    }

    println!("xs is valid and mutated: {xs:?}");
    println!();
}

#[allow(unreachable_patterns)]
fn match_tuple() {
    let tuple = (3.2, 1, "foo");

    // named values on match
    match tuple {
        (x, 1, y) => println!("x: {x}, y: {y}"),
        _ => println!("no match!"),
    }

    // always match, skip remaining
    match tuple {
        (x, ..) => println!("first is {x}"),
        _ => println!("no match!"),
    }

    match tuple {
        (_, second @ 1, ..) => println!("second is named and is {second}"),
        _ => println!("no match!"),
    }

    // skip everythin but last
    match tuple {
        (.., last) => println!("last is {last}"),
        _ => println!("no match!"),
    }

    println!()
}

#[allow(unreachable_patterns)]
fn match_arrays_slices() {
    let xs = [1, 2, 3];

    match xs {
        [1, x, _] => println!("x: {x}"),
        _ => println!("no match"),
    }

    match xs {
        [head, ..] => println!("only head: {head}"),
        _ => println!("no match"),
    }

    match xs {
        [_, tail @ ..] => println!("only tail: {tail:?}"),
        _ => println!("no match"),
    }

    match xs {
        [_, middle @ .., _] => println!("only middle: {middle:?}"),
        _ => println!("no match"),
    }

    match xs {
        [first, middle @ .., last] => {
            println!("all named - first: {first}, middle: {middle:?}, last: {last}")
        }
        _ => println!("no match"),
    }
}

fn match_enums() {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
        Rgb(u8, u8, u8),
        Cmyk(u8, u8, u8, u8),
    }

    let simple = Color::Red;
    let rgb = Color::Rgb(0, 1, 2);
    let cmyk = Color::Cmyk(0, 1, 2, 3);

    match simple {
        Color::Red => println!("red!"),
        Color::Green => println!("green!"),
        Color::Blue => println!("blue!"),
        _ => println!("no match"),
    }

    match &rgb {
        color @ Color::Rgb(r, g, 3) => println!("r: {r}, g: {g}, last is 3 for {color:?}"),
        color @ Color::Rgb(r, g, 2) => println!("r: {r}, g: {g}, last is 2 for {color:?}"),
        _ => println!("no match"),
    }

    match &cmyk {
        Color::Red => println!("red!"),
        Color::Green => println!("green!"),
        Color::Blue => println!("blue!"),
        Color::Rgb(..) => println!("rgb"),

        color @ Color::Cmyk(c, m, y @ 3, k) => {
            println!("c: {c}, m: {m}, y: {y}, k: {k} for {color:?}")
        }
        color @ Color::Cmyk(c, m, y @ 2, k) => {
            println!("c: {c}, m: {m}, y: {y}, k: {k} for {color:?}")
        }

        // no need for catch-all - this is also a catch-all because we're
        // evaluating all values
        color @ Color::Cmyk(c, m, y, k) => {
            println!("c: {c}, m: {m}, y: {y}, k: {k} for {color:?}")
        }
    }
    println!()
}

#[allow(clippy::match_single_binding)]
fn match_refs_pointers() {
    let x = 4;
    let x_amp = &4;
    #[allow(clippy::toplevel_ref_arg)]
    let ref x_ref = 4;
    let mut x_mut = 5;

    // create a reference
    match x {
        ref val => println!("get a reference to x: {val}"),
    }

    // use & to signify we have a reference
    match x_amp {
        &val => println!("using & to indicate we have a reference: {val}"),
    }

    // use * to dereference
    match *x_ref {
        val => println!("dereference a reference: {val}"),
    }

    // create a mutable reference from a mutable value
    match x_mut {
        ref mut val => {
            // we have a reference here - before we can mutate it, we need to
            // dereference it
            *val += 1;
            println!("mutated: {val}");
        }
    }

    println!();
}

fn match_struct() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = &Point { x: 1, y: 2 };

    match point {
        p @ Point { x: 1, .. } => println!("when x is 1: {p:?}"),
        _ => println!("no match"),
    }

    match point {
        p @ Point { x: i, y: 2 } => println!("when y is 2, x is {i} for {p:?}"),
        _ => println!("no match"),
    }
    println!();
}

fn match_guards() {
    #[allow(dead_code)]
    enum Number {
        Integer(i32),
        Decimal(f32),
    }

    let x = Number::Integer(3);

    match x {
        Number::Integer(z) if z > 10 => println!("greater than 10: {z}"),
        Number::Integer(z) if z < 5 => println!("less than 5: {z}"),
        Number::Integer(z) => println!("something else: {z}"),
        Number::Decimal(..) => println!("a decimal"),
    }

    println!();
}

fn main() {
    if_else_no_parens();
    if_else_expressions();

    loop_example();
    nested_loops_and_labels();
    return_from_loop();

    while_example();

    for_example();
    for_into_iter_mut();

    match_tuple();
    match_arrays_slices();
    match_enums();
    match_refs_pointers();
    match_struct();
    match_guards();
}
