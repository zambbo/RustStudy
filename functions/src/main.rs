fn main() {
	// statement vs expression
	//let x = (let y = 6);	 statement doesn't return values!!!!!
	let x = { // parenthesis is one of the expression.
		let y = 3; // statement
		y+1 // expression doesn't end with semicolon.
	}; 
	println!("x = {}",x);
	another_function(5, 6);	
	let x = five();

	println!("X value: {}", x);
	println!("X(add one) value: {}", add_one(x));
}

fn add_one(x : i32) -> i32{
	x + 1
}
fn five() -> i32{
	5
}

fn another_function(x: i32, y: i32){
	println!("Hello from another function!");
	println!("X value: {} Y value: {}", x, y);
}
