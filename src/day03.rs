use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Number {
    start: usize,
    end: usize,
    val: i32,
}

impl Number {
    fn from(start: usize, end: usize, text: &str) -> Self {
        Number {
            start,
            end,
            val: text[start..=end].parse().unwrap(),
        }
    }

    fn get_neighbours(&self, text: &str, m: usize, n: usize) -> Vec<Symbol> {
        let mut output: Vec<Symbol> = vec![];
        let bytes = text.as_bytes();

        let y = self.start / m;
        let min_x = ((self.start % m).checked_sub(1).unwrap_or_else(|| 0)) + (y * m);
        let max_x = (self.end % m + 1).min(m - 1) + (y * m);

        if y > 0 {
            output.extend(
                (min_x..=max_x)
                    .map(|i| Symbol::from_char(bytes[i - m] as char))
                    .collect::<Vec<_>>(),
            );
        }

        if min_x != self.start {
            output.push(Symbol::from_char(bytes[min_x] as char));
        }
        if max_x != self.end {
            output.push(Symbol::from_char(bytes[max_x] as char));
        }

        if y < n - 1 {
            output.extend(
                (min_x..=max_x)
                    .map(|i| Symbol::from_char(bytes[i + m] as char))
                    .collect::<Vec<_>>(),
            )
        }

        output
    }
}

#[derive(PartialEq)]
enum Symbol {
    None,
    Number,
    Gear,
    Symbol,
}

impl Symbol {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Symbol::None,
            '0'..='9' => Symbol::Number,
            '*' => Symbol::Gear,
            _ => Symbol::Symbol,
        }
    }

    fn is_symbol(&self) -> bool {
        *self == Symbol::Symbol || *self == Symbol::Gear
    }
}

fn get_gear_ratio(i: usize, text: &str, m: usize, n: usize) -> Option<i32> {
    let bytes = text.as_bytes();

    let x = i % m;
    let y = i / m;

    let mut nums: HashSet<Number> = HashSet::new();
    let mut dirs: Vec<usize> = vec![];

    let min_x = x.checked_sub(1).unwrap_or_else(|| 0) + (y * m);
    let max_x = (x + 1).min(m - 1) + (y * m);

    if y > 0 {
        dirs.extend((min_x..=max_x).map(|j| j - m).collect::<Vec<_>>());
    }
    if min_x != x {
        dirs.push(min_x);
    }
    if max_x != x {
        dirs.push(max_x);
    }
    if y < n - 1 {
        dirs.extend((min_x..=max_x).map(|j| j + m).collect::<Vec<_>>());
    }

    for dir in dirs {
        let c = bytes[dir] as char;
        if !c.is_digit(10) {
            continue;
        }

        let mut start = dir;
        let mut end = dir;
        while start % m > 0 && Symbol::from_char(bytes[start - 1] as char) == Symbol::Number {
            start -= 1;
        }
        while end % m < m - 1 && Symbol::from_char(bytes[end + 1] as char) == Symbol::Number {
            end += 1;
        }

        nums.insert(Number::from(start, end, text));
    }

    if nums.len() == 2 {
        Some(nums.iter().map(|x| x.val).fold(1, |acc, e| acc * e))
    } else {
        None
    }
}

pub fn part1(text: &str) {
    let m = text.find("\n").unwrap();
    let n = text.len() / m - 1;

    let text = text.replace("\n", "");

    let mut num_start: Option<usize> = None;
    let mut nums: Vec<Number> = vec![];

    for (i, c) in text.chars().enumerate() {
        let sym = Symbol::from_char(c);
        match sym {
            Symbol::Number => {
                if num_start.is_none() {
                    num_start = Some(i);
                }
            }
            _ => {}
        }

        if sym != Symbol::Number {
            if let Some(start) = num_start {
                nums.push(Number::from(start, i - 1, text.as_str()));
                num_start = None;
            }
        }
    }

    let mut total = 0;
    for num in nums {
        let mut neighbours = num.get_neighbours(text.as_str(), m, n);
        neighbours.retain(|x| x.is_symbol());
        if neighbours.len() > 0 {
            total += num.val;
        }
    }

    println!("{}", total);
}

pub fn part2(text: &str) {
    let m = text.find("\n").unwrap();
    let n = text.len() / m - 1;

    let text = text.replace("\n", "");
    let mut total = 0;

    for (i, c) in text.chars().enumerate() {
        if Symbol::from_char(c) == Symbol::Gear {
            if let Some(val) = get_gear_ratio(i, text.as_str(), m, n) {
                total += val;
            }
        }
    }

    println!("{}", total);
}
