fn main() {
	/*
	let number = 3;
	/*if number { // rust branch condition must be boolean type!!!
		println!("This branch is not correct!");	
	}
	*/
	if number < 5 {
		println!("The condition match!");
	} else {
		println!("Wrong condition!");
	}
	*/

	let condition: bool = true;
	//let number: i32 = if condition { 5 } else { 6 }; // if is expression so it can be used for let.
	
	let number = if condition { 5 } else { 'invalid' }; // it is not correct because if block and else block must return same type
	println!("Number value: {}", number);
}
