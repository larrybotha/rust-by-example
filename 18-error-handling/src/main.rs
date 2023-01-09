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

fn option_map_terse() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Food {
        Apple,
        Orange,
        Banana,
    }

    #[derive(Debug)]
    struct Peeled(Food);
    #[derive(Debug)]
    struct Chopped(Food);
    #[derive(Debug)]
    struct Cooked(Food);

    fn peel(x: Option<Food>) -> Option<Peeled> {
        #[allow(clippy::manual_map)]
        match x {
            Some(food) => Some(Peeled(food)),
            None => None,
        }
    }

    fn chop(x: Option<Peeled>) -> Option<Chopped> {
        #[allow(clippy::manual_map)]
        match x {
            // deconstruct x
            Some(Peeled(food)) => Some(Chopped(food)),
            None => None,
        }
    }

    fn cook(x: Option<Chopped>) -> Option<Cooked> {
        match x {
            // manually get the food out of Chopped using a tuple accessor
            Some(food) => Some(Cooked(food.0)),
            None => None,
        }
    }

    // a single function, using `map` to perform the same work as
    // `match` does in the individual functions above
    fn process(x: Option<Food>) -> Option<Cooked> {
        // Peeled is a function, so no need to use a closure
        x.map(Peeled)
            // manually get the food out of Peeled using a tuple accessor
            .map(|peeled| Chopped(peeled.0))
            // or deconstruct from the argument
            .map(|Chopped(food)| Cooked(food))
    }

    let apple = Some(Food::Apple);
    let orange = Some(Food::Orange);

    // noisy
    let cooked_apple = cook(chop(peel(apple)));
    // terse
    let cooked_orange = process(orange);

    println!("apple: {:?}", cooked_apple);
    println!("orange: {:?}", cooked_orange);
    println!()
}

fn option_and_then() {
    let x = Some(6);
    let nested_x = x.map(Some);
    let flattened_x_a = nested_x.and_then(core::convert::identity);
    let flattened_x_b = nested_x.flatten();

    println!("x: {:?}", x);
    println!("nested_x: {:?}", nested_x);
    println!("flattened_x_a: {:?}", flattened_x_a);
    println!("flattened_x_b: {:?}", flattened_x_b);
    println!()
}

fn option_and_then_terse() {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Food {
        CordonBleu,
        Steak,
        Sushi,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
    }

    fn have_ingredients(food: Food) -> Option<Food> {
        match food {
            // we don't have ingredients for Sushi
            Food::Sushi => None,
            _ => Some(food),
        }
    }

    fn have_recipe(food: Food) -> Option<Food> {
        match food {
            // we don't have a recipe for CordonBleu
            Food::CordonBleu => None,
            _ => Some(food),
        }
    }

    #[allow(dead_code)]
    // we can chain matches, but it gets noisy very quickly
    fn cookable_v1(food: Food) -> Option<Food> {
        match have_recipe(food) {
            None => None,
            #[allow(clippy::manual_map)]
            Some(x) => match have_ingredients(x) {
                None => None,
                Some(y) => Some(y),
            },
        }
    }

    // instead of chaining matches, we can use .and_then on Option to
    // remove one level of nesting of Option, and pass the value to
    // another handler
    fn cookable_v2(food: Food) -> Option<Food> {
        // We can map, and then flatten...
        //have_recipe(food).map(have_ingredients).flatten()

        // or we can use .and_then which does the same thing
        have_recipe(food).and_then(have_ingredients)
    }

    fn eat(food: Food, day: Day) {
        match cookable_v2(food) {
            None => println!("we have no food on {:?}!", day),
            Some(x) => println!("we get to eat {:?} on {:?}", x, day),
        }
    }

    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
    println!()
}

fn option_or() {
    let x = None;
    let y_eager = x.or(Some(5));
    #[allow(clippy::unnecessary_lazy_evaluations)]
    let y_lazy = x.or_else(|| Some(5));

    assert_eq!(y_eager, y_lazy);

    println!("y_eager: {:?}", y_eager);
    println!("y_lazy: {:?}", y_lazy);
    println!()
}

fn option_get_or_insert() {
    let mut x_eager = None;

    println!("x_eager before: {:?}", x_eager);

    x_eager.get_or_insert(5);

    println!("x_eager after: {:?}", x_eager);

    let mut x_lazy = None;

    println!("x_lazy before: {:?}", x_lazy);

    #[allow(clippy::unnecessary_lazy_evaluations)]
    x_lazy.get_or_insert_with(|| 5);

    println!("x_lazy after: {:?}", x_lazy);
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
    option_map_terse();
    option_and_then();
    option_and_then_terse();
    option_or();
    option_get_or_insert();
}
