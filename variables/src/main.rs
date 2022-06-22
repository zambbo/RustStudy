fn main() {
	let mut x = 5;
	println!("x value: {}", x);
	x = 6;
	println!("x value: {}", x);

	let tup: (i32, f64, u8) = (500, 6.4, 1);

	let (x, y, z) = tup;
	println!("Y value: {}", y);
	let arr: [i32; 5] = [1, 2, 3, 4, 5];
	println!("arr[0] value: {}", arr[0]);

}
