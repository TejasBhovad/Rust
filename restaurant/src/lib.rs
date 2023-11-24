use std::io::{self, Write};
// multiple imports

mod front_of_the_house {
    pub struct Management {
        pub manager: String,
        bellboy: String,
    }
    impl Management {
        pub fn management(manager: &str, bellboy: &str) -> Management {
            Management {
                manager: String::from(manager),
                bellboy: String::from(bellboy),
            }
        }
    }
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {
            super::serving::take_order();
            // super references parent here it is default parent 'crate'
        }
    }
    mod serving {
        pub fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
// crate
// ├─ front_of_the_house
// │  ├─ hosting
// │  │  ├─ add_to_waitlist
// │  │  ├─ seat_at_table
// │  ├─ serving
// │  │  ├─ take_order
// │  │  ├─ serve_order
// │  │  ├─ take_payment
// https://ascii-tree-generator.com/
pub use crate::front_of_the_house::hosting;
mod back_of_the_house; // importing folder as mod
pub use crate::back_of_the_house::cooking; //calling external mod
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_the_house::hosting::add_to_waitlist();
    // relative path
    front_of_the_house::hosting::add_to_waitlist();

    let mut _Seniors = front_of_the_house::Management::management("Rudolf", "Silly");
    // struct declared in mod cant be called directly unless all fields of the struct are made public
    // but enums can be used regardless
}

// by default child modules are private but child modules can see every other info
