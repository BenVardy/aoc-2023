use std::fs;

// mod day01;
// mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
mod day07;

fn main() {
    let text = fs::read_to_string("data/day-07.txt").expect("Missing input file.");

    day07::part1(text.as_str());
    day07::part2(text.as_str());
}
