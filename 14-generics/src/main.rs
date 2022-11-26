use std::any::type_name;
use std::f32::consts::PI;
use std::fmt::Display;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn generic_structs() {
    // concrete
    struct A;
    // concrete
    struct Single(A);
    // generic
    struct SingleGeneric<T>(T);

    let a = A;
    let single_a = Single(A);
    let single_generic_a = SingleGeneric(A);
    let single_nested_single_a = SingleGeneric(Single);
    let single_generic_i8: SingleGeneric<i8> = SingleGeneric(4);

    println!("type of 'a' is:\n\t{:>10}\n", type_of(a));
    println!("type of 'single_a' is:\n\t{:>10}\n", type_of(single_a));
    println!(
        "type of 'single_generic_a' is:\n\t{:>10}\n",
        type_of(single_generic_a)
    );
    println!(
        "type of 'single_nested_single_a' is:\n\t{:>10}\n",
        type_of(single_nested_single_a)
    );
    println!(
        "type of 'single_generic_i8' is:\n\t{:>10}\n",
        type_of(single_generic_i8)
    );
    println!();
}

fn generic_functions() {
    struct GenStruct<T>(T);

    fn generic<T>(x: GenStruct<T>) {
        println!("{}", type_of(x));
    }

    generic(GenStruct(5)); // implicitly specified type parameter -> i32
    generic::<i8>(GenStruct(5)); // explicitly specified type parameter -> i8
    println!();
}

fn generic_implementation() {
    #[derive(Debug)]
    struct GenStruct<T> {
        val: T,
    }

    impl<T> GenStruct<T> {
        fn value(self) -> T {
            self.val
        }
    }

    let x = GenStruct::<i8> { val: 5 };
    let y = GenStruct { val: 5i16 };
    let z = GenStruct { val: "foo" };

    println!("x: {:?}", type_of(x.value()));
    println!("y: {:?}", type_of(y.value()));
    println!("z: {:?}", type_of(z.value()));
    println!();
}

fn generic_traits() {
    struct Empty;
    struct Null;

    // A trait that is generic over T
    trait DoubleDrop<T> {
        fn drop(self, _: T);
    }

    // implement the DoubleDrop trait, given a generic caller U
    impl<T, U> DoubleDrop<T> for U {
        fn drop(self, _: T) {}
    }

    let x = Empty;
    let null = Null;

    x.drop(null);
    println!();
}

fn generic_bounds() {
    struct BoundedType<T: Display> {
        //                  [1]
        // 1 - T has the bound 'Display'
        value: T,
    }

    fn do_the_print<T: Display>(value: T) {
        println!("{}", type_of(value))
    }

    let x = BoundedType { value: "foo" };
    let y = 2_i16;

    do_the_print(x.value);
    do_the_print(y);

    println!();
}

fn generic_bounds_methods() {
    trait HasArea {
        fn area(&self) -> f32;
    }

    struct Circle {
        radius: i32,
    }

    impl Display for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Circle {{{}}}", self.radius)
        }
    }

    impl HasArea for Circle {
        fn area(&self) -> f32 {
            PI * self.radius.pow(2) as f32
        }
    }

    struct Rectangle {
        width: i32,
        breadth: i32,
    }

    impl Display for Rectangle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Rectangle {{{} x {}}}", self.width, self.breadth)
        }
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f32 {
            (self.breadth * self.width) as f32
        }
    }

    fn print_area<T: HasArea + Display>(shape: &T) {
        // we can use .area here because the bound HasArea means the type
        // is guaranteed to have the method
        let area = shape.area();

        println!("{shape} has area {area}")
    }

    let rectangle = Rectangle {
        breadth: 6,
        width: 5,
    };
    let circle = Circle { radius: 5 };

    print_area(&rectangle);
    print_area(&circle);
    println!();
}

fn generic_bounds_empty_traits() {
    struct NotFoo {}

    trait EmptyTrait {}

    #[derive(Debug)]
    struct Foo {}

    impl EmptyTrait for Foo {}

    fn my_bound<T: EmptyTrait + std::fmt::Debug>(value: &T) {
        println!("Bounded! Value implements EmptyTrait: {:?}", value)
    }

    let x = Foo {};
    let _y = NotFoo {};

    my_bound(&x);

    // _y does not implement EmptyTrait
    //my_bound(&_y);
    println!()
}

