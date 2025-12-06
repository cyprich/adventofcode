use std::{fs, num::ParseIntError, path::PathBuf};

pub fn main() {
    let filename = "assets/input2.txt";
    let s = fs::read_to_string(PathBuf::from(filename)).expect("Couldn't read from file");

    part1(s.clone());
    println!();

    part2(s.clone());
}

fn part1(input: String) {
    println!("Day 2 part 1");

    let mut result: u128 = 0;

    for raw_range in input.split(",") {
        let mut splitted = raw_range.split("-");
        let smaller: i128 = parse_number(splitted.next().unwrap()).unwrap();
        let larger: i128 = parse_number(splitted.next().unwrap()).unwrap();

        for i in smaller..=larger {
            if detect_repeating_simple(i.to_string()) {
                result += i as u128;
            }
        }
    }

    // let result = invalids.join("");
    println!("Result: {}", result);
}

fn part2(input: String) {
    println!("Day 2 part 2");

    let mut wrong_ones: Vec<i128> = Vec::new();

    for raw_range in input.split(",") {
        let mut splitted = raw_range.split("-");
        let smaller: i128 = parse_number(splitted.next().unwrap()).unwrap();
        let larger: i128 = parse_number(splitted.next().unwrap()).unwrap();

        for i in smaller..=larger {
            if !detect_repeating_complex(i.to_string()) {
                continue;
            }
            if !wrong_ones.contains(&i) {
                wrong_ones.push(i);
            }
        }
    }

    let result: i128 = wrong_ones.iter().sum();
    println!("Result: {}", result);
}

fn parse_number(s: &str) -> Result<i128, ParseIntError> {
    s.trim().parse()
}

fn detect_repeating_simple(s: String) -> bool {
    if s.len() % 2 == 0 {
        let (first, second) = s.split_at(s.len() / 2);
        first == second
    } else {
        false
    }
}

fn detect_repeating_complex(s: String) -> bool {
    let l = s.len();

    for i in 1..=(l / 2) {
        let checking = s.split_at(i);
        let how_many_times_to_repeat_checking = l / i;
        let s2 = checking.0.repeat(how_many_times_to_repeat_checking);
        if s == s2 {
            return true;
        }
    }

    false
}
