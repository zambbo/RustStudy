/*
fn largest<T> (list: &[T]) -> T {
	let mut largest = list[0];

	for &item in list.iter() {
		if item > largest { largest = item; }
	}

	largest
}*/
/*
struct Point<T, U> {
	x: T,
	y: U,
}*/
/*
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	fn x(&self) -> &T { &self.x }
}

impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}
*/

struct Point<T, U> {
	x: T,
	y: U,
}

impl<T, U> Point<T, U> {
	fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
		Point {
			x: self.x,
			y: other.y,
		}
	}
}


fn main() {
	let p1 = Point { x:3, y:2.5};
	let p2 = Point { x: "hello", y: 'k'};

	let p3 = p1.mix_up(p2);
	println!("p3.x : {}, p3.y: {}", p3.x, p3.y);
/*
	let number_list = vec![34, 50, 25, 100, 65];

	let result = largest(&number_list);
	println!("largest number: {}", result);

	let char_list = vec!['y', 'm', 'a', 'q'];
	
	let result = largest(&char_list);
	println!("largest character: {}", result);
*/

//	let point1 = Point { x: 5, y: 0.1};
/*
	let point = Point {x: 5, y: 3};
	println!("x: {}", point.x);

	let point_f = Point {x: 5.0, y: 3.0};
	println!("d: {}", point_f.distance_from_origin());
*/

}
