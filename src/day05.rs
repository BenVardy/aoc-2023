use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct MapEntry {
    start_s: i64,
    start_d: i64,
    len: i64,
}

#[derive(Debug)]
struct Mapping {
    to: String,
    entries: Vec<MapEntry>,
}

impl Mapping {
    fn new(to: String) -> Self {
        Mapping {
            to,
            entries: Vec::new(),
        }
    }
}

fn parse_map(text: &str) -> HashMap<String, Mapping> {
    let header_re = Regex::new(r"^(.+?)-to-(.+?) map:$").unwrap();

    let mut map: HashMap<String, Mapping> = HashMap::new();
    for chunk in text.split("\n\n") {
        let (header, maps_str) = chunk.split_once("\n").unwrap();
        let header_matches = header_re.captures(header).unwrap();
        let from = header_matches.get(1).unwrap().as_str();
        let to = header_matches.get(2).unwrap().as_str();

        let mut mapping = Mapping::new(String::from(to));

        for line in maps_str.lines() {
            let mut iter = line.split_whitespace();
            mapping.entries.push(MapEntry {
                start_d: iter.next().unwrap().parse().unwrap(),
                start_s: iter.next().unwrap().parse().unwrap(),
                len: iter.next().unwrap().parse().unwrap(),
            });
        }

        mapping.entries.sort();

        map.insert(String::from(from), mapping);
    }
    map
}

pub fn part1(text: &str) {
    let (seeds_str, rest) = text.split_once("\n\n").unwrap();

    let seeds: Vec<i64> = seeds_str[seeds_str.find(": ").unwrap() + 2..]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let map = parse_map(rest);

    let mut from_str_opt = Some(String::from("seed"));
    let mut values = seeds;
    while let Some(from_str) = from_str_opt {
        from_str_opt = if let Some(mapping) = map.get(&from_str) {
            let mut new_values: Vec<i64> = Vec::new();
            for y in values {
                let mut mapped = false;
                for entry in &mapping.entries {
                    if y >= entry.start_s && y <= entry.start_s + entry.len {
                        new_values.push(entry.start_d + y - entry.start_s);
                        mapped = true;
                    }
                }
                if !mapped {
                    new_values.push(y);
                }
            }
            values = new_values;

            Some(mapping.to.clone())
        } else {
            None
        };
    }

    println!("{}", values.iter().min().unwrap());
}

pub fn part2(text: &str) {
    let (seeds_str, rest) = text.split_once("\n\n").unwrap();
    let map = parse_map(rest);

    let seeds_re = Regex::new(r"\d \d").unwrap();
    let seeds: Vec<(i64, i64)> = seeds_re
        .find_iter(seeds_str)
        .map(|s| {
            let (start, len) = s.as_str().split_once(" ").unwrap();
            let start: i64 = start.parse().unwrap();
            (start, start + len.parse::<i64>().unwrap())
        })
        .collect();

    let mut from_str_opt = Some(String::from("seed"));
    let mut values = seeds;
    while let Some(from_str) = from_str_opt {
        from_str_opt = if let Some(mapping) = map.get(&from_str) {
            values = values
                .iter()
                .flat_map(|(from_start, from_end)| {
                    let mut output: Vec<(i64, i64)> = Vec::new();
                    let mut added: Vec<(i64, i64)> = Vec::new();

                    for entry in &mapping.entries {
                        let to_end_s = entry.start_s + entry.len;
                        if *from_start <= to_end_s && *from_end <= entry.start_s {
                            output.push((
                                entry.start_d + entry.start_s.max(*from_start) - entry.start_s,
                                entry.start_d + to_end_s.min(*from_end) - entry.start_s,
                            ));

                            added.push((entry.start_s.max(*from_start), to_end_s.min(*from_end)))
                        }
                    }

                    // if output.len() == 0 {
                    //     output.push((*from_start, *from_len));
                    // }

                    output
                })
                .collect();

            Some(mapping.to.clone())
        } else {
            None
        }
    }

    println!("{}", values.iter().min().unwrap().0);
}
