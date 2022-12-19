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

fn copy_into() {
    fn do_something(x: u32) {
        println!("doing something with x: {}", x)
    }

    // stack-allocated integer
    let x = 42u32;

    do_something(x);

    println!("x is still accessible: {}", x);
    println!();
}

fn move_into() {
    fn do_something(x: Box<u32>) {
        println!("doing something with x: {}", x);
        // x destroyed here via 'drop'
    }

    // create a heap-allocated integer
    let x = Box::new(42);

    do_something(x);

    println!("x may not be referenced any longer");
    println!()
}

fn change_of_ownership_and_mutability() {
    let immutable_box = Box::<i32>::new(42);
    let mut mutable_box = immutable_box; // by reassigning, we can make the value mutable

    println!("mutable box: {}", mutable_box);

    // mutate the contents of the box
    *mutable_box *= 4;

    println!("mutable box: {}", mutable_box);
    println!();
}

fn partial_moves() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<i32>,
    }

    let person = Person {
        name: String::from("sam"),
        age: Box::<i32>::new(42),
    };
    let Person { name, ref age } = person;

    // name is moved here, and then dropped
    println!("person's name is {}", name);

    // age is referenced here
    println!("person's age is {}", age);

    // we are not allowed to reference `person` here - it has been
    // partially moved
    //println!("person's age is {:?}", person);

    println!()
}

fn borrow_and_destroy() {
    fn i_will_destroy(x: Box<i32>) {
        println!("x is {} and about to be destroyed", x);
    }

    fn i_will_borrow(x: &i32) {
        println!("x is {} and borrowed", x);
    }

    let x_heaped = Box::<i32>::new(42);
    let x_stacked = 43;

    i_will_borrow(&x_heaped);
    i_will_borrow(&x_stacked);

    println!();

    {
        let ref_x_heaped = &x_heaped;

        // we cannot destroy x_heaped here, as it has a reference further below
        //i_will_destroy(x_heaped);

        i_will_borrow(ref_x_heaped);
    }

    // now that we no longer have any references to x_heap, it is safe to
    // let it be moved and destroyed
    i_will_destroy(x_heaped);

    println!()
}

fn mutable_borrows() {
    #[derive(Debug)]
    struct Book {
        // title and author are references to read-only memory:
        //  - read-only, because they are of type str
        //  - references, because of the ampersand
        title: &'static str,
        author: &'static str,
        year: i32,
    }

    fn read_title(book: &Book) {
        let Book { author, title, .. } = book;

        println!("{title} by {author}")
    }

    fn set_year(book: &mut Book, year: i32) -> &mut Book {
        book.year = year;

        book
    }

    let mut mutable_book = Book {
        title: "Harry Potter",
        author: "J K Rowling",
        year: 1990,
    };
    let immutable_book = Book {
        title: "Moby Dick",
        author: "Bett Midler",
        year: 1921,
    };

    // pass the value through, indicating it's an immutable reference
    read_title(&mutable_book);
    println!("mutable book before: {:?}", mutable_book);

    // specify explicitly that we are passing through a mutable reference
    set_year(&mut mutable_book, 1988);
    println!("mutable book after: {:?}\n", mutable_book);

    read_title(&immutable_book);

    // we are unable to pass the book in as a mutable reference, as the variable
    // was defined as mutable
    //set_year(&mut immutable_book, 1918);

    println!()
}

fn aliasing() {
    #[derive(Debug, Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    fn print_point(point: &Point) {
        println!("points are ({} {})", point.x, point.y)
    }

    fn shift_point(point: &mut Point) -> &mut Point {
        point.x += 1;
        point.y += 1;

        point
    }

    let mut point = Point { x: 5, y: 10 };
    let borrow_a = &point;
    let borrow_b = &point;

    print_point(borrow_a);
    print_point(borrow_b);

    {
        // mutably borrow here - any borrows before this mutable borrow may
        // not be referenced again, and no new references may be created
        // until the last reference to this mutable borrow
        let mutable_borrow = &mut point;

        //print_point(borrow_a);
        //print_point(borrow_b);

        shift_point(mutable_borrow);
        print_point(mutable_borrow);
    }

    // borrow_a and borrow_b may not be referenced here any longer, because
    // a mutable borrow was defined after they were defined
    //print_point(borrow_a);
    //print_point(borrow_b);

    // we can create new immutable borrows here, as mutable_borrow is no
    // longer referenced
    let borrow_c = &point;
    let borrow_d = &point;

    print_point(borrow_c);
    print_point(borrow_d);

    println!()
}

fn main() {
    // RAII
    raii_example();
    custom_drop();

    // ownership
    copy_into();
    move_into();
    change_of_ownership_and_mutability();
    partial_moves();

    // borrowing
    borrow_and_destroy();
    mutable_borrows();
    aliasing();
}
