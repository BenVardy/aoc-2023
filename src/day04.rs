use std::collections::{HashMap, HashSet};

fn get_n_matches(card: &str) -> usize {
    let numbers = card.split(": ").nth(1).unwrap();
    let mut halves = numbers.split(" | ");
    let res: HashSet<i32> = halves
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let my: HashSet<i32> = halves
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    res.intersection(&my).collect::<Vec<_>>().len()
}

pub fn part1(text: &str) {
    let mut total = 0;
    for line in text.lines() {
        let n_matches = get_n_matches(line);
        if n_matches > 0 {
            total += 1 << (n_matches - 1);
        }
    }

    println!("{}", total);
}

fn calculate_cards(lines: &Vec<&str>, i: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(&v) = cache.get(&i) {
        v
    } else {
        let mut total = 1;
        let n_matches = get_n_matches(lines.get(i).unwrap());
        for j in (i + 1)..(i + 1 + n_matches).min(lines.len()) {
            total += calculate_cards(lines, j, cache);
        }

        cache.insert(i, total);
        total
    }
}

pub fn part2(text: &str) {
    let lines: Vec<&str> = text.lines().collect();
    let n = lines.len();

    let mut cache: HashMap<usize, usize> = HashMap::new();

    let total: usize = (0..n).map(|i| calculate_cards(&lines, i, &mut cache)).sum();

    println!("{}", total);
}
