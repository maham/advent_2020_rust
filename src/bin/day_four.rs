use std::{fs, env};
use itertools::Itertools;
use std::collections::HashMap;

fn first(passports: &[HashMap<&str,&str>]) {
    let mut counter = 0;
    for passport in passports {
        let num_fields = passport.len();
        if num_fields >= 7 {
            counter += 1;
        }
    }

    println!("First: There are {} valid passports", counter);
}

fn is_byr_valid(birth_year: u32) -> bool {
    birth_year >= 1920 && birth_year <= 2002
}

fn is_iyr_valid(issue_year: u32) -> bool {
    issue_year >= 2010 && issue_year <= 2020
}

fn is_eyr_valid(expiration_year: u32) -> bool {
    expiration_year >= 2020 && expiration_year <= 2030
}

fn is_hgt_valid(height: &str) -> bool {
    let hgt: u32;

    if let Ok(x) = height[..height.len() - 2].parse() {
        hgt = x;
    } else {
        return false;
    }

    match &height[height.len() - 2 ..] {
        "in" => {
            hgt >= 59 && hgt <= 76
        },
        "cm" => {
            hgt >= 150 && hgt <= 193
        },
        _ => false
    }
}

fn is_hcl_valid(hair_color: &str) -> bool {
    if hair_color.len() != 7 {
        return false
    }

    if !hair_color.starts_with('#') {
        return false
    }

    if !hair_color[1..].chars().all(char::is_alphanumeric) {
        return false
    }

    true
}

fn is_ecl_valid(eye_color: &str) -> bool {
    matches!(eye_color, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn is_pid_valid(passport_id: &str) -> bool {
    if passport_id.len() != 9 {
        return false
    }

    if !passport_id.chars().all(char::is_numeric) {
        return false
    }

    true
}

fn second(passports: &[HashMap<&str, &str>]) {
    let mut counter = 0;

    for passport in passports {
        if passport.len() < 7 {
            continue;
        }

        match passport["byr"].parse() {
            Err(_) => continue,
            Ok(byr) => if !is_byr_valid(byr) { continue }
        }

        match passport["iyr"].parse() {
            Err(_) => continue,
            Ok(iyr) => if !is_iyr_valid(iyr) { continue }
        }

        match passport["eyr"].parse() {
            Err(_) => continue,
            Ok(eyr) => if !is_eyr_valid(eyr) { continue }
        }

        if !is_hgt_valid(passport["hgt"]) {
            continue
        }

        if !is_hcl_valid(passport["hcl"]) {
            continue
        }

        if !is_ecl_valid(passport["ecl"]) {
            continue
        }

        if !is_pid_valid(passport["pid"]) {
            continue
        }

        counter += 1;
    }

    println!("Second: There are {} valid passports", counter);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = fs::read_to_string(filename).expect("Failed to load passports from file.");
    let passports: Vec<HashMap<_,_>> = data
        .split("\r\n\r\n")
        .map(|x| {
            x.split_whitespace()
                .filter(|y| !y.trim().starts_with("cid"))
                .flat_map(|z| z.split(':').tuples()
                ).collect()
        })
        .collect();

    first(&passports);
    second(&passports);
}
