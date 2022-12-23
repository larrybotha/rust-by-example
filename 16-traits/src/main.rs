fn internal_access() {
    trait MyTrait {
        fn method_a(&self) -> &Self {
            println!("method_a called!");
            println!("calling self.method_b...");

            self.method_b();

            self
        }

        fn method_b(&self) -> &Self {
            println!("method_b called!");

            self
        }
    }

    struct MyStruct {}

    impl MyTrait for MyStruct {}

    let x = MyStruct {};

    x.method_a();
    println!();
}

fn main() {
    internal_access();
}