fn where_clause_for_readability() {
    trait TraitA {}
    trait TraitB {}
    trait TraitC {}
    trait TraitD {}

    #[derive(Debug)]
    struct X;

    impl TraitA for X {}
    impl TraitB for X {}

    #[derive(Debug)]
    struct Y;

    impl TraitC for Y {}
    impl TraitD for Y {}

    fn foo<T: TraitA + TraitB + std::fmt::Debug, U: TraitC + TraitD + std::fmt::Debug>(
        x: &T,
        y: &U,
    ) {
        println!("{x:?} {y:?}")
    }

    // bar has the same type signature, but the `where` clause makes it
    // easier to read
    fn bar<T, U>(x: &T, y: &U)
    where
        T: TraitA + TraitB + std::fmt::Debug,
        U: TraitC + TraitD + std::fmt::Debug,
    {
        println!("{x:?} {y:?}")
    }

    let x = X;
    let y = Y;

    foo(&x, &y);
    bar(&x, &y);

    println!()
}

fn where_clause_when_required() {
    trait Debuggable {
        fn debug_in_option(self);
        fn get_debug_string(self) -> String;
    }

    impl<T> Debuggable for T
    // we can't set this bound where T is first mentioned, because
    // we want the bound specified for Option<T>, as that is what
    // we're printing
    where
        Option<T>: std::fmt::Debug,
    {
        fn debug_in_option(self) {
            println!("{:?}", Some(self))
        }

        fn get_debug_string(self) -> String {
            // Without the bound on Option, we wouldn't be able to use
            // Some(self) here
            // Furthermore, to use `self`, we'd need a bound on T, too
            let result = format!("{:?}", Some(self));

            result
        }
    }

    let xs = vec![1, 2, 3];
    let ys = vec![1, 2, 3];

    xs.debug_in_option();

    println!("{}", ys.get_debug_string());
    println!();
}

fn generic_associated_types_before() {
    #[derive(Debug)]
    struct Container(i32, i32);

    // A trait with 2 generic types
    trait Contains<A, B> {
        // determine if an instance that implements this traiit
        // contains 2 values
        fn contains(&self, _: &A, _: &B) -> bool;
        // get the first value from the instance
        // - no need for A or B
        fn first(&self) -> i32;
        // get the last value from the instance
        // - no need for A or B
        fn last(&self) -> i32;
    }

    impl Contains<i32, i32> for Container {
        fn contains(&self, x: &i32, y: &i32) -> bool {
            &self.0 == x && &self.1 == y
        }

        fn first(&self) -> i32 {
            self.0
        }

        fn last(&self) -> i32 {
            self.1
        }
    }

    // A, B, and C need to be specified
    fn difference<A, B, C>(container: &C) -> i32
    where
        C: Contains<A, B>,
    {
        container.last() - container.first()
    }

    let x = Container(4, 2);

    println!("x contains 4 and 2: {:?}", x.contains(&4, &2));
    println!("x contains 5 and 2: {:?}", x.contains(&5, &2));
    println!("x difference: {:?}", difference(&x));
    println!();
}

fn generic_associated_types_after() {
    #[derive(Debug)]
    struct Container(i32, i32);

    trait Contains {
        // 'Contains' is a trait with 2 associated types,
        // i.e. output types
        type A;
        type B;

        // use Self::A to reference associated type
        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;

        fn first(&self) -> i32;

        fn last(&self) -> i32;
    }

    impl Contains for Container {
        // 'Container' implements 'Contains', and is thus the input type.
        // 'Container' is defined with 2 i32 values, which are thus 'Contains's
        // output types
        type A = i32;
        type B = i32;

        // we can either use the type, or Self::[associated_type] to specify
        // the output type
        fn contains(&self, x: &Self::A, y: &i32) -> bool {
            &self.0 == x && &self.1 == y
        }

        fn first(&self) -> i32 {
            self.0
        }

        fn last(&self) -> i32 {
            self.1
        }
    }

    // because 'Contains' now uses associated types, and 'Container' needs to
    // implmeent the types, we no longer need to specify all the types
    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

    let x = Container(4, 2);

    println!("x contains 4 and 2: {:?}", x.contains(&4, &2));
    println!("x contains 5 and 2: {:?}", x.contains(&5, &2));
    println!("x difference: {:?}", difference(&x));
    println!();
}

fn main() {
    generic_structs();
    generic_functions();

    generic_implementation();
    generic_traits();
    generic_bounds();
    generic_bounds_methods();
    generic_bounds_empty_traits();

    where_clause_for_readability();
    where_clause_when_required();

    generic_associated_types_before();
    generic_associated_types_after();
}
