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
    println!();
}

fn enum_variants() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum MyEnum {
        Unit,
        Tuple(i32),
        CLike { foo: String },
    }

    let _x = MyEnum::Unit; // <= type is MyEnum

    println!("{:?}", MyEnum::Unit);
    println!("{:?}", MyEnum::Tuple(3));
    println!(
        "{:?}",
        MyEnum::CLike {
            foo: String::from("bar")
        }
    );
    println!();
}

fn enum_matching() {
    #[derive(Debug)]
    enum ScrollDir {
        Up,
        Down,
        Left,
        Right,
    }

    enum Event {
        MouseClick(i32, i32),
        KeyPress(char),
        Scroll(ScrollDir),
    }

    fn handle_event(event: Event) {
        match event {
            Event::MouseClick(x, y) => println!("clicked at ({x}, {y})"),
            Event::KeyPress(x) => println!("pressed key {x}"),
            Event::Scroll(dir) => println!("scrolled {dir:?}"),
        }
    }

    let mouse_click = Event::MouseClick(32, 4);
    let key_press = Event::KeyPress('m');
    let scroll = Event::Scroll(ScrollDir::Right);

    handle_event(mouse_click);
    handle_event(key_press);
    handle_event(scroll);
    println!();
}

fn enum_aliases() {
    #[derive(Debug)]
    enum IReallyLoveCarpeting {
        A,
        B,
        C,
    }

    // alias IReallyLoveCarpeting
    type Carpeting = IReallyLoveCarpeting;

    // variants are accessible via the alias
    let a = Carpeting::A;
    let b = Carpeting::B;
    let c = Carpeting::C;

    println!("{a:?}, {b:?}, {c:?}");
    println!();
}

fn enum_self() {
    #[derive(Debug)]
    enum Operations {
        Sum,
        Product,
    }

    impl Operations {
        fn do_operation(&self, x: i32, y: i32) -> i32 {
            match &self {
                Self::Sum => x + y,
                Self::Product => x * y,
            }
        }
    }

    let sum = Operations::Sum;
    let product = Operations::Product;
    let x = 3;
    let y = 5;

    println!("sum({x}, {y}) = {}", sum.do_operation(x, y));
    println!("product({x}, {y}) = {}", product.do_operation(x, y));
    println!();
}

#[derive(Debug)]
enum TopLevelOne {
    OneA,
    OneB,
}
#[derive(Debug)]
enum TopLevelTwo {
    TwoA,
    TwoB,
}

fn enum_use() {
    // allow variants in the outer scope to be used without manually scoping
    // each variant
    use crate::TopLevelOne::{OneA, OneB as HeyB};
    use crate::TopLevelTwo::*;

    let one_a = OneA;
    let one_b = HeyB;
    let two_a = TwoA;
    let two_b = TwoB;

    println!("one_a: {one_a:?}");
    println!("one_b: {one_b:?}");
    println!("two_a: {two_a:?}");
    println!("two_b: {two_b:?}");
    println!();
}

fn enum_discriminators() {
    enum ImplicitDiscriminator {
        First,
        Second,
    }

    enum ExplicitDiscriminator {
        First = 999,
        Second = 1_000,
    }

    println!(
        "ImplicitDiscriminator::First: {}",
        ImplicitDiscriminator::First as i32
    );
    println!(
        "ImplicitDiscriminator::Second: {}",
        ImplicitDiscriminator::Second as i32
    );
    println!(
        "ExplicitDiscriminator::First: {}",
        ExplicitDiscriminator::First as i32
    );
    println!(
        "ExplicitDiscriminator::Second: {}",
        ExplicitDiscriminator::Second as i32
    );
    println!();
}

enum List {
    // a node in a linked list containing a value, and a pointer to the next node
    Cons(u32, Box<List>),
    // the end of the linked list
    Nil,
}

impl List {
    // create an empty list
    fn new() -> List {
        Self::Nil
    }

    // prepend a value to the current list and return it
    fn prepend(self, elem: u32) -> List {
        Self::Cons(elem, Box::new(self))
    }

    // get the length of the list
    fn len(&self) -> u32 {
        // `self` has type &List
        // `*self` has type List
        //      => matching on a concrete type is described as being better....
        //          but not yet sure why that is
        match *self {
            // not yet sure what `ref` is doing here - &tail doesn't compile
            Self::Cons(_, ref tail) => 1 + tail.len(),
            Self::Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        // again, matching on concrete type rather than a reference
        match *self {
            Self::Cons(head, ref tail) => format!("{} {}", head, tail.stringify()),
            Self::Nil => "Nil".to_string(),
        }
    }
}

fn enum_linked_list() {
    let mut list = List::new();

    list = list.prepend(3);
    list = list.prepend(4);
    list = list.prepend(5);

    println!("list: {}", &list.stringify());
    println!("list length: {}", &list.len());
    println!();
}

const I_AM_GLOBAL: &str = "I am global!";

fn const_definitions() {
    const I_AM_LOCAL: &str = "I am local!";

    println!("{}", I_AM_GLOBAL);
    println!("{}", I_AM_LOCAL);
}

fn main() {
    // structs
    tuple_structs();
    c_structs();
    unit_structs();
    struct_update_syntax();
    struct_destructuring();
    struct_exercise();

    // enums
    enum_variants();
    enum_matching();
    enum_aliases();
    enum_self();
    enum_use();
    enum_discriminators();
    enum_linked_list();

    // constants
    const_definitions();
}
