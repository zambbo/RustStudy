#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height	
	}
	
	fn can_hold(&self, rect: &Rectangle) -> bool {
		return self.width > rect.width && self.height > rect.height;
	}
}

impl Rectangle {
	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size
		}
	}
}

fn main() {
	let rect = Rectangle { width:30, height:50 };

	println!("Area of Rectangle: {} pixels", area(&rect));
	println!("Area of Rectangle: {} pixels", rect.area());
	println!("rect: {:?}", rect);

	let rect1 = Rectangle { width: 30, height:50 };
	let rect2 = Rectangle { width: 10, height:40 };
	let rect3 = Rectangle { width: 60, height:45 };

	println!("rect1 contain rect2? {}", rect1.can_hold(&rect2));
	println!("rect1 contain rect3? {}", rect1.can_hold(&rect3));
	
	let square = Rectangle::square(50);
	println!("Area of sqaure: {}pixels", square.area());
}

fn area(rect: &Rectangle) -> u32 {
	rect.width * rect.height
}
