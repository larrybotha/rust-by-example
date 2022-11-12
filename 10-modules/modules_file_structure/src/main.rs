mod a;
mod b;

fn function() {
    println!("called function")
}

fn main() {
    function();
    a::function();
    a::indirect_access();
    a::nested::function();
    a::a_a::function();

    b::function();
    b::indirect_access();
}
