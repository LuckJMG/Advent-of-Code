// Part 1
use std::fs::read_to_string;

fn main() {
	let mut count: u32 = 0;

	for line in read_to_string("src/input.txt").unwrap().lines() {
		let pair = line.split_once(',').unwrap();
		let left_areas = get_limits(pair.0);
		let right_areas = get_limits(pair.1);

		if (left_areas.0 >= right_areas.0 && left_areas.1 <= right_areas.1)
			|| (left_areas.0 <= right_areas.0 && left_areas.1 >= right_areas.1) {
			count += 1;
		}
	}

	println!("{count}");
}

fn get_limits(string: &str) -> (u32, u32) {
	let (bot, top) = string.split_once('-').unwrap();
	(bot.parse::<u32>().unwrap(), top.parse::<u32>().unwrap())
}

// Part 2
use std::fs::read_to_string;

fn main() {
	let mut count: u32 = 0;

	for line in read_to_string("src/input.txt").unwrap().lines() {
		let pair = line.split_once(',').unwrap();
		let left_areas = get_limits(pair.0);
		let right_areas = get_limits(pair.1);

		if (left_areas.1 >= right_areas.0 && left_areas.0 <= right_areas.1)
			|| (left_areas.0 <= right_areas.1 && left_areas.1 >= right_areas.0) {
			count += 1;
		}
	}

	println!("{count}");
}

fn get_limits(string: &str) -> (u32, u32) {
	let (bot, top) = string.split_once('-').unwrap();
	(bot.parse::<u32>().unwrap(), top.parse::<u32>().unwrap())
}


