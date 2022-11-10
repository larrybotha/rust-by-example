use std::any::type_name;
use std::mem;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn associated_functions_and_methods() {
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // associated function
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }
    }

    #[derive(Debug)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // method
        fn area(&self) -> f64 {
            let Self { p1, p2 } = self;
            let Point { x: x1, y: y1 } = p1;
            let Point { x: x2, y: y2 } = p2;

            ((x2 - x1) * (y2 - y1)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Self { p1, p2 } = self;
            let Point { x: x1, y: y1 } = p1;
            let Point { x: x2, y: y2 } = p2;
            let length = (x2 - x1).abs();
            let breadth = (y2 - y1).abs();

            (length + breadth) * 2.0
        }

        // mutates he instance, so we need to indicate that the instance must
        // be defined as mutable
        fn translate(&mut self, d_x: f64, d_y: f64) -> &Self {
            let Self {
                p1: top_left,
                p2: bottom_right,
            } = self;
            let Point { x: x1, y: y1 } = top_left;
            let Point { x: x2, y: y2 } = bottom_right;

            *x1 += d_x;
            *x2 += d_x;
            *y1 += d_y;
            *y2 += d_y;

            self
        }

        fn from(rectangle: &Self) -> Self {
            let Rectangle { p1, p2 } = rectangle;
            let p1 = Point { ..*p1 };
            let p2 = Point { ..*p2 };

            Rectangle { p1, p2 }
        }
    }

    let rect_1 = Rectangle {
        p1: Point::origin(),
        p2: Point::new(2.0, 3.0),
    };
    let mut rect_2 = Rectangle::from(&rect_1);

    println!("rect_1: {:#?}", rect_1);
    println!("rect_1 area: {:?}", rect_1.area());
    println!("rect_1 perimeter: {:?}", rect_1.perimeter());
    println!();
    println!("rect_2: {:#?}", rect_2);
    println!(
        "rect_2 translated by (1,5): {:#?}",
        rect_2.translate(1.0, 5.0)
    );
    println!();
}

fn consumption_as_destruction() {
    #[derive(Debug)]
    struct HeapInts(Box<i32>, Box<i32>);

    impl HeapInts {
        fn destroy(self) {
            let HeapInts(x, y) = self;

            println!("consuming {} and {}", x, y);
            println!("values are now no longer valid");
        }
    }

    let x = Box::new(1);
    let y = Box::new(2);
    let heap_ints = HeapInts(x, y);

    println!("once: {heap_ints:?}");
    println!("twice: {heap_ints:?}");

    heap_ints.destroy();

    //println!("thrice: {heap_ints:?}");
    println!("heap_ints is invalid after destroy!");
    println!();
}

fn closure_optional_type_annotation() {
    let closure_inferred = |x| x * 2;
    let closure_annotated = |x: u32| -> u32 { x * 2 };

    println!(
        "closure_inferred will be i32 from now on: {}",
        type_of(closure_inferred(5))
    );
    println!(
        "closure_annotated is always u32: {}",
        type_of(closure_annotated(5))
    );
}

fn closure_capture_by_reference() {
    let x = String::from("foo");
    let bar = || println!("x captured by reference: {x}");

    bar();
    bar();

    println!("we still have access to x: {x}");
    println!();
}

fn closure_capture_by_mutable_ref() {
    let mut x = String::from("foo");
    let mut bar = || {
        x += "o";
        println!("x is now: {x}")
    };

    bar();
    bar();
    println!("x mutably referenced: {x}");
    println!();
}

fn closure_mut_ref_borrowing() {
    let mut x = 5;
    let mut inc_x = || x += 1; // x is  borrered mutable here

    inc_x();

    let my_ref = &x;

    println!(
        "my_ref can reference x _after_ inc_x is called, once it no longer borrowed: {my_ref}"
    );

    // not allowed - there is an existing referring to the value inc_x references
    //inc_x();

    println!();
}

fn closure_capture_by_value() {
    let x = Box::new(5); // heap-allocated value - is not Copy
    let drop_x = || {
        println!("dropping x from the heap");
        // x is moved here, before the function is even called;
        // it may not be referenced anyqhere after this definition
        mem::drop(x);
    };

    drop_x();

    println!("x is no longer valid");

    // may not be executed again - x is now invalid
    //drop_x();

    println!();
}

fn closure_move() {
    let x = Box::new(5);
    let move_x = move || println!("x is moved: {x}");

    move_x();

    println!("x is no longer valid");
    println!();

    let xs = vec![1, 2, 3];
    let contains = |needle| xs.contains(needle);

    println!("xs contains 2: {}", contains(&2));
    println!("xs contains 4: {}", contains(&4));
    println!();
}

fn closure_as_input_fn() {
    fn do_that_than<F>(func: F) -> i32
    where
        // parameter is
        F: Fn(i32) -> i32,
    {
        let x = 5;

        println!("x has type: {}", type_of(x));

        func(x)
    }

    let my_closure = |x| {
        println!("x has type: {}", type_of(x));
        x * x
    };

    println!("my_closure: {}", do_that_than(my_closure));
    println!();
}

fn closure_as_input_value_by_reference() {
    fn apply<F>(f: F)
    where
        F: Fn(),
    {
        f()
    }

    let x = "foo";
    let y = 4;
    let my_func = || {
        println!("x (reference): {}", x);
        println!("y (value): {}", y);
        println!("x captured by reference");
        println!("requires Fn, or FnOnce");
    };

    apply(my_func);
    println!();
}

