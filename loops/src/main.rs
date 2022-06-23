fn main() {
	/*
	loop {
		println!("again!");
	}
	*/

	let mut counter = 0;

	let result = loop {
		counter += 1;
		
		if counter == 10 {
			break counter * 2 ;
		}
	};

	println!("Result: {}", result);	

	//////////////////////////////////
	let mut number = 3;

	while number != 0 {
		println!("{}!", number);
		
		number -= 1;
	}

	println!("shoot!!!");

	/////////////////////////////////

	let a = [10, 20, 30, 40, 50];
	let mut index = 0;

	while index < 5 {
		println!("Element value: {}", a[index]);

		index += 1;

	}

	//////////////////////////////////

	for element in a.iter() {
		println!("Element value(for): {}", element);
	}

	//////////////////////////////////

	for number in (1..4).rev() {
		println!("{}!", number);
	}
	println!("Shoot!!!");


}
