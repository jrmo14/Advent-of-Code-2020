use std::collections::HashMap;
use std::process::exit;
use std::{env, fs};

struct PassportBuilder {
    data: String,
}

impl PassportBuilder {
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn vaild_passport(&self) -> (bool, bool) {
        let keys: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let mut passport = HashMap::<String, String>::new();

        let mut valid_keys = true;

        for (key, value) in self
            .data
            .split_whitespace()
            .map(|kp| {
                let kp_split: Vec<&str> = kp.split(":").collect();
                (kp_split[0].to_string(), kp_split[1].to_string())
            })
            .collect::<Vec<(String, String)>>()
        {
            passport.insert(key.clone(), value.clone());
            if !Self::validate_key_val(&*key, &*value) {
                valid_keys = false;
            }
        }
        let all_there = keys.iter().all(|key| passport.contains_key(*key));
        (all_there, valid_keys && all_there)
    }

    fn validate_key_val(key: &str, value: &str) -> bool {
        let parse_bound_num = |v: &str, low: i32, high: i32| -> bool {
            match v.parse::<i32>() {
                Ok(v) => {
                    if v < low || v > high {
                        false
                    } else {
                        true
                    }
                }
                Err(_) => false,
            }
        };
        match key {
            "byr" => {
                if !parse_bound_num(&value, 1920, 2002) {
                    return false;
                }
            }
            "iyr" => {
                if !parse_bound_num(&value, 2010, 2020) {
                    return false;
                }
            }
            "eyr" => {
                if !parse_bound_num(&value, 2020, 2030) {
                    return false;
                }
            }
            "hgt" => {
                if value.ends_with("cm") {
                    if !parse_bound_num(&value[..value.len() - 2], 150, 193) {
                        return false;
                    }
                } else if value.ends_with("in") {
                    if !parse_bound_num(&value[..value.len() - 2], 59, 76) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            "hcl" => {
                if value.len() != 7 {
                    return false;
                }
                if value.chars().nth(0).unwrap() != '#' {
                    return false;
                }
                if !value[1..]
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_numeric())
                {
                    return false;
                }
            }
            "ecl" => {
                let colors: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                if !colors.contains(&&*value) {
                    return false;
                }
            }
            "pid" => {
                if value.len() != 9 || !value.chars().all(char::is_numeric) {
                    return false;
                }
            }
            _ => {}
        }
        true
    }

    pub fn build(&mut self, new_data: &str) {
        self.data.push(' ');
        self.data.push_str(new_data);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        exit(1);
    }

    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading file");
    let mut builder = PassportBuilder::new();

    let mut base_count = 0;
    let mut valid_key_count = 0;
    for line in contents
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    {
        if line.is_empty() {
            let (a, b) = builder.vaild_passport();
            if a {
                base_count += 1
            }
            if b {
                valid_key_count += 1;
            }
            builder.clear()
        } else {
            builder.build(line.as_str())
        }
    }
    let (a, b) = builder.vaild_passport();
    if a {
        base_count += 1
    }
    if b {
        valid_key_count += 1;
    }
    println!("Counted {} passports the proper fields", base_count);
    println!("Counted {} valid passports", valid_key_count);
}
