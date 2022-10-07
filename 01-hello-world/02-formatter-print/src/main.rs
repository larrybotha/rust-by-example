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
}

fn main() {
    format_example();
    eprint_example();
    indentation();
    format_characters();
    impl_display_implements_to_string();
    display_vs_debug();
}
