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

fn iterator_from_array() {
    let xs = [1u32, 2, 3, 4];

    for x in xs.iter() {
        println!("x is {}", x);
    }

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

    println!("> skipping 4, and taking 4...");

    let xs = fib.skip(4).take(4);

    for x in xs {
        println!("x: {x}");
    }

    println!()
}

fn impl_trait_as_argument() {
    // bounds
    fn print_using_bounds<T: std::fmt::Debug>(x: T) {
        println!("debug using bounds: {x:?}");
    }

    // impl trait as argument
    fn print_using_impl_trait(x: impl std::fmt::Debug) {
        println!("debug using impl trait: {x:?}");
    }

    let x = 5_i32;

    print_using_bounds::<i32>(x);

    // turbofish syntax is invalid when using 'impl Trait'
    //print_using_impl_trait::<i32>(x);
    print_using_impl_trait(x);
    println!()
}

fn impl_trait_as_return() {
    // return type defined using bound and 'where'
    fn id_as_bounded_return<T>(x: T) -> T
    where
        T: std::ops::Add<Output = T> + std::fmt::Debug,
    {
        x
    }

    // return type defined using 'impl Trait'
    fn id_as_impl_trait_return(
        x: impl std::ops::Add + std::fmt::Debug,
    ) -> impl std::ops::Add + std::fmt::Debug {
        x
    }

    let x = id_as_bounded_return::<i8>(1);
    let y = id_as_impl_trait_return(1);

    // as with 'impl Trait' arguments, it is invalid to use turbofish to
    // coerce types with
    //let z = id_as_impl_trait_return::<i8>(1);

    println!("x: {x:?}, y: {y:?}");
    println!()
}

fn impl_trait_argument_real_world() {
    // using bounds
    fn parse_csv_using_bounds<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
        src.lines()
            .map(|line_result| {
                line_result.map(|row| {
                    row.split(',')
                        .map(|field| String::from(field.trim()))
                        .collect()
                })
            })
            .collect()
    }

    // using impl Trait
    fn parse_csv_using_impl_trait(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
        src.lines()
            .map(|line_result| {
                line_result.map(|row| {
                    row.split(',')
                        .map(|field| String::from(field.trim()))
                        .collect()
                })
            })
            .collect()
    }

    let csv_data = "a,b,c\nd,e,f";
    let csv_x = std::io::Cursor::new(csv_data);
    let csv_y = csv_x.clone();

    assert_eq!(csv_x, csv_y);
    println!("csv_x: {:?}", parse_csv_using_bounds(csv_x));
    println!("csv_y: {:?}", parse_csv_using_impl_trait(csv_y));
    println!();
}

fn impl_trait_return_real_world() {
    use std::iter;
    use std::vec::IntoIter;

    // explicit return
    fn chain_and_cycle_explicit(
        u: Vec<i32>,
        v: Vec<i32>,
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        u.into_iter().chain(v.into_iter()).cycle()
    }

    // return using impl Trait
    fn chain_and_cycle_impl_trait(u: Vec<i32>, v: Vec<i32>) -> impl Iterator<Item = i32> {
        u.into_iter().chain(v.into_iter()).cycle()
    }

    let xs = Vec::from([1, 2]);
    let ys = Vec::from([3, 4]);
    let mut iter_a = chain_and_cycle_explicit(xs.clone(), ys.clone());
    let mut iter_b = chain_and_cycle_impl_trait(xs, ys);

    { 0..6 }
        .map(|i| {
            let (x, y) = (iter_a.next(), iter_b.next());

            println!("iteration {} -> iter_a: {:?}", i, x);
            println!("iteration {} -> iter_b: {:?}\n", i, y);
        })
        .for_each(drop)
}

fn supertraits() {
    trait Nameable {
        fn name(&self) -> String;
    }

    // Classable is a subtrait, with Nameable as its supertrait
    trait Classable: Nameable {
        fn class(&self) -> String;
    }

    // Indexable is a subtrait of both Nameable and Classable
    trait Indexable: Nameable + Classable {
        fn index(&self) -> i32;
    }

    #[derive(Debug)]
    struct Animal {
        name: String,
        class: String,
        index: i32,
    }

    impl Animal {
        fn new(name: String) -> Self {
            Self {
                name,
                class: "Unknown".to_string(),
                index: 0,
            }
        }
    }

    // Animal must implement Nameable
    impl Nameable for Animal {
        fn name(&self) -> String {
            self.name.to_string()
        }
    }

    // Animal must implement Classable
    impl Classable for Animal {
        fn class(&self) -> String {
            self.class.to_string()
        }
    }

    // Animal must implement Indexable
    impl Indexable for Animal {
        fn index(&self) -> i32 {
            self.index
        }
    }

    let animal = Animal::new("Goat".to_string());

    println!("animal: {:?}", animal);
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
    iterator_from_array();
    iterator_from_impl();

    // impl Trait
    impl_trait_as_argument();
    impl_trait_as_return();
    impl_trait_argument_real_world();
    impl_trait_return_real_world();

    // supertraits
    supertraits();
}
