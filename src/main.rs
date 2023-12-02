use std::fs;

// mod day01;
mod day02;

fn main() {
    let text = fs::read_to_string("data/day-02.txt").expect("Missing input file.");

    day02::part1(text.as_str());
    day02::part2(text.as_str());
}
