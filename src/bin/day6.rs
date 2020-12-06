use std::collections::{HashMap, HashSet};
use std::process::exit;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        exit(1);
    }

    let filename = args.get(1).unwrap();
    let groups = fs::read_to_string(filename)
        .expect("Something went wrong reading file")
        .split("\n\n")
        .map(str::to_string)
        .collect::<Vec<String>>();

    let sums = groups
        .iter()
        .map(|line| {
            let mut set = HashSet::new();
            line.chars().filter(|c| c.is_alphanumeric()).for_each(|c| {
                set.insert(c);
            });
            set.len()
        })
        .collect::<Vec<usize>>();
    println!("Part 1 {}", sums.iter().sum::<usize>());
    let sums: usize = groups
        .iter()
        .map(|group: &String| {
            let num_people = group
                .split("\n")
                .map(str::to_string)
                .collect::<Vec<String>>()
                .len();
            let mut question_counts = HashMap::<char, usize>::new();
            group.chars().filter(|c| c.is_alphanumeric()).for_each(|c| {
                if !question_counts.contains_key(&c) {
                    question_counts.insert(c, 1);
                } else {
                    question_counts.insert(c, question_counts.get(&c).unwrap() + 1);
                }
            });
            question_counts
                .into_iter()
                .filter_map(|(k, v)| {
                    if v == num_people as usize {
                        Some(1)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("Part 2 {}", sums);
}
