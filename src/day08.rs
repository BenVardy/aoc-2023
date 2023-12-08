use regex::Regex;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Barrier};
use std::thread;
use std::thread::{JoinHandle, Thread};

fn parse(text: &str) -> (String, HashMap<String, (String, String)>) {
    let rl_re = Regex::new(r"\((\w{3}), (\w{3})\)").unwrap();

    let (header, body) = text.split_once("\n\n").unwrap();

    let map = HashMap::from_iter(body.lines().map(|line| {
        let (node, rest) = line.split_once(" = ").unwrap();

        let captures = rl_re.captures(rest).unwrap();

        (
            String::from(node),
            (
                String::from(captures.get(1).unwrap().as_str()),
                String::from(captures.get(2).unwrap().as_str()),
            ),
        )
    }));

    (String::from(header), map)
}

pub fn part1(text: &str) {
    let (header, map) = parse(text);

    let mut count = 0;
    let mut current = &String::from("AAA");
    let head_chars: Vec<char> = header.chars().collect();
    while current != "ZZZ" {
        let &dir = head_chars.get(count % head_chars.len()).unwrap();
        current = if dir == 'L' {
            &map.get(current).unwrap().0
        } else {
            &map.get(current).unwrap().1
        };
        count += 1;
    }

    println!("{}", count);
}

fn gcd(a: usize, b: usize) -> usize {
    let mut t: usize;
    let mut a = a;
    let mut b = b;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn part2(text: &str) {
    let (header, map) = parse(text);
    let head_chars: Vec<char> = header.chars().collect();

    let mut starts: Vec<&String> = map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(k, _)| k)
        .collect();

    let mins: Vec<_> = starts
        .iter()
        .map(|&s| {
            let mut current = s;
            let mut count = 0;
            while !current.ends_with('Z') {
                let &dir = head_chars.get(count % head_chars.len()).unwrap();
                current = if dir == 'L' {
                    &map.get(current).unwrap().0
                } else {
                    &map.get(current).unwrap().1
                };
                count += 1;
            }

            count
        })
        .collect();

    let &first = mins.iter().next().unwrap();
    let lcm = mins
        .iter()
        .skip(1)
        .fold(first, |acc, &a| acc * (a / gcd(acc, a)));

    println!("{}", lcm);
}
