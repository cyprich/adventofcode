use std::{fs, path::PathBuf, thread::sleep, time::Duration};

pub fn main() {
    part1();
    println!();
    part2();
}

pub fn part1() {
    println!("Day 1 part 1");

    let mut pointing_at = 50;
    let mut result = 0;

    let filename = "inputs/input1.txt";
    let lines = fs::read_to_string(PathBuf::from(filename)).expect("Couln't read file");

    for line in lines.split("\n") {
        let command = line.chars().next().unwrap_or(' ');

        let number: String = line.chars().skip(1).collect();
        let number: i32 = number.trim().parse().unwrap_or(0);

        match command {
            'R' => pointing_at += number,
            'L' => pointing_at -= number,
            _ => println!("Couldn't parse line: {}", line),
        }

        while pointing_at >= 100 {
            pointing_at -= 100;
        }

        while pointing_at < 0 {
            pointing_at += 100;
        }

        if pointing_at == 0 {
            result += 1;
        }
    }

    println!("Pointing at: {}", pointing_at);
    println!("Result: {}", result);
}

pub fn part2() {
    println!("Day 1 part 2");

    let mut pointing_at = 50;
    let mut result = 0;

    let filename = "inputs/input1.txt";
    // let filename = "examples/example1.txt";
    let lines = fs::read_to_string(PathBuf::from(filename)).expect("Couln't read file");

    for line in lines.split("\n") {
        if line.trim().is_empty() {
            continue; // skip empty lines
        }

        let (command, number) = line.split_at(1);

        let number: i32 = number.trim().parse().unwrap_or(0);

        match command {
            "R" => {
                for _ in 0..number {
                    pointing_at += 1;
                    pointing_at %= 100;
                    if pointing_at == 0 {
                        result += 1;
                    }
                }
            }
            "L" => {
                for _ in 0..number {
                    pointing_at -= 1;
                    pointing_at %= 100;
                    if pointing_at == 0 {
                        result += 1;
                    }
                }
            }
            _ => println!("Couldn't parse line: {}", line),
        }

        // while pointing_at >= 100 {
        //     pointing_at -= 100;
        //     result += 1;
        // }
        //
        // while pointing_at < 0 {
        //     pointing_at += 100;
        //     result += 1;
        // }

        // println!("{}  \t{}  \t{}", line, pointing_at, result);
        // sleep(Duration::from_secs(2));
    }

    println!("Pointing at: {}", pointing_at);
    println!("Result: {}", result);
}
