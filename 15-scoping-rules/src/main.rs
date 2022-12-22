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

    let (x_heaped, x_stacked) = (Box::<i32>::new(42), 43);

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
    let (borrow_a, borrow_b) = (&point, &point);

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

#[allow(clippy::toplevel_ref_arg)]
fn ref_ampersand_equivalence() {
    let x = 42;
    let ref x_ref_a = x;
    let x_ref_b = &x;

    assert_eq!(*x_ref_a, *x_ref_b);

    println!("ref and & point to the same value");
    println!();
}

fn ref_destructuring() {
    // Copy and Clone are derivable because all fields are stack-allocated
    #[derive(Debug, Clone, Copy)]
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 1, y: 1 };

    let x_copy = {
        let Point {
            // destructure x as a ref
            x: ref ref_to_x,
            y: _,
        } = point;

        *ref_to_x
    };

    println!("point: {point:?}");
    println!("x_copy: {x_copy}");
    println!()
}

fn ref_mutable_destructuring() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }

    let mut point = Point { x: 1, y: 1 };

    println!("point before: {point:?}");

    let Point {
        // create a mutable reference
        x: ref mut mutable_x,
        y: _,
    } = point;

    *mutable_x *= 2;

    println!("point after: {point:?}");
    println!();
}

fn lifetime_intro() {
    let i = 42; // lifetime for i starts

    {
        let borrow_a = &i; // lifetime for borrow_a starts

        println!("borrow_a: {}", borrow_a);
    } // borrow_a lifetime ends

    {
        let borrow_b = &i; // lifetime for borrow_b starts

        println!("borrow_b: {borrow_b}");
    } // lifetime of borrow_b ends

    println!();
} // i is destroyed - outliving its references

fn lifetime_explicit() {
    fn implicit_without_return(x: &i32) {
        println!("implicit no return: x is {x}");
    }

    #[allow(clippy::needless_lifetimes)]
    fn explicit_without_return<'a>(x: &'a i32) {
        println!("explicit no return: x is {x}");
    }

    fn implicit_returned(x: &i32) -> &i32 {
        println!("implicit returned: x is {x}");
        x
    }

    #[allow(clippy::needless_lifetimes)]
    fn explicit_returned<'a>(x: &'a i32) -> &'a i32 {
        println!("explicit returned: x is {x}");
        x
    }

    fn implicit_mut_returned(x: &mut i32) -> &mut i32 {
        *x = x.pow(2);
        println!("implicit mutable returned: x is {x}");

        x
    }

    #[allow(clippy::needless_lifetimes)]
    fn explicit_mut_returned<'a>(x: &'a mut i32) -> &'a mut i32 {
        *x = x.pow(2);
        println!("explicit mutable returned: x is {x}");

        x
    }

    let [x, mut mut_x] = [42, 43];

    implicit_without_return(&x);
    explicit_without_return(&x);
    implicit_returned(&x);
    explicit_returned(&x);
    implicit_mut_returned(&mut mut_x);
    explicit_mut_returned(&mut mut_x);

    println!()
}

fn lifetime_explicit_multiple_parameters() {
    fn implicit_multiple_no_return(x: &i32, y: &i32) {
        println!("x: {x}, y: {y}")
    }

    #[allow(clippy::needless_lifetimes)]
    fn explicit_multiple_no_return<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x: {x}, y: {y}")
    }

    // this function will result in an error in compilation:
    // we're returning a value that is a reference, but:
    //  - we have 2 borrows in the function signature
    //  - therefore we have 2 different lifetimes that must outlive
    //      this function
    // so without being explicit as to which lifetime is associated with
    // the return type, the compiler would have to guess which lifetime
    // to use
    //fn implicit_multiple_with_return(x: &i32, y: &i32) -> &i32 {
    //    x + y
    //}

    // we need to be explicit about which lifetime is being returned, and
    // we may not return a value associated with another lifetime
    fn explicit_multiple_with_return<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
        println!("x: {x}, y: {y}");

        x
    }

    let (x, y) = (42, 43);

    implicit_multiple_no_return(&x, &y);
    explicit_multiple_no_return(&x, &y);
    //implicit_multiple_with_return(&x, &y);
    explicit_multiple_with_return(&x, &y);

    println!()
}

fn lifetime_explicit_static() {
    fn explicit_static_return<'a>(x: &'a i32) -> &'static str {
        println!("lifetimed x: {x}");

        "foo"
    }

    // str will live for the duration of the application, and is thus
    // implicitly 'static:
    let x: &'static str = "I am built as text into the binary";

    println!("static lifetime x: {x}");

    explicit_static_return(&42);

    println!()
}

