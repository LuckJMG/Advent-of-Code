use std::fs::read_to_string;

fn main() {
	let mut total_priority: u32 = 0;

	let binding = read_to_string("src/input.txt").unwrap();
	let mut input = binding.lines();
	while let Some(line) = input.next() {
		let elf1 = line;
		let elf2 = input.next().unwrap();
		let elf3 = input.next().unwrap();
		

		for item in elf1.chars() {
			if elf2.contains(item) && elf3.contains(item) {
				total_priority += item as u32 - if item.is_uppercase() { 38 } else { 96 };
				break;
			}
		}
	}

	println!("{total_priority}");
}

