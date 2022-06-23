fn main() {
	let s1 = String::from("hello");

	let len = calculate_length_reference(&s1);

	println!("{}'s length is {}", s1, len);

	let (s2, len) = calculate_length_ownership(s1);

	println!("{}'s length is {}", s2, len);

	////////////////////////////////////
	/*
	let mut s = String::from("hello");
	change(&s); // error!
	change(&mut s);

	// race condition
	let r1= &mut s;
	let r2= &mut s;
	println!("{}, {}", r1, r2); 
	*/
	
	let reference_to_nothing = dangle();
}

fn dangle() -> &String {
	let s = String::from("hello");

	&s
}

fn change(some_string: &mut String) {
	some_string.push_str(", world");
}

fn calculate_length_ownership(s: String) -> (String ,usize) {
	let len = s.len();
	
	(s, len)
}

//
fn calculate_length_reference(s: &String) -> usize {
	s.len()
}
