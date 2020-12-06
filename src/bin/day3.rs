use std::process::exit;
use std::{env, fs};

fn iterative_slope(right: usize, down: usize, hill: &Vec<String>) -> usize {
    let mut str_idx = 0;
    let mut count = 0;
    for i in (0..hill.len()).step_by(down).skip(1) {
        str_idx = (str_idx + right) % hill[0].len();
        if hill[i].chars().nth(str_idx).unwrap() == '#' {
            count += 1;
        }
    }
    count
}

#[allow(dead_code)]
fn functional_version(step_w: usize, step_h: usize, hill: &Vec<String>) -> usize {
    hill.iter()
        .step_by(step_h)
        .zip((0..hill[0].len()).cycle().step_by(step_w))
        .skip(1)
        .filter(|&(row, c)| row.chars().nth(c).unwrap() == '#')
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        exit(1);
    }

    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading file");
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    let mut prod = 1;
    for i in (1..=7).step_by(2) {
        prod *= iterative_slope(i, 1, &lines);
    }
    prod *= iterative_slope(1, 2, &lines);
    println!("You hit {} trees", prod);
}
