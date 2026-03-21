// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

use std::{cmp::Ordering, io}; // syntaxe pour importer plusieur modules de std ici
                              //
                              // ramplace use std::cmp::Ordering;
                              //          use std::io;

use std::collections::*; // syntaxe d'import global    

mod front_of_house;

fn deliver_order() {} // pourais etre deplacer dans un autre ficher de le même manière que front_of_house
mod back_of_house {

    // ================ rendre un enumù public rend tout ces champs public ================
        pub enum Appetizer {
        Soup,
        Salad,
    }

    // ================ rendre un struct public ne rend pas public ces champs (permet le cas par cas) ================
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {   // * dans le cas ou on garde des champs priver il faut une fonction (ici summer) qui initialise la struct 
            Breakfast {                             // * et renvoie une instance de celle ci sinon l'utilisateur sans dans l'autre scope ne poura pas ecrire $seasonal_fruit 
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

use crate::front_of_house::hosting; // use permet de ne pas réecrire les chemin a chaque fois
                                    // * ! seulement utilisable dans le scope actuelle
                                    // il est possible d'utiliser pub pour rendre "l'import" utilisable dans d'autre scope

use std::fmt::Result;        
use std::io::Result as IoResult;  // ici as permet d'utiliser deux Result different dans le scope

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // avec use
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}