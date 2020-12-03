use std::process::exit;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        exit(1);
    }

    let filename = args.get(1).unwrap();
    let contents =
        fs::read_to_string(filename).expect("Something went wrong with reading the file");

    // Get each line
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    // Slice our lines up
    let splits: Vec<(i32, i32, char, String)> = lines
        .iter()
        .map(|line| {
            let splits: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
            let bounds: Vec<i32> = splits[0]
                .split('-')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            let (lower_bound, upper_bound) = (bounds[0], bounds[1]);
            let check_char = splits[1].chars().nth(0).unwrap();
            (lower_bound, upper_bound, check_char, splits[2].clone())
        })
        .collect();

    // Find the number of valid passwords for each test type
    let task1_valid_count: i32 = splits
        .iter()
        .map(|x| is_valid_pw1(x.0, x.1, x.2, x.3.as_str()))
        .sum();

    let task2_valid_count: i32 = splits
        .iter()
        .map(|x| is_valid_pw2(x.0, x.1, x.2, x.3.as_str()))
        .sum();

    println!("Task 1 answer: {}", task1_valid_count);
    println!("Task 2 answer: {}", task2_valid_count);
}

fn is_valid_pw2(lower: i32, upper: i32, check_char: char, pw: &str) -> i32 {
    let low_check = pw.chars().nth(lower as usize - 1).unwrap() == check_char;
    let upper_check = pw.chars().nth(upper as usize - 1).unwrap() == check_char;
    if (low_check || upper_check) && !(low_check && upper_check) {
        1
    } else {
        0
    }
}

fn is_valid_pw1(lower: i32, upper: i32, check_char: char, pw: &str) -> i32 {
    let char_count: i32 = pw
        .chars()
        .map(|c| if c == check_char { 1 } else { 0 })
        .sum();
    if char_count >= lower && char_count <= upper {
        1
    } else {
        0
    }
}
