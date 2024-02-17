pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

    }

    pub mod serving {
        fn take_order() {}
    }
}

// This mod is private, because it's "mod", not "pub mod", so one can not use crate::libs::back_of_house,
// but can use crate::libs::front_of_house
mod back_of_house {
    // in order to be able to do it, the module must be defined public
    pub mod cook {
        pub fn make_steak() {}
        pub fn make_salad() {}
    }


    // "super" references a function in a parent of back_of_house
    fn fix_incorrect_order() {
        super::deliver_order();
    }

    mod accounting {
        fn calculate_revenue() {}
    }
}

fn deliver_order() {}

pub fn cook_meal() {
    // both the cook module and make_steak / make_salad functions are public, so we can use them
    // we don't expose the whole back_of_hose though
    back_of_house::cook::make_salad();
    back_of_house::cook::make_steak();
}