use std::{fs, num::ParseIntError, path::PathBuf};

pub fn main() {
    part1();
}

fn part1() {
    println!("Day 2 part 1");

    let filename = "assets/input2.txt";
    let s = fs::read_to_string(PathBuf::from(filename)).expect("Couldn't read from file");

    let mut result: u128 = 0;

    for raw_range in s.split(",") {
        let mut splitted = raw_range.split("-");
        let smaller: i128 = parse_number(splitted.next().unwrap()).unwrap();
        let larger: i128 = parse_number(splitted.next().unwrap()).unwrap();

        for i in smaller..=larger {
            if detect_repeating(i.to_string()) {
                result += i as u128;
            }
        }
    }

    // let result = invalids.join("");
    println!("Result: {}", result);
}

fn parse_number(s: &str) -> Result<i128, ParseIntError> {
    s.trim().parse()
}

fn detect_repeating(s: String) -> bool {
    if s.len() % 2 == 0 {
        let (first, second) = s.split_at(s.len() / 2);
        first == second
    } else {
        false
    }
}
