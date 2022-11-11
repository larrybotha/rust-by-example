use crate::a::a_a;
// Unable to import a_b - it is private, as it is made public in a.rs
//use crate::a::a_b;

pub fn function() {
    println!("called b::function");
}

pub fn indirect_access() {
    println!("called b::indirect_access");
    println!("calling a_a::function...");
    a_a::function();
}
