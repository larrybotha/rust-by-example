use std::convert::{From, TryFrom};

fn from_example() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Foo {
        bar: i32,
    }

    impl From<i32> for Foo {
        fn from(value: i32) -> Self {
            Foo { bar: value }
        }
    }

    impl From<f32> for Foo {
        fn from(value: f32) -> Self {
            Foo { bar: value as i32 }
        }
    }

    impl From<&str> for Foo {
        fn from(value: &str) -> Self {
            Foo {
                bar: value.parse().unwrap_or(0),
            }
        }
    }

    let x = 6;
    let y = 6.2;
    let foo_from_x = Foo::from(x);
    let foo_from_y = Foo::from(y);
    let foo_from_z = Foo::from("10");
    let foo_from_invalid = Foo::from("10a");

    println!("foo_from_x: {foo_from_x:?}");
    println!("foo_from_y: {foo_from_y:?}");
    println!("foo_from_z: {foo_from_z:?}");
    println!("foo_from_invalid: {foo_from_invalid:?}");
    println!();
}

fn into_example() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Foo {
        value: i32,
    }

    impl From<i32> for Foo {
        fn from(value: i32) -> Self {
            Foo { value }
        }
    }

    impl From<&str> for Foo {
        fn from(value: &str) -> Self {
            Foo {
                value: value.parse().unwrap_or(0),
            }
        }
    }

    let x: Foo = 6.into();
    let y: Foo = "42".into();

    println!("x: {x:?}");
    println!("y: {y:?}");
    println!();
}

fn try_from_try_into() {
    #[derive(Debug, PartialEq)]
    #[allow(dead_code)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            match value % 2 {
                0 => Ok(EvenNumber(value)),
                _ => Err(()),
            }
        }
    }

    let even_from: Result<EvenNumber, ()> = EvenNumber::try_from(2);
    let odd_from: Result<EvenNumber, ()> = EvenNumber::try_from(1);
    let even_into: Result<EvenNumber, ()> = 42i32.try_into();
    let odd_into: Result<EvenNumber, ()> = 41i32.try_into();

    assert_eq!(even_from, Ok(EvenNumber(2)));
    assert_eq!(odd_from, Err(()));
    assert_eq!(even_into, Ok(EvenNumber(42)));
    assert_eq!(odd_into, Err(()));

    println!("even_from: {even_from:?}");
    println!("odd_from: {odd_from:?}");
    println!("even_into: {even_into:?}");
    println!("odd_into: {odd_into:?}");
    println!();
}

fn try_from_try_into_again() {
    #[derive(Debug, PartialEq)]
    struct LongString(String);

    impl std::convert::TryFrom<&str> for LongString {
        type Error = bool;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value.len() >= 5 {
                true => Ok(LongString(value.to_string())),
                _ => Err(false),
            }
        }
    }

    let short: Result<LongString, bool> = "nope".try_into();
    let x = "hell yes!";
    let long: Result<LongString, bool> = LongString::try_from(x);

    assert_eq!(Err(false), short);
    assert_eq!(Ok(LongString(x.into())), long);

    println!("short: {short:?}");
    println!("long: {long:?}");
    println!();
}

fn display_and_to_string() {
    struct Point(i32, i32);

    impl std::fmt::Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", &self.0, &self.1)
        }
    }

    let point = Point(1, 2);
    let point_string = point.to_string();

    println!("the point is at {point_string}");
    println!();
}

fn turbo_fish_parsing() {
    let string_int = "42";
    let string_float = "42.6";
    let x_i32 = string_int.parse::<i32>().unwrap();
    let x_f32 = string_float.parse::<f32>().unwrap();

    println!("x_i32: {x_i32}");
    println!("x_f32: {x_f32}");
    println!();
}

fn to_string_custom_types() {
    #[derive(Debug)]
    struct Total(i32);

    impl std::str::FromStr for Total {
        type Err = ();

        fn from_str(value: &str) -> Result<Total, Self::Err> {
            let result = value
                .split(' ')
                .map(|x| x.parse().unwrap_or(0))
                // fold == sum
                //.fold(0, |x, acc| x + acc);
                .sum();

            Ok(Total(result))
        }
    }

    let sum = "10 4 5".parse::<Total>();
    println!("sum: {sum:?}");
    println!();
}

fn main() {
    from_example();
    into_example();

    try_from_try_into();
    try_from_try_into_again();

    display_and_to_string();

    turbo_fish_parsing();
    to_string_custom_types();
}
