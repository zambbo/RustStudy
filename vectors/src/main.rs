#[derive(Debug)]
enum SpreadSheetCell {
	Int(i32),
	Float(f64),
	Text(String)
}


fn main() {
	let v: Vec<i32> = Vec::new();

	let v = vec![1, 2, 3];

	let mut v = Vec::new();
	
	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8);
	
	let v = vec![1, 2, 3, 4, 5];

	let third: &i32 = &v[2];

	println!("Third element of vector: {}", third);

	match v.get(2) {
		
		Some(third) => println!("Third element of vector: {}", third),
		None => println!("No third element!"),
	}

	let mut v = vec![1, 2, 3, 4, 5];
	for i in &mut v {
		*i += 50
	}

	for item in &mut v { println!("element is {}", item);}
	
	let row = vec![
		SpreadSheetCell::Int(3),
		SpreadSheetCell::Float(10.12),
		SpreadSheetCell::Text(String::from("apple"))
	];
	
	for i in &row { println!("{:?}",i);}
}

