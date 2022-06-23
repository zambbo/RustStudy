fn main() {
	let mut s = "foo".to_string();
	let s2 = "bar";
	s.push_str(s2);
	println!("s: {}, s2: {}", s, s2);

	let s1 = String::from("hello");
	let s2 = ", world!".to_string();

	let s3 = s1 + &s2; // s1 loose its owenership

	println!("{}", s3);
	println!("{}", s2);
	//println!("{}", s1);


	let s1 = "tic".to_string();
	let s2 = "tac".to_string();
	let s3 = "toe".to_string();
	
	let s = format!("{}-{}-{}", s1, s2, s3);
	println!("{}",s);
	println!("{} {} {}", s1, s2, s3);

	let hello = "안녕하세요";
	
	let s = &hello[0..3];

	println!("{}", s);

	for c in "안녕하세요".chars() {
		println!("{}", c);
	}
	
	for b in "안녕하세요".bytes() {
		println!("{}", b);
	}	
}

