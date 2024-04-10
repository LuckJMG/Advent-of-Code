struct Hand {
    red: u8,
    green: u8,
    blue: u8,
}

impl Hand {
    fn check(&self, max_hand: &Hand) -> bool {
        println!("{} red, {} green, {} blue", self.red, self.green, self.blue);
        if self.red <= max_hand.red 
            && self.blue <= max_hand.blue 
            && self.green <= max_hand.green {
            return true;
        }

        return false;
    }
}

struct Game {
    id: u8,
    hands: Vec<Hand>,
}

impl Game {
    fn from(mut input: String) -> Self {
        let id = input[5..input.find(':').unwrap()].parse::<u8>().unwrap();
        input.replace_range(0..input.find(':').unwrap()+2, "");
        input = input.replace(',', "");

        let game = input.split("; ");
        let mut hands: Vec<Hand> = Vec::new();

        for raw_hand in game {
            let mut values = raw_hand.split(' ');
            let mut hand = Hand { red: 0, green: 0, blue: 0 };

            loop {
                let num: u8;
                match values.next() {
                    Some(n) => num = n.parse::<u8>().unwrap(),
                    None => break,
                }

                let color = values.next().unwrap();

                match color {
                    "red" => hand.red = num,
                    "green" => hand.green = num,
                    "blue" => hand.blue = num,
                    s => println!("{}", s),
                }
            }

            hands.push(hand);
        }

        Self {
            id,
            hands
        }
    }

    // Part 1
    fn check(&self, max_hand: &Hand) -> u8 {
        println!("Game {}", self.id);

        for hand in &self.hands {
            if !hand.check(max_hand) { return 0; }
        }

        return self.id;
    }

    // Part 2
    fn min_hand(&self) -> Hand {
        let mut min_hand = Hand { red: 0, green: 0, blue: 0 };

        for hand in self.hands.iter() {
            if hand.red > min_hand.red { min_hand.red = hand.red; }
            if hand.green > min_hand.green { min_hand.green = hand.green; }
            if hand.blue > min_hand.blue { min_hand.blue = hand.blue; }
        }

        return min_hand;
    }
}
