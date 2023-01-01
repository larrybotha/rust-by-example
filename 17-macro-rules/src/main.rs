macro_rules! say_hello {
    // () means 'take no arguments'
    () => {
        println!("hello!\n")
    };
}

fn macros_inside_functions() {
    macro_rules! inner {
        () => {
            println!("inner called!")
        };
    }

    inner!();
    println!()
}

fn function_factory() {
    macro_rules! create_function {
        ($name: ident) => {
            fn $name() {
                println!("called function {}()", stringify!($name))
            }
        };
    }

    create_function!(foo_a);
    create_function!(foo_b);

    foo_a();
    foo_b();

    println!()
}

fn expression_evaluator() {
    macro_rules! eval {
        ($expression: expr) => {
            println!(
                "evaluating expression:\n{}\n=> {}\n",
                stringify!($expression),
                $expression
            )
        };
    }

    eval!(2 + 2);

    // a block is an expression
    eval!({
        let x = String::from("foo");

        x + &"bar"
    });
}

fn overload() {
    macro_rules! and_or {
        // '; or ' is a template
        ($left: expr; or $right: expr) => {
            println!(
                "{} or {}\n= {}\n",
                stringify!($left),
                stringify!($right),
                $left || $right
            )
        }; // arm ends with a semi-colon

        // '; and ' is a template
        ($left: expr; and $right: expr) => {
            println!(
                "{} and {}\n= {}\n",
                stringify!($left),
                stringify!($right),
                $left && $right
            )
        };

        ($left: expr) => {
            println!("only left given: {}\n= {}", stringify!($left), !!$left)
        };
    }

    and_or!(2 * 1 == 2_i32; or 1 + 2 == 3_i32);
    and_or!(true; and false);
    and_or!("a" == "A");
    println!()
}

fn repetition() {
    macro_rules! zero_or_more {
        () => {
            println!("no arguments\n")
        };

        ($x: expr) => {
            let xs = Vec::from([$x]);

            println!("many arguments: {:?}", xs)
        };

        ($x: expr, $($xs:expr),*) => {
            let xs = Vec::from([$x, $($xs),*]);

            println!("many arguments: {:?}", xs)
        };
    }

    macro_rules! one_or_more {
        ($x: expr) => {
            let xs = Vec::from([$x]);

            println!("one argument: {:?}", xs)
        };

        ($x: expr, $($xs: expr), +) => {
            let xs = Vec::from([$x, $($xs),+]);

            println!("many arguments: {:?}", xs)
        };
    }

    zero_or_more!();
    zero_or_more!(1);
    zero_or_more!(1, 2, 3);
    println!();

    one_or_more!(1);
    one_or_more!(1, 2, 3);
    println!();
}

fn recursion() {
    macro_rules! min {
        ($x: expr) => ( $x );

        ($x: expr, $($xs: expr),+) => {
            core::cmp::min($x, min!($($xs),+))
        }
    }

    println!("min: {:?}", min!(1));
    println!("min: {:?}", min!(3, 6, 2, 37, 35));
    println!()
}

fn calculator_dsl() {
    macro_rules! calculate {
        ($x: expr) => {
            $x
        };

        (debug $x: expr) => {
            println!("debugging expression...");
            println!("{}\n= {}", stringify!($x), $x);
        };
    }

    println!["without debugging: {}\n", calculate! {2 + 2}];
    calculate! ( debug 2+2 );
}

fn main() {
    say_hello!();

    macros_inside_functions();

    // designators
    function_factory();
    expression_evaluator();

    // overloads
    overload();

    // repetition
    repetition();
    recursion();

    // DSL
    calculator_dsl();
}
