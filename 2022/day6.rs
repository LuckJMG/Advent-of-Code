// Part 1
use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
	let stream = read_to_string("src/input.txt").unwrap();
	
	for index in 0..stream.len()-4 {
		let marker: HashSet<char> = stream[index..index+4].chars().collect();
		if marker.len() == 4 {
			println!("{}", index+4);
			break;
		}
	}
}

// Part 2
use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
	let stream = read_to_string("src/input.txt").unwrap();
	
	for index in 0..stream.len()-14 {
		let marker: HashSet<char> = stream[index..index+14].chars().collect();
		if marker.len() == 14 {
			println!("{}", index+14);
			break;
		}
	}
}

