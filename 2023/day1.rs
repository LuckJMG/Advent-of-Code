use std::fs::read_to_string;

// Part 1
fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let lines = input.split('\n');
    let mut result: i32 = 0;

    for line in lines {
        if line.is_empty() { break; }

        let mut calibration_value = line.chars()
                                        .find(|x| x.is_numeric())
                                        .unwrap()
                                        .to_string();

        calibration_value.push(line.chars()
                                    .rev()
                                    .find(|x| x.is_numeric())
                                    .unwrap());

        result += calibration_value.parse::<i32>().unwrap();
    }

    println!("{}", result);
}

// Part 2
const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let input = read_to_string("src/input.txt").unwrap();

    let lines = input.split('\n');
    let mut result: u32 = 0;


    for line in lines {
        if line.is_empty() { break; }

        let mut num = String::new();
        let mut head = String::new();
        for char in line.chars() {
            if num.len() == 1 { break; }

            if char.is_numeric() {
                num.push(char);
                head.clear();
                break;
            }

            head.push(char);
            for i in 1..=9 {
                if !head.contains(DIGITS[i-1]) { continue; }

                num.push_str(&i.to_string());
                head.clear();
                break;
            }
        }

        for char in line.chars().rev() {
            if num.len() == 2 { break; }

            if char.is_numeric() {
                num.push(char);
                head.clear();
                break;
            }

            head = char.to_string() + &head;
            for i in 1..=9 {
                if !head.contains(DIGITS[i-1]) { continue; }

                num.push_str(&i.to_string());
                head.clear();
                break;
            }
        }

        println!("{}: {}", line, num);
        result += num.parse::<u32>().unwrap();
    }

    println!("{}", result);
}