fn lifetime_methods() {
    #[derive(Debug)]
    struct MyNewType(i32);

    impl MyNewType {
        #[allow(clippy::needless_arbitrary_self_type)]
        fn add_one<'a>(self: &'a mut Self) -> &'a mut Self {
            //let Self(x) = self;
            //*x += 1;
            self.0 += 1;

            self
        }

        #[allow(clippy::needless_arbitrary_self_type)]
        fn debug<'a>(self: &'a Self) -> &'a Self {
            println!("{self:?}");

            self
        }
    }

    let mut value = MyNewType(42);

    println!("before: {value:?}");

    value.add_one().debug();

    println!();
}

fn lifetime_struct_fields() {
    #[derive(Debug)]
    // Each instance of this struct may not outlive the value that its
    // reference is derived from
    struct BorrowedTuple<'a>(&'a String);

    #[derive(Debug)]
    #[allow(dead_code)]
    struct BorrowedNamed<'a, 'b> {
        x: &'a String,
        y: &'b String,
    }

    // one of the variants has a lifetime that the instance may not outlive
    #[derive(Debug)]
    enum Either<'a> {
        Left(String),
        Right(&'a String),
    }

    let x = String::from("foo");
    let borrowed_tuple = BorrowedTuple(&x);

    let y = "bar".to_owned();
    let borrowed_named = BorrowedNamed { x: &x, y: &y };

    let z = "right".to_owned();
    let left = Either::Left("left".to_owned());
    let right = Either::Right(&z);

    println!("borrowed_tuple: {borrowed_tuple:?}");
    println!("borrowed_named: {borrowed_named:?}");
    println!("left: {left:?}");
    println!("right: {right:?}");

    // We can't define a reference and pass it at the same time, as the
    // reference would then outlive the value.
    // The value needs to outlive the Borrowed item's lifetime, so the
    // value needs to be declared before a reference is created
    //let invalid_borrow = Borrowed(&String::from("foo"));

    //println!("invalid_borrow: {invalid_borrow:?}");

    println!()
}

fn lifetime_traits() {
    #[derive(Debug)]
    struct TupleStruct<'a>(&'a i32);

    impl<'a> Default for TupleStruct<'a> {
        fn default() -> Self {
            Self(&42)
        }
    }

    // the lifetime can be defined at the `impl` declaration...
    impl<'a> TupleStruct<'a> {
        #[allow(clippy::needless_arbitrary_self_type)]
        fn debug(self: &'a Self) -> &'a Self {
            println!("{self:?}");

            self
        }
    }

    // or at the method-level
    impl TupleStruct<'_> {
        #[allow(clippy::needless_arbitrary_self_type)]
        fn debug_again<'a>(self: &'a Self) -> &'a Self {
            println!("{self:?}");

            self
        }
    }

    TupleStruct::default().debug().debug_again();

    println!()
}

fn lifetime_bounds() {
    #[derive(Debug, Clone, Copy)]
    struct MyTupleStruct<'a>(&'a String);

    impl<'a> std::fmt::Display for MyTupleStruct<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyTupleStruct({})", self.0)
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct MyNamedStruct<'a> {
        value: &'a String,
    }

    impl<'a> std::fmt::Display for MyNamedStruct<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyNamedStruct {{ value: {} }}", self.value)
        }
    }

    // First, we define a lifetime parameter.
    // Next, we indicate that the type must implement Display.
    // Finally, we indicate that all of the type's references may not have
    // lifetimes that exceed 'a
    fn bounded_impl_lifetime<'a, T: std::fmt::Display + 'a>(x: T) {
        println!("x is {x}")
    }

    let my_string = String::from("foo");
    let tuple_struct = MyTupleStruct(&my_string);
    let named_struct = MyNamedStruct { value: &my_string };

    bounded_impl_lifetime(tuple_struct);
    bounded_impl_lifetime(named_struct);

    println!();
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

    // ref pattern
    ref_ampersand_equivalence();
    ref_destructuring();
    ref_mutable_destructuring();

    // lifetimes
    lifetime_intro();
    lifetime_explicit();
    lifetime_explicit_multiple_parameters();
    lifetime_explicit_static();
    lifetime_methods();
    lifetime_struct_fields();
    lifetime_traits();
    lifetime_bounds();
}
