use std::collections::HashMap;
use std::process::exit;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        exit(1);
    }

    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).expect("Something's wrong");

    let numbers: Vec<i32> = contents
        .split("\n")
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    let preamble_size = 25;
    let mut preamble: HashMap<i32, i32> =
        numbers[0..preamble_size].iter().map(|n| (*n, 1)).collect();

    let codes = numbers[preamble_size..].to_vec();

    let mut vuln_num = 0;
    for i in 0..codes.len() {
        let mut sums_to_num = false;
        for check_num in &preamble {
            if preamble.contains_key(&(codes[i] - check_num.0)) {
                sums_to_num = true;
            }
        }
        if !sums_to_num {
            println!("Part 1: {}", codes[i]);
            vuln_num = codes[i];
            break;
        }
        let remove_num = numbers[i];
        let ins_num = numbers[i + preamble_size];

        preamble.insert(remove_num, preamble.get(&remove_num).unwrap_or(&0) - 1);
        preamble = preamble.into_iter().filter(|(_, v)| v > &0).collect();
        preamble.insert(ins_num, preamble.get(&ins_num).unwrap_or(&0) + 1);
    }
    for i in 0..numbers.len() - 1 {
        let mut sum = numbers[i];
        let mut found_answer = false;
        for j in i + 1..numbers.len() {
            sum += numbers[j];
            if sum == vuln_num {
                found_answer = true;
                let ans_range: Vec<i32> = (i..=j).map(|n| numbers[n]).collect();
                let min = ans_range.iter().min().unwrap();
                let max = ans_range.iter().max().unwrap();
                println!("Part 2: {} + {} = {}", min, max, min + max);
                break;
            }
            if sum > vuln_num {
                break;
            }
        }
        if found_answer {
            break;
        }
    }
}
