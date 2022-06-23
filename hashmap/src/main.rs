use std::collections::HashMap;

fn main() {

	let mut scores = HashMap::new();
	let blue = String::from("blue");
	scores.insert(blue, 10);  // blue move ownership
	scores.insert(String::from("yellow"), 50);

	let teams = vec!["blue".to_string(), "yellow".to_string()];
	let initial_scores = vec![10, 50];
	
	let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
	
	let team_name = String::from("blue");
	let score = scores.get(&team_name);
	println!("score: {:?}", score);

	for (key, item) in &scores {
		println!("{} : {}", key, item);	
	}

	let black = "black".to_string();
	let black_score = 40;
	scores.entry(&black).or_insert(&black_score);
	println!("scores: {:?}", scores);

	let text = "hello world wonderful world";
	
	let mut map = HashMap::new();

	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}
	println!("map : {:?}", map);	
}
