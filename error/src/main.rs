use std::fs::File;
use std::io::ErrorKind;
use std::io { self, Read };

fn main() {
//	panic!("crash and burn");
//	let v = vec![1, 2, 3];
	
//	v[99];
/*
	let f = File::open("hello.txt");

	let f = match f {
		Ok(file) => file,
		Err(ref error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => panic!("Fail to create file: {:?}", e),
			},
			other_error => panic!("Cannot open file!: {:?}", other_error),
		},
	};*/
	let f = File::open("hello.txt").expect("File does not exists!");
}

fn read_username_from_file() -> Result<String, io::Error> {
/*	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	}

	let mut s = String::new();

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e)
	}
*/

/*	let mut s = String::new();

	File::open("hello.txt")?.read_to_string(&mut s)?;

	Ok(s)
*/

	fs::read_to_string("hello.txt");

}
