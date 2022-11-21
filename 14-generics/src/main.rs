use std::any::type_name;

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

fn main() {
    generic_structs();
    generic_functions();

    generic_implementation();
}
