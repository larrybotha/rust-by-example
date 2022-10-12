use std::io;
use std::mem;

fn underscored_integers() {
    let x = 20_000_000;
    let y = 0.000_000_01;

    println!("20_000_000 prints as {x}");
    println!(" 0.000_000_01 prints as {y}");
    println!();
}

fn isize_usize_for_indexing() {
    let xs = [1, 2, 3, 4, 5];
    let index: usize = 3; // <= index is a usize
    let value = &xs[index];

    println!("xs at index {index}: {value:?}");
    println!();
}

fn addition_and_subtraction() {
    println!("{}", 42u32);
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!();
}

fn division() {
    println!("integer division: 2 / 3 = {}", 2 / 3);
    println!("floating point division: 2.0 / 3.0 = {}", 2.0 / 3.0);
    println!();
}

fn boolean_logic() {
    println!("AND with &&: {}", true && false);
    println!("OR with ||: {}", true || false);
    println!("NOT with !: {}", !true);
    println!();
}

fn bitwise_operations() {
    println!("1101 AND 1010: {:04b}", 0b1101 & 0b1010);
    println!("1101 OR 1010: {:04b}", 0b1101 | 0b1010);
    println!("1101 XOR 1010: {:04b}", 0b1101 ^ 0b1010);
    println!("left shift: 1 << 4: {}", 1u32 << 4);
    println!("right shift: 0x80 >> 4: {}", 0x80u32 >> 4);
    println!();
}

fn destructuring_tuples() {
    let tup = ("hey", 'o', 3);
    let (x, y, z) = tup;

    println!("tuple: {tup:?}\n x: {x}, y: {y}, z: {z}");
    println!();
}

fn functions_returning_tuples() -> (i32, &'static str) {
    let tup = (1, 'a', "foo");

    println!("{tup:?}");

    let (x, _, z) = tup;

    (x, z)
}

fn single_value_tuples_with_comma() {
    let tup = (1,);
    let not_tup = (1);

    println!("tuple: {tup:?}");
    println!("not tuple: {not_tup:?}");
    println!();
}

fn tuple_exercise() {
    fn transpose(matrix: &Matrix) -> Matrix {
        let Matrix(a, b, c, d) = matrix;

        Matrix(*a, *c, *b, *d)
    }

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    impl std::fmt::Display for Matrix {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let Matrix(x1, x2, y1, y2) = &self;

            write!(f, "({x1:?} {x2:?})\n({y1:?} {y2:?})",)
        }
    }

    let matrix = Matrix(1.0, 1.1, 2.0, 2.2);
    let transpose_matrix = transpose(&matrix);

    println!("matrix: {matrix:?}");
    println!("{matrix}\n");
    println!("tranpose matrix: {matrix:?}");
    println!("{transpose_matrix}");
    println!();
}

fn array_definition_without_binding() {
    let xs: [i8; 3];

    // we can bind xs here
    xs = [1, 2, 3];

    println!("{xs:?}");
    println!();
}

fn array_initialisation() {
    let xs = [3; 5];

    println!("{xs:?}");
    println!();
}

fn index_error_inputter() {
    let xs = [1, 2, 3, 4, 5];

    loop {
        println!("\nEnter an index for the array of length: {}", xs.len());
        let mut index = String::new(); // => unintialised

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = match index.trim().parse() {
            Ok(n) => n,
            Err(n) => {
                eprintln!("{n}");
                continue;
            }
        };
        let element = xs[index]; // <= crashses if out of bounds

        println!("value at index {index} is {element}");
    }
}

fn size_of_array() {
    let xs = [0; 500];

    println!("size of xs in bytes: {}", mem::size_of_val(&xs));
    println!();
}

fn slices_as_sections_of_array() {
    let mut temp = 0;
    let xs: [i32; 500] = [0; 500].map(|x| {
        temp += 1;
        temp
    });
    let start = 4;
    let end = 10;
    let slice = &xs[start..=end]; // => value is borrowed here

    println!("slice from {start} to {end}: {slice:?}");
    println!();
}

fn safely_reference_arrays_with_get() {
    let xs = [1, 2, 3];
    let index = 4;

    match xs.get(index) {
        Some(x) => {
            println!("found {x}");
        }
        None => {
            eprintln!("oops - too far!");
        }
    };

    let value = match xs.get(index) {
        Some(x) => x,
        None => &-1,
    };
    println!("value at index {index} is {value}");

    let value = xs.get(index).unwrap_or(&-1);
    println!("unwrapped: value at index {index} is {value}");
    println!();
}

fn main() {
    underscored_integers();
    isize_usize_for_indexing();
    addition_and_subtraction();
    division();
    boolean_logic();
    bitwise_operations();

    println!("---");

    // tuples
    destructuring_tuples();
    functions_returning_tuples();
    single_value_tuples_with_comma();
    tuple_exercise();

    println!("---");

    // arrays
    array_definition_without_binding();
    array_initialisation();
    size_of_array();
    slices_as_sections_of_array();
    safely_reference_arrays_with_get();
    index_error_inputter();
}
