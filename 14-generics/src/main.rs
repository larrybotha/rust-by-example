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

fn main() {
    generic_structs();
    generic_functions();

    generic_implementation();
    generic_traits();
    generic_bounds();
    generic_bounds_methods();
}
