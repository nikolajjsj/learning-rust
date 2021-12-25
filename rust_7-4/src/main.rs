mod front_of_house {
    pub mod hosting {
        pub fn add_to_wishlist() {}
    }
}

use crate::front_of_house::hosting;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    hosting::add_to_wishlist();

    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..101);
}
