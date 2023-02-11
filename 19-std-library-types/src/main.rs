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

fn vector_from_iterator() {
    let iter = 0..10;
    let xs: Vec<i32> = iter.collect();

    println!("xs: {:?}", xs);
    println!()
}

fn vector_push_pop() {
    let mut xs = vec![1, 2, 3];

    println!("xs before: {:?}", xs);

    xs.push(4);

    println!("xs after push: {:?}", xs);

    let x = xs.pop();

    println!("xs after pop: {:?}", xs);
    println!("x from pop: {:?}", x);
    println!()
}

fn vector_pop_empty() {
    let mut xs: Vec<i32> = Vec::new();
    let x = xs.pop();
    let y = Vec::pop(&mut xs); // popping via struct definition

    assert_eq!(x, None);
    println!("x from empty vec with length {}: {:?}", xs.len(), x);
    println!("y from empty vec with length {}: {:?}", xs.len(), y);
    println!()
}

fn vector_out_of_bounds_panics() {
    use std::panic;

    let xs: Vec<i32> = vec![];
    let panic_result = panic::catch_unwind(|| xs[0]);

    println!("xs out of bounds panid result: {:?}", panic_result);
    println!()
}

fn vector_for_iteration() {
    let xs = vec![1, 2, 3];

    for x in xs.iter() {
        println!("x: {}", x);
    }
    println!()
}

fn vector_for_enumeration() {
    let xs = (100..103).collect::<Vec<i32>>();

    for (i, x) in xs.iter().enumerate() {
        println!("x in xs at position {}: {}", i, x);
    }

    println!()
}

fn vector_map_enumeration() {
    let xs = (0..10).collect::<Vec<i32>>();
    let ys = xs
        .iter()
        .enumerate()
        .map(|(i, x)| (i, *x * (i as i32)))
        .collect::<Vec<(usize, i32)>>();

    ys.iter()
        .map(|(i, x)| println!("(i, x): ({}, {})", i, x))
        .for_each(drop);
    println!()
}

fn vector_mutable_iteration() {
    fn print_vec(xs: &[i32]) {
        xs.iter()
            .enumerate()
            .map(|(i, x)| println!("xs at {}: {}", i, x))
            .for_each(drop);
    }

    let mut xs = vec![1, 2, 3];

    println!("mutably iterate without .iter_mut:");
    xs = xs.iter().map(|x| i32::pow(*x, 2)).collect::<Vec<i32>>();

    print_vec(&xs);

    println!("\nmutably iterate with for and .iter_mut:");

    for x in xs.iter_mut() {
        *x = i32::pow(*x, 2);
    }

    print_vec(&xs);

    println!();
}

fn string_slice_explicit_type() {
    let x: &'static str = "hello, world!";

    println!("x is a reference to a string with a static lifetime:\n{x}");
    println!("x's pointer to its memory location: {:?}", x.as_ptr());
    println!("x as bytes: {:?}", x.as_bytes());
    println!()
}

fn string_split() {
    fn reverse_print(xs: &str) {
        xs.split_whitespace()
            .rev()
            .enumerate()
            .map(|(index, word)| println!("index: {index}, word: {word}"))
            .for_each(drop);

        println!()
    }

    let string_slice = "The quick brown fox";
    let string = String::from(string_slice);

    reverse_print(string_slice);
    reverse_print(&string);
}

fn string_chars() {
    let sentence = "foo bar";
    let chars_iterator = sentence.chars();
    let mut chars_vec: Vec<char> = chars_iterator.collect();

    println!("chars_vec: {chars_vec:?}");

    chars_vec.sort();

    println!("chars_vec sorted: {chars_vec:?}");

    chars_vec.dedup();

    println!("chars_vec deduped: {chars_vec:?}");

    println!()
}

fn string_is_growable() {
    let original_sentence = "foo bar";
    let mut chars: Vec<char> = original_sentence.chars().collect();

    chars.sort();
    chars.dedup();

    let mut growable_string = String::new();

    for c in chars {
        growable_string.push(c);
    }

    let chars_to_trim: &[char] = &[' '];
    let slice = growable_string.trim_matches(chars_to_trim);

    println!("slice: {slice:?}");
    println!()
}

fn string_replace() {
    let x = "I like dogs";
    let y = x.replace("dog", "cat");

    println!("x: {x}");
    println!("y: {y}");
    println!()
}

