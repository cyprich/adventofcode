use std::{fs, path::PathBuf};

fn main() {
    // let filename = "assets/example3.txt";
    let filename = "assets/input3.txt";
    let input = fs::read_to_string(PathBuf::from(filename)).unwrap();

    part1(input.clone());

    // TODO part 2 does not work right
    part2(input);
}

fn part1(input: String) {
    println!("Part 1");
    let joltages: Vec<i32> = input
        .split("\n")
        .map(|line| {
            if !line.trim().is_empty() {
                let first = get_largest(line, true);

                let second = get_substring(line, first.parse().unwrap());
                let second = get_largest(second, false);

                let joltage = first + &second;
                let joltage: i32 = joltage.parse().unwrap();
                joltage
            } else {
                0
            }
        })
        .collect();

    let result: i32 = joltages.iter().sum();

    println!("Result: {}", result);
}

fn part2(input: String) {
    println!("Part 2");
    let joltages: Vec<String> = input
        .split("\n")
        .map(|line| {
            if !line.trim().is_empty() {
                const JOLTAGE_LENGTH: i32 = 12;

                let mut remaining = line;
                let mut result = "".to_string();

                for i in 1..=JOLTAGE_LENGTH {
                    let largest = get_largest_complex(remaining, JOLTAGE_LENGTH - i);
                    result += largest.as_str();
                    remaining = get_substring(remaining, largest.parse().unwrap());
                }

                result
            } else {
                "".to_string()
            }
        })
        .collect();

    let mut result: u128 = 0;

    for j in joltages {
        let n: u128 = j.parse().unwrap_or(0);
        result += n;
    }

    println!("Result {}", result);
}

fn get_largest(s: &str, check_for_last: bool) -> String {
    let mut largest = 0;
    for (i, c) in s.chars().enumerate() {
        let n = c.to_string().trim().parse::<i32>().unwrap();
        if n > largest {
            if check_for_last && i >= s.len() - 1 {
                continue;
            }
            largest = n;
        }

        if largest == 9 {
            break;
        }
    }

    largest.to_string()
}

fn get_largest_complex(s: &str, skip_numbers: i32) -> String {
    let mut largest = 0;

    for (i, c) in s.chars().enumerate() {
        if i > s.len() - skip_numbers as usize {
            break;
        }

        let n = c.to_string().trim().parse::<i32>().unwrap();

        if n == 9 {
            return 9.to_string();
        }

        if n > largest {
            largest = n;
        }
    }

    largest.to_string()
}

fn get_substring(whole_line: &str, number_to_look_for: i32) -> &str {
    let mut result: usize = 0;
    for (index, character) in whole_line.chars().enumerate() {
        let number: i32 = character.to_digit(10).unwrap() as i32;
        if number == number_to_look_for {
            result = index + 1;
            break;
        }
    }

    let result = whole_line.split_at(result);
    result.1
}
