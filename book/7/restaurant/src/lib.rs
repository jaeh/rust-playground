fn serve_order() {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            let breakfast = Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            };
            // we can access seasonal_fruit here, but not outside this fn.
            println!("breakfast.seasonal_fruit = {}", breakfast.seasonal_fruit);
            
            // return breakfast
            breakfast
        }
    }
    
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting;
// or
// use front_of_house::hosting;

// do not use, would make add_to_waitlist() available
// making distinguishing between a local and a used function impossible.
// hosting::add_to_waitlist() is the idiomatic way to call this fn
// use crate::front_of_house::hosting::add_to_waitlist;

pub fn host_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    crate::back_of_house::fix_incorrect_order();

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    // The next two lines won't compile if we uncomment them; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    // println!("meal.seasonal_fruit = {}", meal.seasonal_fruit);
}