fn string_escaping() {
    let escaped = "\"Hey,\" he said";

    println!("escaped: {escaped}");
    println!()
}

fn option_example() {
    fn checked_division(numerator: f64, divisor: f64) -> Option<f64> {
        if divisor == 0.into() {
            None
        } else {
            Some(numerator / divisor)
        }
    }

    fn try_divide(numerator: f64, divisor: f64) {
        match checked_division(numerator, divisor) {
            None => println!("failed: attempted to divide by zero"),
            Some(n) => println!("succeeded: {numerator} / {divisor} = {n}"),
        }
    }

    try_divide(1.0, 0.0);
    try_divide(1.0, 2.0);

    println!()
}

fn result_example() {
    mod checked {
        #[derive(Debug)]
        pub enum MathError {
            DivisionByZero,
            NegativeSquareRoot,
            NonPositiveLogarithm,
        }

        pub type MathResult = Result<f64, MathError>;

        pub fn div(x: f64, y: f64) -> MathResult {
            if y == 0.0 {
                Err(MathError::DivisionByZero)
            } else {
                Ok(x / y)
            }
        }

        pub fn sqrt(x: f64) -> MathResult {
            if x < 0.0 {
                Err(MathError::NegativeSquareRoot)
            } else {
                Ok(x.sqrt())
            }
        }

        pub fn ln(x: f64) -> MathResult {
            if x <= 0.0 {
                Err(MathError::NonPositiveLogarithm)
            } else {
                Ok(x.ln())
            }
        }
    }

    let x = 2.0;
    let y = 4.0;

    println!("div(x, y) = {:?}", checked::div(x, y));
    println!("div(x, 0.0) = {:?}", checked::div(x, 0.0));
    println!("sqrt(x) = {:?}", checked::sqrt(x));
    println!("sqrt(-1.0) = {:?}", checked::sqrt(-1.0));
    println!("ln(x) = {:?}", checked::ln(x));
    println!("ln(0.0) = {:?}", checked::ln(0.0));
    println!()
}

fn result_question_mark() {
    #[derive(Debug)]
    struct MyError;

    type MyResult = Result<i32, MyError>;

    fn try_double(x: MyResult) -> MyResult {
        // unwrap the value using ?
        let value = x?;

        Ok(value * 2)
    }

    let x: MyResult = Ok(2);

    println!("x: {x:?}");
    println!("x doubled: {:?}", try_double(x));
    println!("error doubled: {:?}", try_double(Err(MyError)));
    println!()
}

fn hashmap_capacity() {
    use std::collections::HashMap;

    let hash_map_from_new: HashMap<String, i32> = HashMap::new();
    let hash_map_from_cap: HashMap<String, i32> = HashMap::with_capacity(6);

    println!("hash_map_from_new: {:?}", hash_map_from_new);
    println!(
        "hash_map_from_new capacity: {}",
        hash_map_from_new.capacity()
    );
    println!("hash_map_from_new length: {}\n", hash_map_from_new.len());

    println!("hash_map_from_cap: {:?}", hash_map_from_cap);
    println!(
        "hash_map_from_cap capacity: {}",
        hash_map_from_cap.capacity()
    );
    println!("hash_map_from_cap length: {}\n", hash_map_from_cap.len());
}

fn hashmap_interactions() {
    use std::collections::HashMap;

    let mut hm = HashMap::new();

    hm.insert("foo", "bar");
    hm.insert("baz", "quux");

    match hm.get("foo") {
        Some(v) => println!("got {v} at key 'foo'"),
        None => println!("nothing at key 'foo'"),
    }

    match hm.get("boo") {
        Some(v) => println!("got {v} at key 'boo'"),
        None => println!("nothing at key 'boo'"),
    }

    let remove_key = "foo";
    // removing a value returns an Option
    let removed_value = hm.remove(&remove_key);

    assert_eq!(removed_value, Some("bar"));
    println!("removed value at key {remove_key}: {removed_value:?}");

    let removed_value = hm.remove_entry(&remove_key);

    assert_eq!(removed_value, None);
    println!("removed value at key {remove_key} again: {removed_value:?}\n");

    // iterating
    hm.iter()
        .map(|(key, value)| println!("key: {key}, value: {value}"))
        .for_each(drop);

    println!()
}

