use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::process::exit;

fn part1(bags: &mut HashMap<String, Vec<String>>) -> usize {
    let mut to_visit = Vec::new();
    let mut visited = HashSet::new();

    for bag in bags.get("shinygold").unwrap() {
        to_visit.push(bag.clone());
    }
    let mut container_count: usize = 0;
    visited.insert("shinygold".to_string());
    while !to_visit.is_empty() {
        let cur = to_visit.pop().unwrap();
        if visited.contains(&*cur) {
            continue;
        }
        for x in bags.get(&*cur).unwrap_or(&Vec::<String>::new()) {
            if !visited.contains(&*x.to_string()) {
                to_visit.push(x.clone());
            }
        }
        container_count += 1;
        visited.insert(cur.to_string());
    }
    container_count
}

fn part2(
    bag: &str,
    bags: &HashMap<String, Vec<(usize, String)>>,
    ans: &mut HashMap<String, usize>,
) -> usize {
    let mut count = 1;
    for child in &bags[bag] {
        if ans.contains_key(child.1.as_str()) {
            count += child.0 * ans.get(child.1.as_str()).unwrap();
        } else {
            let tmp = part2(child.1.as_str(), bags, ans);
            ans.insert(child.clone().1, tmp);
            count += child.0 * tmp;
        }
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        exit(1);
    }

    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).expect("Something's wrong");

    let mut reverse_container_order_bags = HashMap::new();
    let mut forward_order_bags = HashMap::new();

    for line in contents.split('\n') {
        let mut splits = line.split("bags contain").map(str::trim);
        let outer_bag = splits.next().unwrap().replace(" ", "");
        let tmp = splits.next().unwrap();
        let cleanup = |s: &str| -> (usize, String) {
            let tmp = s
                .trim()
                .replace('.', "")
                .replace(" bags", "")
                .replace(" bag", "");
            let mut data = tmp.split(' ');
            let count: usize = data.next().unwrap().parse().unwrap_or(0);
            let bag = data.collect::<String>();
            (count, bag)
        };
        let inner_bags = if tmp.contains(",") {
            tmp.split(",")
                .map(cleanup)
                .collect::<Vec<(usize, String)>>()
        } else {
            let clean = cleanup(tmp);
            if clean.0 == 0 {
                Vec::new()
            } else {
                vec![clean]
            }
        };

        for inner_bag in &inner_bags {
            if !reverse_container_order_bags.contains_key(&inner_bag.1) {
                reverse_container_order_bags.insert(inner_bag.1.clone(), Vec::new());
            }
            reverse_container_order_bags
                .get_mut(&inner_bag.1)
                .unwrap()
                .push(outer_bag.clone());
        }
        forward_order_bags.insert(outer_bag, inner_bags);
    }
    println!("Part 1: {}", part1(&mut reverse_container_order_bags));

    // This counts the shiny gold bag as well, so get rid of that
    let mut ans = HashMap::new();
    println!(
        "Part 2: {}",
        part2("shinygold", &forward_order_bags, &mut ans) - 1
    )
}
