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

fn main() {
    boxed_values();
}
