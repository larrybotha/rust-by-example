// import everything from from a_a.rs, making it public from this file
pub mod a_a;
// import a_b.rs, using it only in this file
mod a_b;

pub fn function() {
    println!("called a::function")
}

pub fn indirect_access() {
    println!("called a::indirect_access");
    println!("calling a_b::function...");
    a_b::function();
}

// nested module inside
pub mod nested {
    pub fn function() {
        println!("called a::nested::function")
    }
}
