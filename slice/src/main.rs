fn main() {
	let mut s = String::from("hello world");

	let word = first_word(&s);
	println!("{}",word);

	let s = String::from("hello world");

	let hello = &s[0..5];
	let world = &s[6..];

	println!("{}, {}", hello, world);
	println!("{}", first_word_slice(&s));
}


fn first_word(s: &String) -> usize {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		
		if item == b' ' { return i; } 	
	}
	s.len()
}

fn first_word_slice(s: &String) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[..i];
		}
	}
	
	&s[..]
}
