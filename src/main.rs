use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let path: &Path = Path::new("./src/input.txt");
    let mut greatest: [u32; 3] = [0; 3];
    let mut min: u32 = 0;
    let mut sum: u32 = 0;

    for line in read_to_string(path).unwrap().lines() {
        if line.is_empty() {
            if sum > min {
                let index = greatest.iter().position(|&x| x == min ).unwrap();
                greatest[index] = sum;
                min = *greatest.iter().min().unwrap();
            }
            sum = 0;
            continue;
        }

        sum += line.parse::<u32>().unwrap();
    }

    println!("{}", greatest.iter().sum::<u32>());
}
