use std::fs;

// mod day01;
// mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
mod day08;

fn main() {
    let text = fs::read_to_string("data/day-08.txt").expect("Missing input file.");

    // day08::part1(text.as_str());
    day08::part2(text.as_str());
}
