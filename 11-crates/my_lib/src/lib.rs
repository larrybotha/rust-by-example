pub fn public_function() {
    println!("called my_library's public_function")
}

fn private_function() {
    println!("called my_library's private_function")
}

pub fn indirect_access() {
    println!("called my_library's indirect_access");
    println!("calling my_library's private_function...");
    private_function();
}
