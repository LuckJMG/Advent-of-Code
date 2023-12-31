use std::fs::read_to_string;
use std::path::Path;

// Part 1
fn main() {
    const GAME_POINTS: (u32, u32, u32) = (0, 3, 6); // (lose, tie, win)
    const HAND_POINTS: (u32, u32, u32) = (1, 2, 3); // (rock, paper, scissor)

    let path: &Path = Path::new("./src/input.txt");
    let mut opponent: u32;
    let mut hand: u32;
    let mut sum: u32 = 0;

    for line in read_to_string(path).unwrap().lines() {
        let line: Vec<&str> = line.split_whitespace().collect();

        match *line.first().unwrap() {
            "A" => opponent = HAND_POINTS.0,
            "B" => opponent = HAND_POINTS.1,
            "C" => opponent = HAND_POINTS.2,
            _ => opponent = 0,
        }

        match *line.last().unwrap() {
            "X" => hand = HAND_POINTS.0,
            "Y" => hand = HAND_POINTS.1,
            "Z" => hand = HAND_POINTS.2,
            _ => hand = 0,
        }

        if hand == opponent {
            sum += GAME_POINTS.1;
        }
        else {
            let win: bool = (hand == HAND_POINTS.0 && opponent == HAND_POINTS.2) ||
                            (hand == HAND_POINTS.1 && opponent == HAND_POINTS.0) ||
                            (hand == HAND_POINTS.2 && opponent == HAND_POINTS.1);

            if win { sum += GAME_POINTS.2 }
        }

        sum += hand
    }

    println!("{sum}");
}

// Part 2
fn main() {
    const GAME_POINTS: (u32, u32, u32) = (0, 3, 6); // (lose, tie, win)
    const HAND_POINTS: (u32, u32, u32) = (1, 2, 3); // (rock, paper, scissor)

    let path: &Path = Path::new("./src/input.txt");
    let mut opponent: u32;
    let mut result: u32;
    let mut sum: u32 = 0;

    for line in read_to_string(path).unwrap().lines() {
        let line: Vec<&str> = line.split_whitespace().collect();

        match *line.first().unwrap() {
            "A" => opponent = HAND_POINTS.0,
            "B" => opponent = HAND_POINTS.1,
            "C" => opponent = HAND_POINTS.2,
            _ => opponent = 0,
        }

        match *line.last().unwrap() {
            "X" => result = GAME_POINTS.0,
            "Y" => result = GAME_POINTS.1,
            "Z" => result = GAME_POINTS.2,
            _ => result = 0,
        }

        sum += result;

        if result == GAME_POINTS.1 { sum += opponent }
        else if result == GAME_POINTS.0 {
            match opponent {
                1 => sum += HAND_POINTS.2,
                2 => sum += HAND_POINTS.0,
                3 => sum += HAND_POINTS.1,
                _ => (),
            }
        }
        else if result == GAME_POINTS.2 {
            match opponent {
                1 => sum += HAND_POINTS.1,
                2 => sum += HAND_POINTS.2,
                3 => sum += HAND_POINTS.0,
                _ => (),
            }
        }
    }

    println!("{sum}");
}
