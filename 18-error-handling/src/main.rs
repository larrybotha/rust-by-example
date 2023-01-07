fn panic_example() {
    fn gimme_even(x: i32) {
        if x % 2 != 0 {
            panic!("Even expected, got {}", x)
        }

        println!("Thanks for the even number {}", x)
    }

    gimme_even(2);

    //gimme_even(3);
    println!();
}

fn abort_condition_runtime() {
    fn do_the_panic() {
        if cfg!(panic = "abort") {
            println!("I'm aborting from runtime")
        } else {
            println!("I'm unwinding from runtime")
        }
    }

    do_the_panic();

    println!();
}

fn abort_condition_compiletime() {
    #[cfg(panic = "abort")]
    fn do_the_panic() {
        println!("I'm aborting from compiled")
    }

    #[cfg(not(panic = "abort"))]
    fn do_the_panic() {
        println!("I'm unwinding from compiled")
    }

    do_the_panic();
    println!()
}

fn option_unwrap() {
    use std::panic;

    let x: Option<i32> = None;
    let panic_result = panic::catch_unwind(|| x.unwrap());

    println!("{:?}", panic_result);
    println!()
}

fn option_expect() {
    use std::panic;

    let x: Option<i32> = None;
    let y: Option<i32> = Some(5);
    let panic_result = panic::catch_unwind(|| x.expect("custom panic message!"));
    let valid_result = panic::catch_unwind(|| y.unwrap());

    println!("panic result: {:?}", panic_result);
    println!("valid result: {:?}", valid_result);
    println!();
}

fn option_matching() {
    type Drink<'a> = Option<&'a str>;

    fn have_drink(drink: Drink) {
        match drink {
            Some("lemonade") => {
                println!("mmm, lemonade")
            }
            Some(inner) => {
                println!("mmm, not lemonade, but {}", inner)
            }
            None => {
                println!("awww... thirsty")
            }
        }
    }

    let x: Drink = Some("lemonade");
    let y: Drink = Some("coffee");
    let z: Drink = None;

    have_drink(x);
    have_drink(y);
    have_drink(z);
    println!()
}

fn option_unpacking() {
    fn maybe_plus_one(x: Option<i32>) -> Option<String> {
        let plus_one = x? + 1;

        Some(plus_one.to_string())
    }

    let x = Some(1);
    let y = None;

    println!("Some(1) plus one'd: {:?}", maybe_plus_one(x));
    println!("None plus one'd: {:?}", maybe_plus_one(y));
    println!()
}

fn option_chaining() {
    #[derive(Debug)]
    struct A {
        foo: Option<Foo>,
    }

    #[derive(Debug)]
    struct Foo {
        bar: Option<Bar>,
    }

    #[derive(Debug)]
    struct Bar {
        value: i32,
    }

    fn maybe_value(x: A) -> Option<i32> {
        // A similar syntax to 'optional chaining' in Javascript
        Some(x.foo?.bar?.value)
    }

    let x = A {
        foo: Some(Foo {
            bar: Some(Bar { value: 6 }),
        }),
    };
    let y = A {
        foo: Some(Foo { bar: None }),
    };

    println!("x's deep value: {:?}", maybe_value(x));
    println!("y's deep value: {:?}", maybe_value(y));
    println!()
}

fn option_map() {
    fn double(x: i32) -> i32 {
        x * 2
    }

    fn square(x: i32) -> i32 {
        x.pow(2)
    }

    fn process(x: Option<i32>) -> Option<i32> {
        x.map(double).map(square).map(|n| n - 1)
    }

    let x = Some(6).map(double).map(square).map(|n| n - 1);
    let y = process(Some(6));
    let z = process(None);

    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);
    println!()
}

fn main() {
    // panic
    panic_example();
    abort_condition_runtime();
    abort_condition_compiletime();

    // option and unwrap
    option_unwrap();
    option_expect();
    option_matching();
    option_unpacking();
    option_chaining();
    option_map();
}
