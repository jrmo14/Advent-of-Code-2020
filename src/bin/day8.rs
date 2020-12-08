use std::collections::HashSet;
use std::process::exit;
use std::time::Instant;
use std::{env, fs};

type Instruction<'a> = (&'a str, i32);

// Returns true if the program exits, as well as the value of acc
// Returns false if the program loops, as well as the value of acc before the loop starts
fn will_exit(instructions: &Vec<Instruction>) -> (bool, i32) {
    let mut acc = 0;
    let mut reg: i32 = 0;
    let mut visited = HashSet::new();
    loop {
        if visited.contains(&reg) {
            break;
        }
        visited.insert(reg);
        match instructions.get(reg as usize) {
            Some(instruction) => {
                println!("{:?}", instruction);
                match instruction.0 {
                    "jmp" => reg += instruction.1,
                    "acc" => {
                        acc += instruction.1;
                        reg += 1;
                    }
                    _ => reg += 1,
                };
            }
            None => {
                return (true, acc);
            }
        }
    }
    (false, acc)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        exit(1);
    }

    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).expect("Something's wrong");

    let mut instructions: Vec<(&str, i32)> = contents
        .split('\n')
        .map(|line| -> Instruction {
            let mut split = line.split(' ');
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap_or(0),
            )
        })
        .collect();

    println!("Part 1: {}", will_exit(&instructions).1);
    for i in 0..instructions.len() {
        match instructions.get(i).unwrap().0 {
            "jmp" => {
                instructions.get_mut(i).unwrap().0 = "nop";
            }
            "nop" => {
                instructions.get_mut(i).unwrap().0 = "jmp";
            }
            _ => {
                continue;
            }
        }
        let res = will_exit(&instructions);
        if res.0 {
            println!("Part 2: {}, answer found at {}", res.1, i);
            break;
        }
        // Reset and keep going
        match instructions.get(i).unwrap().0 {
            "jmp" => {
                instructions.get_mut(i).unwrap().0 = "nop";
            }
            "nop" => {
                instructions.get_mut(i).unwrap().0 = "jmp";
            }
            _ => {
                continue;
            }
        }
    }
}
