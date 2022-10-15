fn tuple_structs() {
    #[derive(Debug)]
    struct MyTuple(i32, char, String);

    let tup_1 = MyTuple(42, 'h', String::from("foo"));
    println!("&tup_1: {:?}", &tup_1);
    println!("tup_1.0: {}", tup_1.0);

    let tup_2 = MyTuple {
        2: String::from("foo"),
        1: 'h',
        0: 42,
    };
    println!("&tup_2: {:?}", &tup_2);
    println!("tup_2.0: {}", tup_2.0);

    println!();
}

fn c_structs() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Thing<'a> {
        foo: String,
        bar: &'a str,
    }

    let not_foo = "foo".to_string();
    let bar = "hey";
    let thing = Thing { foo: not_foo, bar };

    println!("{:?}", &thing);
    println!();
}

fn unit_structs() {
    #[derive(Debug)]
    struct MyUnit;

    impl std::fmt::Display for MyUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "aw I'm a unit: {:?}", &self)
        }
    }

    let unit = MyUnit;
    println!("{unit}");
    println!();
}

fn struct_update_syntax() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Thing<T> {
        a: T,
        b: T,
        c: T,
    }

    let thing_1 = Thing { a: 1, b: 2, c: 3 };
    let thing_2 = Thing { b: 3, ..thing_1 };

    println!("thing 1: {:?}", &thing_1);
    println!("thing 2: {:?}", &thing_2);
    assert!(thing_1.a == thing_2.a);
    assert_eq!(thing_1.c, thing_2.c);
    println!();
}

fn struct_destructuring() {
    struct Thing<T> {
        a: T,
        b: T,
        c: T,
    }

    let thing = Thing { a: 1, b: 2, c: 3 };
    let Thing {
        a: another_a,
        b: hello,
        c,
    } = thing;

    println!("another_a: {another_a}, hello: {hello}, c: {c}");
    println!();
}

fn struct_exercise() {
    #[derive(Debug)]
    struct Point<T = f32>(T, T);

    struct Rectangle<T = f32> {
        top_left: Point<T>,
        bottom_right: Point<T>,
    }

    impl std::fmt::Debug for Rectangle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let Rectangle {
                top_left,
                bottom_right,
            } = &self;
            let Point(x1, y1) = &top_left;
            let Point(x2, y2) = &bottom_right;

            write!(
                f,
                "
              [{x1}, {y1}]
                ---------------------
                |                   |
                |                   |
                |                   |
                |                   |
                ---------------------
                                   [{x2}, {y2}]
            ",
            )
        }
    }

    impl Rectangle {
        fn area(&self) -> f32 {
            let Rectangle {
                top_left: Point(x1, y1),
                bottom_right: Point(x2, y2),
            } = &self;

            (x2 - x1) * (y2 - y1)
        }

        fn from(point: Point, scalar: f32) -> Rectangle {
            let bottom_right = Point(point.0 + scalar, point.1 + scalar);
            Rectangle {
                top_left: point,
                bottom_right,
            }
        }
    }

    let top_left = Point(1.0, 2.3);
    let rect = Rectangle {
        top_left,
        bottom_right: Point(3.0, 5.2),
    };
    let square = Rectangle::from(Point(1.0, 4.4), 3.0);

    println!("rect: {:?}", &rect);
    println!("rect area: {}", rect.area());

    println!("square: {:?}", &square);
    println!("square area: {}", square.area());
}

fn main() {
    // structs
    tuple_structs();
    c_structs();
    unit_structs();
    struct_update_syntax();
    struct_destructuring();
    struct_exercise();
}
