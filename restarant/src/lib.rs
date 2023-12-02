use back_of_house::Appetizer;

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

// Shortening the path
use front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Shortened path due to `use`
    hosting::add_to_waitlist()
}

#[allow(dead_code)]
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Recall: use impl $struct to declare functions associated to $struct
    impl Breakfast {
        // If the function doesn't take in a Self, then it's not a method,
        // but simply an associated function, which needs :: to be called
        /// Returns a summer breakfast
        pub fn summmer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_breakfast() {
    // Create a breakfast order
    let mut meal = back_of_house::Breakfast::summmer("Rye");
    // Change our mind about the bread type
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // Cannot change the seasonal fruit
    // meal.seasonal_fruit = String::from("apples") // error: field `seasonal_fruit` of `Breakfast` is private

    // You cannot create Breakfast using the literal syntax due to the presence
    // of a private field
    // let mut breakfast = back_of_house::Breakfast {
    //     toast: String::from("asdf"),
    //     seasonal_fruit: String::from("sss"), // error: field is private
    // };
}

pub fn eat_appetizer() {
    let _salad = back_of_house::Appetizer::Salad;
    let _soup = Appetizer::Soup; // weird, why doesn't this need back_of_house?
}

fn deliver_order() {}
