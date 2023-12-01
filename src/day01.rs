use regex::Regex;
use std::collections::HashMap;

const RE_STR: &str = r"one|two|three|four|five|six|seven|eight|nine|[1-9]";
fn get_number(line: &str) -> i32 {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    for x in line.chars() {
        if x.is_numeric() {
            if first.is_none() {
                first = Some(x);
            }
            last = Some(x)
        }
    }

    format!("{}{}", first.unwrap(), last.unwrap())
        .parse()
        .unwrap()
}

fn get_word_numbers(line: &str, digit_map: &HashMap<&str, i32>) -> i32 {
    let re = Regex::new(RE_STR).unwrap();

    let mut first: Option<i32> = None;
    let mut last: Option<i32> = None;

    let mut i = 0usize;
    while i < line.len() {
        if let Some(m) = re.find_at(line, i) {
            let word = m.as_str();
            let d: i32 = word
                .parse::<i32>()
                .unwrap_or_else(|_| *digit_map.get(word).unwrap());
            if first.is_none() {
                first = Some(d);
            }
            last = Some(d);
            i = m.start() + 1;
        } else {
            // Can end loop.
            i = line.len();
        }
    }

    first.unwrap() * 10 + last.unwrap()
}

pub fn part1(text: &str) {
    let res: i32 = text.lines().map(get_number).sum();
    println!("{}", res);
}

pub fn part2(text: &str) {
    let digit_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let res: i32 = text.lines().map(|l| get_word_numbers(l, &digit_map)).sum();
    println!("{}", res);
}
