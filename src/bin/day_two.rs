use std::str::FromStr;
use std::num::ParseIntError;
use std::borrow::Borrow;
use std::{fs, env};

struct PasswordRow {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}

impl FromStr for PasswordRow {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, rest) = get_int(s, '-');
        let (max, rest) = get_int(rest, ' ');
        let letter = rest.chars().next().unwrap();
        let password = rest[2..].to_string();

        Ok(PasswordRow {min, max, letter, password})
    }
}

fn get_int(data: &str, separator: char) -> (u32, &str) {
    if let Some(pos) = data.find(separator) {
        let (num, rest_s) = data.split_at(pos);
        let num: u32 = num.parse().expect("Failed to parse int from string");
        return (num, rest_s[1..].borrow());
    } else {
        panic!();
    }
}

fn load_passwords(filename: &str) -> Vec<PasswordRow> {
    fs::read_to_string(filename)
        .expect("Failed to load passwords from file.")
        .lines()
        .map(|x| x.parse().expect("Failed to parse password row."))
        .collect()
}

fn test_password_first(password: &PasswordRow) -> bool {
    let mut counter = 0;
    for l in password.password.chars() {
        if l == password.letter {
            counter += 1;
        }
    }

    password.min <= counter && counter <= password.max
}

fn first(filename: &str) {
    let passwords = load_passwords(filename);

    let num_valid_passwords: usize = passwords
        .iter()
        .filter(|x| test_password_first(x))
        .count();

    println!("Found {} valid passwords on first search.", num_valid_passwords);
}

fn test_password_second(p: &PasswordRow) -> bool {
    let mut matches = 0;
    for (i, l) in p.password.char_indices() {
        if (i == p.min as usize || i == p.max as usize) && l == p.letter {
            matches += 1;
        }
    }

    matches == 1
}

fn second(filename: &str) {
    let passwords = load_passwords(filename);

    let num_valid_passwords: usize = passwords
        .iter()
        .filter(|x| test_password_second(x))
        .count();

    println!("Found {} valid passwords on second search.", num_valid_passwords);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    first(filename);
    second(filename);
}