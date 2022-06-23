fn main() {
	{ // s is not available
		let mut s = String::from("hello"); // literal vs String ( s is String, "hello" is literal
		// literal is immutable but String is mutable

		s.push_str(", world!");

		println!("{}", s);
	} // s is not available

	////////////////////////////////

	let s1 = String::from("hello"); 
	let s2 = s1; // move ( is not same to shallow copy!!! )

	//println!("{}, world!", s1); // is not correct
	println!("{}, world!", s2);

	////////////////////////////////
	
	let s1 = String::from("hello");
	let s2 = s1.clone(); // making new data in heap memory

	println!("s1: {} , s2: {}", s1, s2);

	////////////////////////////////
	let s = String::from("hello"); // s is created

	takes_ownership(s); // variable s move into the function
	// s become no longer available

	let x = 5; // x is created
	
	makes_copy(x); // variable x move into the function
	// but i32 type make copy
	// x is avaialbe after this time.

	let s1 = gives_ownership();

	let s2 = String::from("hello");

	let s3 = takes_and_gives_back(s2);

} // x out of bound, s out of bound
// but variable s move into the function so nothing happen.

fn gives_ownership() -> String {
	let some_string = String::from("hello");
	
	some_string
}

fn takes_and_gives_back(some_string: String) -> String {
	some_string
}

fn takes_ownership(some_string: String) { // some_string variable is created
	println!("{}", some_string);
} // some_string out of bound, "drop" function is called
// the memory that some_string used is deallocated.

fn makes_copy(some_integer: i32) { // some_integer variable is created
	println!("{}", some_integer);
} // some_integer out of bound, but nothing happen.
