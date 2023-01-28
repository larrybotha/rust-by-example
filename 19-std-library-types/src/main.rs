fn boxed_values() {
    use std::mem;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }
    }

    impl std::default::Default for Point {
        fn default() -> Self {
            Self { x: 0.0, y: 0.0 }
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    fn origin() -> Point {
        Point::default()
    }

    fn boxed_origin() -> Box<Point> {
        #[allow(clippy::box_default)]
        Box::new(Point::default())
    }

    fn debug_stack(name: &str, x: &impl std::fmt::Debug) {
        println!("{name}: {x:?}");
        println!(
            "{name} occupies {} bytes on the stack\n",
            mem::size_of_val(x)
        )
    }

    let point = origin();
    let double_boxed_point: Box<Box<Point>> = Box::new(boxed_origin());
    let rectangle = Rectangle {
        top_left: origin(),
        // we can convert i32 into f64 using .into() -
        // .into() determines the resulting type depending on context
        bottom_right: Point::new(3.into(), (-4).into()),
    };
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point::new(3.0, -4.0),
    });

    debug_stack("point", &point);
    debug_stack("double_boxed_point", &double_boxed_point);
    debug_stack("*double_boxed_point", &*double_boxed_point);
    debug_stack("**double_boxed_point", &**double_boxed_point);
    println!();

    debug_stack("rectangle", &rectangle);
    debug_stack("boxed_rectangle", &boxed_rectangle);
    debug_stack("*boxed_rectangle", &*boxed_rectangle);
    println!()
}

fn vector_element_addresses() {
    let mut xs = vec![1, 2, 3, 4];

    // capacity here is 4
    println!("xs capacity: {}", xs.capacity());
    println!("xs[0] address: {:p}", &xs[0]);

    xs.push(5);

    // capacity here is 8
    println!("xs capacity: {}", xs.capacity());
    // address of element has changed
    println!("xs[0] address: {:p}", &xs[0]);
    println!();

    println!("memory locations of each item are contiguous:");
    xs.iter().map(|x| println!("{:p}", x)).for_each(drop);

    println!()
}

fn main() {
    boxed_values();
    vector_element_addresses();
}
