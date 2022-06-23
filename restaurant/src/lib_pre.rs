fn serve_order() {}


mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {
			super::super::serve_order();	
		}
		
		fn seat_at_table() {}
	}

	mod serving {
		fn take_over() {}
		
		fn serve_order() {}

		fn take_payment() {}
	}
}

mod back_of_house {
	pub struct Breakfast {
		pub toast: String,
		fruit_of_season: String
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				fruit_of_season: String::from("peach")	
			}
		}	
	}
	
	pub enum Appetizer {
		Soup,
		Salad,
	}
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
	hosting::add_to_waitlist();
		
	let order1 = back_of_house::Appetizer::Soup;
	let order2 = back_of_house::Appetizer::Salad;

	let mut meal = back_of_house::Breakfast::summer("rice bread");

	meal.toast = String::from("another bread");
		
	crate::front_of_house::hosting::add_to_waitlist(); // absoulte path

	front_of_house::hosting::add_to_waitlist(); // relative path
}
