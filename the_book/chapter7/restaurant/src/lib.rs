#[allow(dead_code)]
mod front_of_house {

    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

#[allow(dead_code)]
mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

use crate::front_of_house::hosting;

#[allow(dead_code)]
mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant_too() {
        // To use 'use' keyword it has to be difined on the module
        hosting::add_to_waitlist();
        // Or use super to acess the outer call of the 'use'
        super::hosting::add_to_waitlist();
    }
}
#[allow(unused_variables)]
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a Breakfast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change the toast value
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Order a Appetizer
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Shorter version using 'use'
    hosting::add_to_waitlist();
}

//// Using the 'use' keyword for two different modules that have the same name
// use std::{fmt, io};
// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

//// Another approach is using alias
// use std::fmt::Result;
// use std::io::Result as IoResult;
// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}

//// Nested Paths example
//use std::io::{self, Write, Read as r};

//// Using the glob operator example
// use std::collections::*;

// Example of module separation
mod new_front_of_house;
pub use crate::new_front_of_house::new_hosting;
pub fn eat_at_new_restaurant() {
    hosting::add_to_waitlist();
}