fn hash_map_string_str() {
    use std::collections::HashMap;

    let mut hash_map = HashMap::new();

    // build the HashMap with as HashMap<String, usize>
    vec!["a", "b", "c", "d", "e"]
        .iter()
        .enumerate()
        .map(|(index, &value)| hash_map.insert(value.to_string(), index))
        .for_each(drop);

    // We can access items in the hashmap using &str - no need to use String!
    let maybe_a = hash_map.get("a");

    assert_eq!(maybe_a, Some(&0_usize));
    println!("the value at 'a' is: {maybe_a:?}");

    println!()
}

fn hash_primitive_keys() {
    use std::collections::HashMap;

    let mut bool_map = HashMap::new();
    let mut int_map = HashMap::new();
    let mut str_map = HashMap::new();
    let mut string_map = HashMap::new();

    vec!["a", "b", "c"]
        .iter()
        .enumerate()
        // destructure &&str to &str here
        .map(|(index, &value)| {
            bool_map.insert(index % 2 != 0, index % 2);
            int_map.insert(index, index + 1);
            str_map.insert(value, index);
            string_map.insert(value.to_string(), index + 1);
        })
        .for_each(drop);

    println!("bool_map: {bool_map:#?}");
    println!("int_map: {int_map:#?}");
    println!("str_map: {str_map:#?}");
    println!("string_map: {string_map:#?}");

    println!()
}

fn hash_collection() {
    use std::collections::HashMap;

    type IntVec = Vec<i32>;
    // IntVec can be a key, because i32 implements Hash and Eq
    type IntVecHashMap = HashMap<IntVec, i32>;

    let mut hash_map: IntVecHashMap = HashMap::new();
    let xs: IntVec = (1..=3).into_iter().collect();
    let sum = xs.iter().sum();

    hash_map.insert(xs, sum);

    println!("hash_map: {hash_map:?}");
    println!()
}

fn hash_custom_types() {
    use std::collections::HashMap;
    use std::hash::Hash;

    #[derive(Debug, PartialEq, Eq, Hash)]
    enum Animal {
        Dog,
        Cat,
    }

    #[derive(Debug)]
    enum PetFood {
        DogFood,
        CatFood,
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Pet {
        animal: Animal,
        name: String,
    }

    // Pet, and Pet::animal are hashable, and so can be used as a key
    // in a HashMap
    type PetHashMap = HashMap<Pet, PetFood>;

    fn get_pet<'a, 'b>(
        pet_map: &'a PetHashMap,
        name: &'b str,
        animal: Animal,
    ) -> Result<&'a Pet, String> {
        let search_pet = Pet {
            name: name.to_owned(),
            animal,
        };

        match pet_map.get_key_value(&search_pet) {
            None => Err("Pet not found!".to_owned()),
            Some((pet, _)) => Ok(pet),
        }
    }

    let kitty = Pet {
        name: "kitty".to_owned(),
        animal: Animal::Cat,
    };
    let doggy = Pet {
        name: String::from("doggy"),
        animal: Animal::Dog,
    };
    let pets = [kitty, doggy];
    let mut pet_map: PetHashMap = HashMap::new();

    pets.into_iter()
        .map(|x| match x.animal {
            Animal::Cat => pet_map.insert(x, PetFood::CatFood),
            Animal::Dog => pet_map.insert(x, PetFood::DogFood),
        })
        .for_each(drop);

    println!("pet_map: {pet_map:#?}\n");

    let pet_search_result = get_pet(&pet_map, "kitty", Animal::Dog);

    println!("invalid pet search result: {pet_search_result:#?}");

    let pet_search_result = get_pet(&pet_map, "kitty", Animal::Cat);
    println!("valid pet search result: {pet_search_result:#?}");
    println!()
}

fn main() {
    boxed_values();

    // vectors
    vector_element_addresses();
    vector_from_iterator();
    vector_push_pop();
    vector_pop_empty();
    vector_out_of_bounds_panics();
    vector_for_iteration();
    vector_for_enumeration();
    vector_map_enumeration();
    vector_mutable_iteration();

    // strings
    string_slice_explicit_type();
    string_split();
    string_chars();
    string_is_growable();
    string_replace();
    string_escaping();

    // option
    option_example();

    // result
    result_example();
    result_question_mark();

    // hashmap
    hashmap_capacity();
    hashmap_interactions();
    hash_map_string_str();
    hash_primitive_keys();
    hash_collection();
    hash_custom_types();
}
