use std::process::exit;
use std::{env, fs};

// Binary search partition
fn bsp(low: i32, high: i32, mut codes: Vec<char>) -> i32 {
    if low == high || codes.is_empty() {
        return low;
    }
    let step = (high - low) / 2;
    return if codes.pop().unwrap() == '1' {
        bsp(low + step + 1, high, codes)
    } else {
        bsp(low, high - step - 1, codes)
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        exit(1);
    }

    let filename = args.get(1).unwrap();
    let lines = fs::read_to_string(filename)
        .expect("Something went wrong reading file")
        .split('\n')
        .map(str::to_string)
        .collect::<Vec<String>>();

    let mut ids = lines
        .iter()
        .map(|line| {
            let row: &str = &line[..8];
            let col: &str = &line[7..];
            (
                bsp(
                    0,
                    127,
                    row.chars()
                        .map(|c| if c == 'B' { '1' } else { '0' })
                        .rev()
                        .collect(),
                ),
                bsp(
                    0,
                    7,
                    col.chars()
                        .map(|c| if c == 'R' { '1' } else { '0' })
                        .rev()
                        .collect(),
                ),
            )
        })
        .map(|(r, c)| r * 8 + c)
        .collect::<Vec<i32>>();
    ids.sort();

    println!("Max id {}", ids[ids.len() - 1]);
    // Find where the range and the id's stop matching and subtract one to get the value
    // (id is surrounded by -1 and +1 of itself)
    println!(
        "Your seat id is {:?}",
        ids.iter()
            .zip(ids[0]..*(ids.iter().last().unwrap()))
            .find_map(|(&id, range)| if id != range { Some(id - 1) } else { None })
            .unwrap()
    );
}
