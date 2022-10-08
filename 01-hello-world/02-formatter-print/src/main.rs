use std::fmt;

// allow for dead code to compile
#[allow(dead_code)]
struct UnusedStruct {}

fn format_example() {
    let a = format!("A dynamic value: {}", "foo");
    println!("{}", a);

    let a = format!("keyword arg: {value}", value = "foo");
    println!("{}", a);

    let value = "foo";
    let a = format!("named arg: {value}");
    println!("{}", a);

    let a = format!("positioned args: {1} {0}", "first", "second");
    println!("{}", a);

    println!();
}

fn eprint_example() {
    let a = "foo";
    eprintln!("{}", a);
}

fn indentation() {
    let a = "foo";
    let x = 42;
    let width = 10;

    println!("occupy {width} using spaces right: |{:>width$}|", a);
    println!("occupy {width} using spaces left: |{:<width$}|", a);
    println!("occupy {width} using spaces centered: |{:^width$}|", a);
    println!("occupy {width} using - centered: |{:-^width$}|", a);
    println!(
        "pad with leading zeros until width of {width}: |{:0width$}|",
        x
    );
    println!(
        "pad with leading zeros until width of {width}: |{:0width$}|",
        -x
    );
    println!("decimal precision of {width}: |{:.width$}|", 5.0);
    println!();
}

fn format_characters() {
    #[derive(Debug)]
    struct Foo {
        bar: String,
    }

    impl fmt::Display for Foo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Here we have a Foo: {{ bar: {} }}", &self.bar)
        }
    }

    let a = Foo {
        bar: "baz".to_string(),
    };
    let x = 42;

    println!("Display trait         {{}}: {}", a);
    println!("Debug trait           {{:?}}: {:?}", a);
    println!("binary {0}            {{:b}}: {0:b}", x);
    println!("octal {0}             {{:o}}: {0:o}", x);
    println!("hexadecimal lower {0} {{:x}}: {0:x}", x);
    println!("hexadecimal upper {0} {{:X}}: {0:X}", x);
    println!();

    println!("alternate Display trait        {{:#}}: {:#}", a);
    println!("alternate Debug trait          {{:#?}}: {:#?}", a);
    println!("alternate binary {0}            {{:#b}}: {0:#b}", x);
    println!("alternate octal {0}             {{:#o}}: {0:#o}", x);
    println!("alternate hexadecimal lower {0} {{:#x}}: {0:#x}", x);
    println!("alternate hexadecimal upper {0} {{:#X}}: {0:#X}", x);
}

fn impl_display_implements_to_string() {
    struct Foo {
        bar: String,
    }

    impl fmt::Display for Foo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Here we have a Foo: {{ bar: {} }}", &self.bar)
        }
    }

    let a = Foo {
        bar: "baz".to_string(),
    };
    // we didn't define .to_stirng() on Foo - it's automatially implemented
    // when we implement Display
    let foo_string = a.to_string();
    println!("Foo.to_string: {}", foo_string);
    println!();
}

fn display_vs_debug() {
    #[derive(Debug)]
    struct MinMax(i8, i8);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", &self.0, &self.1)
        }
    }

    impl fmt::Binary for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({:b}, {:b})", &self.0, &self.1)
        }
    }

    let min_max = MinMax(-5, 10);

    println!("Debug min_max: {min_max:?}");
    println!("Pretty-print min_max: {min_max:#?}");
    println!("Display min_max: {min_max}");
    println!("alternate Display min_max: {min_max:#}");
    println!("Binary min_max: {min_max:b}");
    println!();
}

fn display_for_list() {
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Get the vector out of the type
            // List is a tuple struct, so we use the index of the tuple to
            // extract the value
            let vec = &self.0;

            // write the open parens to the formatter, using ? to handle
            // the Result
            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                //                  [1]     [2]
                // 1 - get an iterator from vec. This is an explicit analogue to
                //      Python's __iter__ methods on objects
                // 2 - as in Python, we can use enumerate to generate a tuple of
                //      the index and value for iteration

                // if we are beyond the first value, write a comma to the
                // formatter
                if count > 0 {
                    write!(f, ", ")?;
                }

                // write the value to the formatter
                write!(f, "{}: {}", count, v)?;
            }

            // append a closing parens
            // Note that at this point we are returning write! - a Result
            // We don't want to handle the Result using ? - the type signature
            // expects that we return a result
            write!(f, "]")
        }
    }

    let array = [1, 2, 3];
    let vec = array.to_vec();
    let list = List(vec);

    println!("{}", list);
    println!();
}

fn another_display() {
    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Rust's ternary is easier to read than Python's:
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(
                f,
                "{name}: {lat:.4}°{lat_c} {lon:.4}°{lon_c}",
                name = self.name,
                lat = &self.lat,
                lon = &self.lon
            )
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl fmt::LowerHex for Color {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // pad each value on the left with 0s to a width of 2,
            // converting to hexadecimal
            let hex_red = format!("{:0>2x}", &self.red);
            let hex_green = format!("{:0>2x}", &self.green);
            let hex_blue = format!("{:0>2x}", &self.blue);
            let hex = hex_red + &hex_green + &hex_blue;

            write!(f, "{:?} 0x{}", self, hex)
        }
    }

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", city);
    }
    println!();

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", *color);
        println!("{:x}", *color);
    }
}

fn main() {
    format_example();
    eprint_example();
    indentation();
    format_characters();
    impl_display_implements_to_string();
    display_vs_debug();
    display_for_list();
    another_display();
}
