/*
#[derive(Debug)]
enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32)
}

impl Message {
	fn printMessage(&self) {
		println!("{:?}",self);
	}
}
*/

#[derive(Debug)]
enum UsState {
	Alabama, Alaska, NewYork
}

enum Coin {
	Penny,
	Nickle,
	Dime,
	Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u32 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickle => 5,
		Coin::Dime => 10,
		Coin::Quarter(state)  => {
			println!("State quarter from {:?}!", state);
			25
		},
	}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	} 
}

fn main() {
	value_in_cents(Coin::Quarter(UsState::NewYork));
	let five = Some(5);
	let six = plus_one(five);
	let seven = plus_one(None);
	
	let some_u8_value = Some(0u8);
	match some_u8_value {
		Some(3) => println!("three!"),
		_ => (),
	}

	let coin1 = Coin::Quarter(UsState::Alaska);
	let mut count = 0;
	let coin2 = Coin::Penny;

	if let Coin::Quarter(state) = coin1 {
		println!("{:?}'s coin", state);
	} else {
		count += 1;
	}
	if let Coin::Penny = coin2 { println!("Penny!");}
	//println!("{} {} {}", five, six, seven);
/*
	let q = Message::Quit;
	let m = Message::Move { x: 20, y: 20};
	let w = Message::Write(String::from("inner_message"));
	let c = Message::ChangeColor(20,15,35);

	q.printMessage();
	m.printMessage();
	w.printMessage();
	c.printMessage();
*/
}
