fn associated_functions_and_methods() {
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // associated function
        fn origin() -> Point {
            let point = Point { x: 0.0, y: 0.0 };

            point
        }

        fn new(x: f64, y: f64) -> Point {
            let point = Point { x, y };

            point
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

fn main() {
    associated_functions_and_methods();
    consumption_as_destruction();
}
