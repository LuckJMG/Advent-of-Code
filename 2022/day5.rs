// Part 1
use std::fs::read_to_string;

fn main() {
	let mut stacks: Vec<Vec<char>> = Vec::new();
	stacks.push(vec!['T', 'D', 'W', 'Z', 'V', 'P']);
	stacks.push(vec!['L', 'S', 'W', 'V', 'F', 'J', 'D']);
	stacks.push(vec!['Z', 'M', 'L', 'S', 'V', 'T', 'B', 'H']);
	stacks.push(vec!['R', 'S', 'J']);
	stacks.push(vec!['C', 'Z', 'B', 'G', 'F', 'M', 'L', 'W']);
	stacks.push(vec!['Q', 'W', 'V', 'H', 'Z', 'R', 'G', 'B']);
	stacks.push(vec!['V', 'J', 'P', 'C', 'B', 'D', 'N']);
	stacks.push(vec!['P', 'T', 'B', 'Q']);
	stacks.push(vec!['H', 'G', 'Z', 'R', 'C']);

	for line in read_to_string("src/input.txt").unwrap().lines() {
		let instruction: Vec<&str> = line.split(' ').collect();
		let amount = instruction[1].parse::<usize>().unwrap();
		let start_stack = instruction[3].parse::<usize>().unwrap() - 1;
		let end_stack = instruction[5].parse::<usize>().unwrap() - 1;

		for _ in 0..amount {
			if stacks[start_stack].is_empty() {
				break;
			}

			let cage = stacks[start_stack].pop().unwrap();
			stacks[end_stack].push(cage);
		}
	}

	for stack in stacks {
		print!("{}", stack.last().unwrap());
	}
}

// Part 2
use std::fs::read_to_string;

fn main() {
	let mut stacks: Vec<Vec<char>> = Vec::new();
	stacks.push(vec!['T', 'D', 'W', 'Z', 'V', 'P']);
	stacks.push(vec!['L', 'S', 'W', 'V', 'F', 'J', 'D']);
	stacks.push(vec!['Z', 'M', 'L', 'S', 'V', 'T', 'B', 'H']);
	stacks.push(vec!['R', 'S', 'J']);
	stacks.push(vec!['C', 'Z', 'B', 'G', 'F', 'M', 'L', 'W']);
	stacks.push(vec!['Q', 'W', 'V', 'H', 'Z', 'R', 'G', 'B']);
	stacks.push(vec!['V', 'J', 'P', 'C', 'B', 'D', 'N']);
	stacks.push(vec!['P', 'T', 'B', 'Q']);
	stacks.push(vec!['H', 'G', 'Z', 'R', 'C']);

	let mut picker: Vec<char> = Vec::new();
	for line in read_to_string("src/input.txt").unwrap().lines() {
		let instruction: Vec<&str> = line.split(' ').collect();
		let amount = instruction[1].parse::<usize>().unwrap();
		let start_stack = instruction[3].parse::<usize>().unwrap() - 1;
		let end_stack = instruction[5].parse::<usize>().unwrap() - 1;

		for _ in 0..amount {
			if stacks[start_stack].is_empty() {
				break;
			}

			picker.push(stacks[start_stack].pop().unwrap());
		}
		picker.reverse();
		stacks[end_stack].append(&mut picker);
		picker.clear();
	}

	for stack in stacks {
		print!("{}", stack.last().unwrap());
	}
}
