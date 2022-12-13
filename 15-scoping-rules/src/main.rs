fn raii_example() {
    fn create_box(value: i32) {
        // _box is an owned value - the i32 is value is stored on the heap
        println!("create_box allocating memory...");
        let _box = Box::new(value);

        println!("create_box freeing memory...")
        // _box goes out of scope here, and memory is freed
    }

    // allocating an i32 to the heap
    println!("_box_1 coming into scope");
    let _box_1 = Box::new(5);

    {
        // allocating another i32 to the heap
        println!("_box_2 coming into scope");
        let _box_2 = Box::new(6);

        println!("_box_2 going out of scope");
        // _box_2 goes out of scope here, is destroyed, and memory is freed
    }

    // creating many boxes - no need to free memory
    for x in 0i32..10 {
        create_box(x);
    }

    println!("_box_1 going out of scope");
    println!();
    // _box_1 goes out of scope here, is destroyed, and memory is freed
}

fn custom_drop() {
    use std::mem;

    struct MyDropStruct(i32);

    // custom drop logic
    impl Drop for MyDropStruct {
        fn drop(&mut self) {
            println!("value uses {} bytes of memory", mem::size_of_val(self));
            println!("we're dropping... weeeeee!!!")
        }
    }

    {
        let _value = MyDropStruct(42);
    }

    println!()
}

fn main() {
    raii_example();
    custom_drop();
}
