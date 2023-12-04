use std::fs;

// mod day01;
// mod day02;
// mod day03;
mod day04;

fn main() {
    let text = fs::read_to_string("data/day-04.txt").expect("Missing input file.");

    day04::part1(text.as_str());
    day04::part2(text.as_str());
}
