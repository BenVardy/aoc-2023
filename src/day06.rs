use regex::Regex;

fn get_num_opt((time, distance): &(i64, i64)) -> i32 {
    let x1 = ((-time as f64) - ((time * time - (4 * distance)) as f64).sqrt()) / -2.0;
    let x2 = ((-time as f64) + ((time * time - (4 * distance)) as f64).sqrt()) / -2.0;

    let mut min = x1.min(x2).floor();
    let mut max = x1.max(x2);
    if max.fract() == 0.0 {
        max -= 1.0;
    }
    max = max.floor();

    (max - min) as i32
}

pub fn part1(text: &str) {
    let pattern = Regex::new(r"Time:\s+((?:\d+ *)*)\nDistance:\s+((?:\d+ *)*)").unwrap();

    let captures = pattern.captures(text).unwrap();
    let times = captures.get(1).unwrap().as_str().split_whitespace();
    let distances = captures.get(2).unwrap().as_str().split_whitespace();
    let races: Vec<(i64, i64)> = times
        .zip(distances)
        .map(|(t, d)| (t.parse().unwrap(), d.parse().unwrap()))
        .collect();

    println!("{:?}", races);

    let total = races.iter().map(get_num_opt).fold(1, |acc, x| acc * x);

    println!("{}", total);
}

pub fn part2(text: &str) {
    let pattern = Regex::new(r"Time:\s+((?:\d+ *)*)\nDistance:\s+((?:\d+ *)*)").unwrap();

    let captures = pattern.captures(text).unwrap();
    let time: i64 = captures
        .get(1)
        .unwrap()
        .as_str()
        .replace(" ", "")
        .parse()
        .unwrap();
    let distance: i64 = captures
        .get(2)
        .unwrap()
        .as_str()
        .replace(" ", "")
        .parse()
        .unwrap();

    let race = (time, distance);

    println!("{}", get_num_opt(&race));
}
