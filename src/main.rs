use std::fs;

mod day01;

fn main() {
    let text = fs::read_to_string("data/day-01.txt").expect("Missing input file.");

    // day01::part1(text.as_str());
    day01::part2(text.as_str());
}
