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
    let panic_result = panic::catch_unwind(|| x.expect("custom panic message!"));

    println!("{:?}", panic_result);
    println!();
}

fn main() {
    // panic
    panic_example();
    abort_condition_runtime();
    abort_condition_compiletime();

    // option and unwrap
    option_unwrap();
    option_expect();
}
