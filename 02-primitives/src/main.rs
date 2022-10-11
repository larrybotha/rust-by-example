fn underscored_integers() {
    let x = 20_000_000;
    let y = 0.000_000_01;

    println!("20_000_000 prints as {x}");
    println!(" 0.000_000_01 prints as {y}");
    println!();
}

fn addition_and_subtraction() {
    println!("{}", 42u32);
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
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
}

fn main() {
    underscored_integers();
    addition_and_subtraction();
    boolean_logic();
    bitwise_operations();
}
