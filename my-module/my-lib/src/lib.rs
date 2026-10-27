// mod front_of_house {
//     // all are private need pub
//     pub mod hosting {
//         pub fn add_to_waitlist() {

//         }
//     }
// }

mod front_of_house;

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }


mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal: String::from("peaches") }
        }
    }

    pub enum Appitizer {
        Soup,
        Salad
    }
}

// use crate::front_of_house::hosting;
pub use self::front_of_house::hosting;

pub fn eat_at_restaurent() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let appetizer = crate::back_of_house::Appitizer::Soup;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}