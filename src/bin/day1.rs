use std::collections::HashSet;
use std::process::exit;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        exit(1);
    }
    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).expect("Something is wrong with the file");
    let mut numbers: Vec<i32> = contents
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Not a number"))
        .collect();

    numbers.sort();
    let target_num = 2020;
    let mut sum_map = HashSet::<i32>::new();
    for num in numbers.iter() {
        let test_num = target_num - num;
        if sum_map.contains(&test_num) {
            println!(
                "Found answer for q1: {} * {} = {}",
                num,
                test_num,
                num * test_num
            );
        }
        sum_map.insert(*num);
    }

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            let test_num = target_num - numbers[i] - numbers[j];
            if sum_map.contains(&test_num) {
                println!(
                    "Found answer for q2: {} * {} * {} = {}",
                    numbers[i],
                    numbers[j],
                    test_num,
                    numbers[i] * numbers[j] * test_num
                );
            }
        }
    }
}
