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
}
