mod my_mod {
    pub fn public_function() {
        println!("called my_mod::public_function")
    }

    fn private_function() {
        println!("called my_mod::private_function")
    }

    pub fn indirect_access() {
        println!("called my_mod::indirect_access");
        println!("calling my_mod::private_function...");
        private_function()
    }

    pub fn nested_a_parent_access() {
        println!("called my_mod::nested_a_parent_access");
        println!("calling my_mod::nested_a::nested_available_in_parent");
        nested_a::nested_available_in_parent();
        println!("called my_mod::nested_a::nested_available_in_parent");
    }

    pub fn nested_a_crate_access() {
        println!("called my_mod::nested_a_parent_access");
        println!("calling my_mod::nested_a::nested_available_in_crate");
        nested_a::nested_available_in_crate();
        println!("called my_mod::nested_a::nested_available_in_crate");
    }

    pub mod nested_a {
        // This function is only accessible inside my_mod::nested_a.
        // The pub(self) syntax is equivalent to not using pub at all
        #[allow(dead_code)]
        pub(self) fn nested_private_explicit() {
            println!("called my_mod::nested_a::nested_private_explicit")
        }

        pub fn nested_public_function() {
            println!("called my_mod::nested_a::public_function")
        }

        pub fn nested_indirect_access() {
            println!("called my_mod::nested::nested_indirect_access");
            println!("calling my_mod::nested_a::nested_private_explicit...");
            nested_private_explicit()
        }

        pub(in crate::my_mod) fn nested_available_in_crate() {
            println!("called my_mod::nested_a::nested_available_in_crate")
        }

        pub(super) fn nested_available_in_parent() {
            println!("called my_mod::nested_a::nested_available_in_crate")
        }
    }

    pub mod nested_b {
        pub fn nested_a_crate_access() {
            println!("called my_mod::nested_b::nested_a_parent_access");
            println!("calling crate::my_mod::nested_a::nested_available_in_crate");
            crate::my_mod::nested_a::nested_available_in_crate();
            println!("called crate::my_mod::nested_a::nested_available_in_crate");
        }
    }
}

fn visibility() {
    my_mod::public_function();
    println!();

    my_mod::indirect_access();
    println!();

    my_mod::nested_a::nested_public_function();
    println!();

    my_mod::nested_a::nested_indirect_access();
    println!();

    my_mod::nested_a_parent_access();
    println!();

    my_mod::nested_a_crate_access();
    println!();

    my_mod::nested_b::nested_a_crate_access();
    println!();
}

fn struct_visibility() {
    mod struct_mod {
        #[allow(dead_code)]
        #[derive(Debug)]
        pub struct PrivateFieldsStruct<T> {
            // this field is never directly accessible outside of the
            // struct_mod module
            contents: T,
        }

        impl<T> PrivateFieldsStruct<T> {
            // the method needs to be made explicitly public, too
            pub fn new(contents: T) -> PrivateFieldsStruct<T> {
                PrivateFieldsStruct { contents }
            }
        }

        #[derive(Debug)]
        pub struct PublicFieldsStruct {
            pub contents: Vec<i32>,
        }
    }

    let private_instance = struct_mod::PrivateFieldsStruct::new(vec![1, 2, 3]);
    let public_instance = struct_mod::PublicFieldsStruct {
        contents: vec![1, 2, 3],
    };

    println!("private_instance: {:?}", private_instance);
    println!("public_instance: {public_instance:?}");
}

fn main() {
    visibility();
    struct_visibility();
}
