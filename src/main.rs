use std::fs;

// mod day01;
// mod day02;
mod day03;

fn main() {
    let text = fs::read_to_string("data/day-03.txt").expect("Missing input file.");

    day03::part1(text.as_str());
    day03::part2(text.as_str());
}
