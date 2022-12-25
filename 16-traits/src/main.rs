use rand::random;
use std::default::Default;

fn internal_access() {
    trait MyTrait {
        fn method_a(&self) -> &Self {
            println!("method_a called!");
            println!("calling self.method_b...");

            self.method_b();

            self
        }

        fn method_b(&self) -> &Self {
            println!("method_b called!");

            self
        }
    }

    struct MyStruct {}

    impl MyTrait for MyStruct {}

    let x = MyStruct {};

    x.method_a();
    println!();
}

fn instantiate_via_trait() {
    trait Animal {
        fn new(name: &'static str) -> Self;
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise())
        }
    }

    struct Sheep {
        name: &'static str,
        naked: bool,
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) -> &mut Self {
            self.naked = if !self.naked { true } else { self.naked };

            self
        }
    }

    impl Animal for Sheep {
        fn new(name: &'static str) -> Self {
            Self { name, naked: false }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.naked {
                "baaaa!!!"
            } else {
                "baaaa?"
            }
        }
    }

    // we can instantiate via a trait by annotating the type
    let mut dolly: Sheep = Animal::new("Dolly");

    println!("is {} naked? {}", dolly.name(), dolly.is_naked());
    dolly.talk();
    println!("let's shave {}...", dolly.name());

    dolly.shear();

    println!("is {} naked now? {}", dolly.name(), dolly.is_naked());
    dolly.talk();
    println!();
}

fn default_trait() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Default for Point {
        fn default() -> Self {
            Self { x: 1, y: 1 }
        }
    }

    let point = Point::default();

    println!("point: {point:?}");
    println!()
}

#[allow(clippy::eq_op, clippy::cmp_nan)]
fn nan_never_equal() {
    const NAN: f64 = f64::NAN;

    println!("f64::NAN == f64::NAN: {}", NAN == NAN);
    println!()
}

fn derive_equality_comparison() {
    // Eq can be derived, as there are no values in i32
    // that are not equal to themselves
    #[derive(Debug, PartialEq, Eq)]
    struct Integer(i32);

    // Eq cannot be derived for f64 - NAN != NAN ever
    #[derive(Debug, PartialEq)]
    struct Float(f64);

    let x = Integer(5);
    let y = Integer(6);
    let z = Integer(5);

    assert_ne!(x, y);
    assert_eq!(x, z);
    println!("x.eq(&y): {}", x.eq(&y));
    println!("x == y: {}", x == y);
    println!()
}

fn derive_ordinal_comparison() {
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Metre(f64);

    use std::cmp::Ordering;

    let x = 7;
    let y = 6;

    assert!(x > y);
    assert_eq!(x.cmp(&y), Ordering::Greater);
    assert!(x.gt(&y));
    println!("x.cmp(&y): {:?} == {:?}", x.cmp(&y), Ordering::Greater);
    println!("x.gt(&y): {}", x.gt(&y));
    println!("x > y: {}", x > y);
    println!();
}

fn dynamic_trait_no_struct() {
    trait Printable {
        fn stringify(&self) -> String;
    }

    // implement Printable for i32
    impl Printable for i32 {
        fn stringify(&self) -> String {
            self.to_string()
        }
    }

    fn print_using_bound<T: Printable>(x: T) {
        println!("printing x using trait object: {}", x.stringify())
    }

    // we don't know what the concrete type for x is, but we do know that
    // it must implement Printable
    fn print_using_trait_object(x: Box<dyn Printable>) {
        println!("printing x using trait object: {}", x.stringify())
    }

    print_using_bound(6);
    print_using_trait_object(Box::new(6));

    println!();
}

fn return_trait_with_dyn() {
    //use rand;

    trait Animal {
        fn noise(&self) -> &'static str;
        fn name(&self) -> &'static str;
    }

    #[derive(Debug)]
    struct Cow {}

    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "mooooooooooooo"
        }

        fn name(&self) -> &'static str {
            "Cow"
        }
    }

    #[derive(Debug)]
    struct AnglerFish {}

    impl Animal for AnglerFish {
        fn noise(&self) -> &'static str {
            "bloop bloop"
        }

        fn name(&self) -> &'static str {
            "Angler Fish"
        }
    }

    fn random_animal(x: f64) -> Box<dyn Animal> {
        if x > 0.5 {
            Box::new(Cow {})
        } else {
            Box::new(AnglerFish {})
        }
    }

    let rand_float = random();
    let animal = random_animal(rand_float);

    println!("random value is {}", rand_float);
    println!("{} goes {}", animal.name(), animal.noise());

    println!();
}

fn main() {
    internal_access();
    instantiate_via_trait();

    // deriving
    default_trait();
    nan_never_equal();
    derive_equality_comparison();
    derive_ordinal_comparison();

    // dyn
    dynamic_trait_no_struct();
    return_trait_with_dyn();
}
