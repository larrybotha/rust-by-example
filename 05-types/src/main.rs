use std::mem::size_of_val;

fn casting_explicit() {
    let x = 5_i8;
    let y = x as f32;

    println!("{y:?}");
    println!();
}

fn casting_numbers_to_char() {
    let x = 5_u8;
    let _y = 8.2;
    let x_char = x as char;
    //let y_char = y as char; // <= invalid - a float cannot be cast to a char
    let char_from_u32 = char::from_u32(8).unwrap();
    let char_from_digit = char::from_digit(9, 10).unwrap();

    println!("{x_char}");
    println!("{char_from_u32:?}");
    println!("{char_from_digit:?}");
    println!();
}

fn max_and_min_numbers() {
    let min_i8 = i8::MIN;
    let max_i8 = i8::MAX;

    println!("min_i8: {min_i8}");
    println!("max_i8: {max_i8}");
    println!();
}

fn casting_to_unsigned_types() {
    println!("300 as u8: {}", 300_i32 as u8);
    println!("-300 as u8: {}", -300_i32 as u8);
    println!();

    println!("130 as i8: {}", 130_i32 as i8);
    println!("-130 as i8: {}", -130_i32 as i8);
    println!();
}

fn nan_values() {
    println!("f32::NAN: {}", f32::NAN);
    println!("f64::NAN: {}", f64::NAN);
    println!("f32::NAN as u8: {}", f32::NAN as u8);
    println!("f32::NAN as i8: {}", f32::NAN as i8);
    println!();
}

fn size_of_numeric_literals() {
    println!("size of i8 in bytes: {}", size_of_val(&1i8));
    println!("size of u8 in bytes: {}", size_of_val(&1u8));
    println!("size of i16 in bytes: {}", size_of_val(&1i16));
    println!("size of i32 in bytes: {}", size_of_val(&1i32));
    println!("size of i64 in bytes: {}", size_of_val(&1i64));
    println!("size of f32 in bytes: {}", size_of_val(&1f32));
    println!("size of f64 in bytes: {}", size_of_val(&1f64));
    println!("size of usize in bytes: {}", size_of_val(&1usize));
    println!("size of isize in bytes: {}", size_of_val(&1isize));
    println!();
}

fn defining_type_aliases() {
    #![allow(dead_code)]

    #[derive(Debug)]
    struct MyReallyLongStructName;

    type MyStruct = MyReallyLongStructName;
    type SmallInt = i8;
}

fn main() {
    casting_explicit();
    casting_numbers_to_char();
    max_and_min_numbers();
    casting_to_unsigned_types();
    nan_values();

    size_of_numeric_literals();

    defining_type_aliases();
}
