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

fn main() {
    // panic
    panic_example();
    abort_condition_runtime();
    abort_condition_compiletime();

    // option and unwrap
    option_unwrap();
    option_expect();
    option_matching();
}
