use regex::Regex;

struct Hand {
    red: usize,
    green: usize,
    blue: usize
}

impl Hand {
    fn new() -> Self {
        Hand {red: 0, green: 0, blue: 0}
    }

    fn add_colour(&mut self, colour: &str, n: usize) {
        match colour {
            "red" => self.red += n,
            "green" => self.green += n,
            "blue" => self.blue += n,
            _ => {}
        }
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let cube_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

        let mut hand = Self::new();

        for cube in value.split(",") {
            let caps = cube_re.captures(cube).unwrap();
            let n: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let colour = caps.get(2).unwrap().as_str();

            hand.add_colour(colour, n);
        }

        hand
    }
}

pub fn part1(text: &str) {
    let mut total = 0usize;
    for (i, line) in text.lines().enumerate() {
        let split = line.split(":").collect::<Vec<&str>>();
        let game_str = split.get(1).unwrap();

        let mut valid = true;
        for hand in game_str.split(";") {
            let game= Hand::from(hand);

            if game.red > 12 || game.green > 13 || game.blue > 14 {
                valid = false;
                break;
            }
        }
        if valid {
            total += i + 1;
        }
    }

    println!("{}", total);
}

pub fn part2(text: &str) {
    let mut total = 0usize;
    for line in text.lines() {
        let split = line.split(":").collect::<Vec<&str>>();
        let game_str = split.get(1).unwrap();

        let mut hand_maxes = Hand::new();
        for hand_str in game_str.split(";") {
            let hand = Hand::from(hand_str);

            hand_maxes.red = hand_maxes.red.max(hand.red);
            hand_maxes.green = hand_maxes.green.max(hand.green);
            hand_maxes.blue = hand_maxes.blue.max(hand.blue);
        }

        total += hand_maxes.power();
    }

    println!("{}", total);
}
