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

    { 0..3 }
        .into_iter()
        .map(|_| {
            let rand_float = random();
            let animal = random_animal(rand_float);

            (rand_float, animal)
        })
        .map(|(x, animal)| {
            println!("random value is {}", x);
            println!("{} goes {}\n", animal.name(), animal.noise());
        })
        // use .for_each(drop) to consume an iterator and throw away the result
        .for_each(drop);
}

fn operator_between_types() {
    #[derive(Debug)]
    struct Inches(f64);

    impl std::fmt::Display for Inches {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}inches", self.0)
        }
    }

    impl std::ops::Add<Centimeters> for Inches {
        type Output = Self;

        fn add(self, rhs: Centimeters) -> Self::Output {
            let other_inches = rhs.0 / 2.45;

            Self(self.0 + other_inches)
        }
    }

    #[derive(Debug)]
    struct Centimeters(f64);

    impl std::fmt::Display for Centimeters {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}cm", self.0)
        }
    }

    impl std::ops::Add<Inches> for Centimeters {
        type Output = Self;

        fn add(self, rhs: Inches) -> Self::Output {
            let other_cm = rhs.0 * 2.45;

            Self(self.0 + other_cm)
        }
    }

    let cms = Centimeters(2.0);
    let inches = Inches(1.0);

    println!("cms + inches = {}", cms + inches);

    let cms = Centimeters(2.0);
    let inches = Inches(1.0);

    println!("inches + cms = {}", inches + cms);
    println!()
}

fn operator_within_type_with_refs() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Person {
        first_name: String,
        last_name: String,
        age: i32,
    }

    impl<'a, 'b> std::ops::Add<&'b Person> for &'a Person {
        type Output = Person;

        fn add(self, rhs: &'b Person) -> Self::Output {
            let Person {
                first_name: l_first_name,
                last_name: l_last_name,
                ..
            } = self;
            let Person {
                first_name: r_first_name,
                last_name: r_last_name,
                ..
            } = rhs;

            Person {
                age: 0,
                first_name: [l_first_name, r_first_name].map(String::from).join("-"),
                last_name: [l_last_name, r_last_name].map(String::from).join("-"),
            }
        }
    }

    let joe = Person {
        first_name: "John".to_owned(),
        last_name: String::from("Smith"),
        age: 26,
    };
    let sam = Person {
        first_name: "Sam".to_string(),
        last_name: "Doe".to_string(),
        age: 24,
    };
    let child_a = &joe + &sam;
    let child_b = &sam + &joe;

    println!("Child a is {child_a:?}");
    println!("Child b is {child_b:?}");
    println!()
}

fn drop_with_print() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct DropStruct {
        name: &'static str,
    }

    impl Drop for DropStruct {
        fn drop(&mut self) {
            println!("dropping {:?}", self)
        }
    }

    fn drop_warning(x: &DropStruct) {
        println!("{:?} is about to be dropped", x)
    }

    let x = DropStruct { name: "x" };

    {
        let y = DropStruct { name: "y" };

        {
            let z = DropStruct { name: "z" };

            drop_warning(&z);
        } // z no longer in scope

        drop_warning(&y);
    } // y no longer in scope

    drop_warning(&x);
    drop(x);

    println!()
}

fn iterator_from_range() {
    let mut range = 0..3;

    println!("next is {:?}", &range.next());
    println!("next is {:?}", &range.next());
    println!("next is {:?}", &range.next());
    println!("next is {:?}", &range.next());

    println!()
}

fn iterator_from_impl() {
    #[derive(Debug)]
    struct Fibonnacci {
        current: u32,
        next: u32,
    }

    impl Iterator for Fibonnacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let Self { current, next } = self;
            let (current, next) = (*next, *current + *next);

            self.current = current;
            self.next = next;

            Some(current)
        }
    }

    impl std::default::Default for Fibonnacci {
        fn default() -> Self {
            Self {
                current: 0,
                next: 1,
            }
        }
    }

    let mut fib = Fibonnacci::default();

    for _ in 0..10 {
        println!("fib: {:?}", fib.next());
    }

    println!()
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

    // operators
    operator_between_types();
    operator_within_type_with_refs();

    // drop
    drop_with_print();

    // iterators
    iterator_from_range();
    iterator_from_impl();
}
