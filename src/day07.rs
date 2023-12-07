use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Card(i32);

impl Card {
    fn parse_p1(c: char) -> Result<Self, ()> {
        let val = match c {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => -1,
        };
        if val != -1 {
            Ok(Card(val))
        } else {
            Err(())
        }
    }

    fn parse_p2(c: char) -> Result<Self, ()> {
        let Card(val) = Card::parse_p1(c)?;
        if val == 11 {
            Ok(Card(1))
        } else {
            Ok(Card(val))
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandValue {
    High,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

type Hand = Vec<Card>;

impl HandValue {
    fn get_hand_p1(hand: &Hand) -> Self {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for &Card(c) in hand {
            *map.entry(c).or_insert(0) += 1;
        }

        let (_, &most) = map.iter().max_by(|(_, &v1), (_, &v2)| v1.cmp(&v2)).unwrap();
        match most {
            5 => HandValue::Five,
            4 => HandValue::Four,
            3 => {
                if map.len() == 2 {
                    HandValue::FullHouse
                } else {
                    HandValue::Three
                }
            }
            2 => {
                if map.len() == 3 {
                    HandValue::TwoPair
                } else {
                    HandValue::Pair
                }
            }
            _ => HandValue::High,
        }
    }

    fn get_hand_p2(hand: &Hand) -> Self {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut joker = 0usize;
        for &Card(c) in hand {
            if c != 1 {
                *map.entry(c).or_insert(0) += 1;
            } else {
                joker += 1;
            }
        }

        let (_, &most) = map
            .iter()
            .max_by(|(_, &v1), (_, &v2)| v1.cmp(&v2))
            .unwrap_or((&0, &0));

        if joker == 0 {
            return HandValue::get_hand_p1(hand);
        }

        match most + joker {
            5 => HandValue::Five,
            4 => HandValue::Four,
            3 => {
                if map.len() == 2 {
                    HandValue::FullHouse
                } else {
                    HandValue::Three
                }
            }
            2 => {
                if map.len() == 3 {
                    HandValue::TwoPair
                } else {
                    HandValue::Pair
                }
            }
            _ => HandValue::High,
        }
    }
}

fn ord_hands<F>(hand1: &Hand, hand2: &Hand, valuer: &F) -> Ordering
where
    F: Fn(&Hand) -> HandValue,
{
    let mut ord = valuer(hand1).cmp(&valuer(hand2));
    if ord == Ordering::Equal {
        for (h1, h2) in hand1.iter().zip(hand2) {
            ord = h1.cmp(h2);
            if ord != Ordering::Equal {
                break;
            }
        }
    }
    ord
}

fn calc_hands<F, G>(text: &str, parser: &F, valuer: &G) -> i32
where
    F: Fn(char) -> Result<Card, ()>,
    G: Fn(&Hand) -> HandValue,
{
    let mut hands: Vec<(Hand, i32)> = text
        .lines()
        .map(|line| {
            let (hand_str, bid) = line.split_once(" ").unwrap();
            (
                hand_str.chars().map(|c| parser(c).unwrap()).collect(),
                bid.parse().unwrap(),
            )
        })
        .collect();

    hands.sort_by(|(h1, _), (h2, _)| ord_hands(h1, h2, valuer));

    hands
        .iter()
        .enumerate()
        .map(|(i, (_, val))| (i as i32 + 1) * val)
        .sum()
}

pub fn part1(text: &str) {
    println!(
        "{}",
        calc_hands(text, &Card::parse_p1, &HandValue::get_hand_p1)
    );
}

pub fn part2(text: &str) {
    println!(
        "{}",
        calc_hands(text, &Card::parse_p2, &HandValue::get_hand_p2)
    )
}
