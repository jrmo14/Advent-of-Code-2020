use std::collections::HashMap;
use std::process::exit;
use std::{env, fs};

fn count_combos(cur_val: i32, numbers: &[i32], cache: &mut HashMap<i32, usize>) -> usize {
    if numbers.len() == 1 {
        return 1;
    }
    if cache.contains_key(&cur_val) {
        return *cache.get(&cur_val).unwrap();
    }
    let mut result = 0;
    for i in 1..=3 {
        if numbers.contains(&(cur_val + i)) {
            let mut idx = 0;
            for j in 0..numbers.len() {
                if numbers[j] == cur_val + i {
                    idx = j;
                    break;
                }
            }
            result += count_combos(cur_val + i, &numbers[idx..], cache);
        }
    }
    cache.insert(cur_val, result);
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        exit(1);
    }

    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).expect("Something's wrong");

    let mut numbers: Vec<i32> = contents
        .split("\n")
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    numbers.sort();
    let mut three_count = 1;
    let mut one_count = 1;
    (1..numbers.len())
        .map(|n| numbers[n] - numbers[n - 1])
        .for_each(|n| match n {
            3 => three_count += 1,
            1 => one_count += 1,
            _ => {}
        });

    println!(
        "Part 1: {} * {}  = {}",
        one_count,
        three_count,
        three_count * one_count
    );

    let mut cache = HashMap::new();
    println!(
        "Part 2: {}",
        count_combos(0, numbers.as_slice(), &mut cache)
    );
}
