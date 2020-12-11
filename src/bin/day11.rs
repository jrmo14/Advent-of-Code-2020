use std::process::exit;
use std::{env, fs};

const DELTAS: &[(i32, i32); 8] = &[
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn valid_coord(coord: (i32, i32), dim: (i32, i32)) -> bool {
    let in_range = |num: i32, upper: i32, lower: i32| -> bool { num >= lower && num < upper };
    in_range(coord.0, dim.0, 0) && in_range(coord.1, dim.1, 0)
}

fn count_surrounding_part1(seats: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> i32 {
    let mut count = 0;
    for delta in DELTAS {
        if valid_coord(
            (delta.0 + row_idx as i32, delta.1 + col_idx as i32),
            (seats.len() as i32, seats[0].len() as i32),
        ) {
            if seats[(row_idx as i32 + delta.0) as usize][(col_idx as i32 + delta.1) as usize]
                == '#'
            {
                count += 1;
            }
        }
    }
    count
}

fn step_simulation_part1(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_seats = Vec::with_capacity(seats.len());
    for row_idx in 0..seats.len() {
        new_seats.push(Vec::with_capacity(seats[0].len()));
        for col_idx in 0..seats[0].len() {
            match seats[row_idx][col_idx] {
                'L' => {
                    if count_surrounding_part1(seats, row_idx, col_idx) == 0 {
                        new_seats[row_idx].push('#');
                    } else {
                        new_seats[row_idx].push('L');
                    }
                }
                '#' => {
                    if count_surrounding_part1(seats, row_idx, col_idx) >= 4 {
                        new_seats[row_idx].push('L');
                    } else {
                        new_seats[row_idx].push('#');
                    }
                }
                _ => new_seats[row_idx].push('.'),
            }
        }
    }
    new_seats
}

fn count_surrounding_part2(seats: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> i32 {
    let mut count = 0;

    for delta in DELTAS {
        for step in 1..seats.len() {
            let new_coord = (
                delta.0 * (step as i32) + row_idx as i32,
                delta.1 * (step as i32) + col_idx as i32,
            );
            if valid_coord(new_coord, (seats.len() as i32, seats[0].len() as i32)) {
                match seats[new_coord.0 as usize][new_coord.1 as usize] {
                    'L' => {
                        break;
                    }
                    '#' => {
                        count += 1;
                        break;
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }
    }
    count
}

fn step_simulation_part2(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_seats = Vec::with_capacity(seats.len());
    for row_idx in 0..seats.len() {
        new_seats.push(Vec::with_capacity(seats[0].len()));
        for col_idx in 0..seats[0].len() {
            match seats[row_idx][col_idx] {
                'L' => {
                    if count_surrounding_part2(seats, row_idx, col_idx) == 0 {
                        new_seats[row_idx].push('#');
                    } else {
                        new_seats[row_idx].push('L');
                    }
                }
                '#' => {
                    if count_surrounding_part2(seats, row_idx, col_idx) >= 5 {
                        new_seats[row_idx].push('L');
                    } else {
                        new_seats[row_idx].push('#');
                    }
                }
                _ => {
                    new_seats[row_idx].push('.');
                }
            }
        }
    }
    new_seats
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        exit(1);
    }

    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).expect("Something's wrong");

    let original_seats: Vec<Vec<char>> =
        contents.split("\n").map(|s| s.chars().collect()).collect();
    let check_stable = |seats: &Vec<Vec<char>>, new_seats: &Vec<Vec<char>>| -> bool {
        seats
            .iter()
            .zip(new_seats.iter())
            .map(|(cur_row, new_row)| {
                cur_row
                    .iter()
                    .zip(new_row.iter())
                    .all(|(cur_seat, new_seat)| *cur_seat == *new_seat)
            })
            .all(|res| res)
    };
    let mut seats = original_seats.clone();

    loop {
        let new_seats = step_simulation_part1(&seats);
        if check_stable(&seats, &new_seats) {
            break;
        }
        seats = new_seats;
    }
    let _print_seats = |to_print: &Vec<Vec<char>>| {
        println!(
            "{}",
            to_print
                .iter()
                .map(|row| {
                    let mut row_str = row.iter().map(|c| *c as u8).collect::<Vec<u8>>();
                    row_str.push('\n' as u8);
                    String::from_utf8(row_str).unwrap()
                })
                .collect::<String>()
        );
    };

    let count_occupied = |to_count: Vec<Vec<char>>| {
        to_count
            .iter()
            .map(|row| row.iter().filter(|seat| **seat == '#').count())
            .sum::<usize>()
    };

    // print_seats(&seats);
    println!("Part 1: {}", count_occupied(seats));

    // Part 2
    let mut seats = original_seats.clone();
    loop {
        let new_seats = step_simulation_part2(&seats);
        if check_stable(&seats, &new_seats) {
            break;
        }
        seats = new_seats;
    }
    // print_seats(&seats);
    println!("Part 2: {}", count_occupied(seats));
}