fn closure_as_input_value_by_mutable_reference() {
    fn apply<F>(mut f: F)
    where
        F: FnMut(),
    {
        f()
    }

    let mut x = Box::new(5);
    let my_func = || {
        *x += 1;
        println!("x captured as mutable reference");
        println!("requires FnMut, or FnOnce");
    };

    apply(my_func);
    println!();
}

fn closure_as_input_value_by_value() {
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f()
    }

    let mut x = Box::new(4);
    let my_drop_func = || {
        *x += 1;
        println!("x: {x}");
        println!("x captured by value because of mem::drop");
        println!("requires FnOnce");
        mem::drop(x);
        println!("x no longer valid after drop");
    };

    apply(my_drop_func);
    println!();

    let x = Box::new(5);
    let my_move_func = move || {
        let z = x;
        println!("z: {z}");
        println!("x captured by value because of 'move'");
        println!("requires FnOnce");
        println!("x no longer valid after drop");
    };

    apply(my_move_func);
    println!();
}

fn closure_as_output() {
    fn output_as_fn() -> impl Fn() {
        let x = String::from("bar");
        let y = String::from("foo");

        move || println!("called! Ref: {}, Owned: {}", &x, y)
    }

    fn output_as_fn_mut() -> impl FnMut() {
        let mut x = String::from("foo");

        move || {
            println!("x before: {x}");
            x.push_str(" bar");
            println!("x mutated: {}", x)
        }
    }

    fn output_as_fn_once() -> impl FnOnce() {
        let x = "foo".to_owned();

        move || {
            println!("x before: {}", x);
            println!("dropping x from within closure...");
            mem::drop(x);
            println!("x has been dropped");
        }
    }

    let as_fn = output_as_fn();
    let mut as_fn_mut = output_as_fn_mut();
    let as_fn_once = output_as_fn_once();

    as_fn();
    println!();

    as_fn_mut();
    println!();

    as_fn_once();
    println!();
}

fn closure_any() {
    let xs = vec![1, 2, 3];
    let result = xs.iter().any(|&x| x > 2);

    println!("any of vec xs > 2?: {result}");

    let result = xs.into_iter().any(|x| x > 2);
    println!("any of vec xs > 2?: {result}");
    println!("xs has been dropped thanks to .into_iter");
    println!();

    let xs = [true, false, true];
    let result = xs.iter().any(|&x| x);

    println!("any of array xs == true?: {result}");

    let result = xs.into_iter().any(|x| x);

    println!("any of array xs == true?: {result}");
    println!("xs is no longer valid");
    println!();
}

fn closure_find() {
    let xs = vec![1, 2, 3];
    // .iter() on vectors iterates over &T - a reference. To reference a
    // reference, we need to destructure with a double ampersand
    let first_even = xs.iter().find(|&&x| x == 2);

    if let Some(n) = first_even {
        println!("first even value in 'xs' is {}", n);
        println!("type of first_even is {}", type_of(n));
    } else {
        println!("no even numbers in 'xs'");
    }

    // .into_iter() on vectors iterators over values by value
    let first_even = xs.into_iter().find(|&x| x == 2);

    if let Some(n) = first_even {
        println!("first even value in 'xs' is {}", n);
        println!("type of first_even is {}", type_of(n));
    }

    println!();
}

fn closure_find_is_filter_next() {
    let xs = vec![1, 2, 3];
    #[allow(clippy::filter_next)]
    let result = xs.iter().filter(|&&x| x % 2 == 0).next();

    if let Some(x) = result {
        println!("first even value of 'xs' is {x}");
    }

    println!();
}

fn closure_position() {
    let xs = vec!["bar", "foo"];
    let needle = "foo";
    // we destructure `x` here so that the type of x matches how we're using it
    let index = xs.iter().position(|&x| x == needle);

    if let Some(i) = index {
        println!("'{needle}' in 'xs' is at index {}", i);
    }

    let xs = vec!["bar".to_string(), "foo".to_string()];
    let needle = "foo";
    // 'x' does not need to be destructured with an ampersand here - its type
    // is &String, so it is already a reference
    let index = xs.iter().position(|x| x == needle);

    if let Some(i) = index {
        println!("'{needle}' in 'xs' is at index {}", i);
    }

    let xs = vec![1, 2, 3, 4, 5];
    let needle = 5;
    // in this example,
    let index = xs.iter().position(|&x| x == needle);

    if let Some(i) = index {
        println!("'{needle}' in 'xs' is at index {}", i);
    }

    println!();
}

fn higher_order_functions() {
    fn square(x: i32) -> i32 {
        x * x
    }

    fn is_lte(upper: i32) -> impl FnMut(&i32) -> bool {
        move |x| x <= &upper
    }

    let upper = 10;
    let xs: Vec<i32> = (0..)
        .into_iter()
        .map(square)
        // the predicate must implement FnMut
        .take_while(is_lte(upper))
        .collect();
    let sum: i32 = xs.iter().sum();

    println!("xs: {xs:?} => {sum}");
    println!();
}

fn main() {
    associated_functions_and_methods();
    consumption_as_destruction();

    closure_optional_type_annotation();
    closure_capture_by_reference();
    closure_capture_by_mutable_ref();
    closure_mut_ref_borrowing();
    closure_capture_by_value();
    closure_move();

    closure_as_input_fn();
    closure_as_input_value_by_reference();
    closure_as_input_value_by_mutable_reference();
    closure_as_input_value_by_value();

    closure_as_output();

    closure_any();
    closure_find();
    closure_find_is_filter_next();
    closure_position();

    higher_order_functions();
}
