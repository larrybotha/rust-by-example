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

fn main() {
    generic_structs();
    generic_functions();
}
