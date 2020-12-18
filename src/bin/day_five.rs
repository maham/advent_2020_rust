use std::{fs, env};
use itertools::Itertools;
use time::OffsetDateTime;

fn parse_row(s: &str) -> u32 {
    let mut row = 0;

    for (i, c) in s.chars().enumerate() {
        if let 'B' = c { row += 64 >> i }
    }

    row
}

fn parse_col(s: &str) -> u32 {
    let mut col = 0;
    for (i, c) in s.chars().enumerate() {
        if let 'R' = c { col += 4 >> i }
    }

    col
}

fn get_pass_id(s: &str) -> usize {
    let row = parse_row(&s[..7]) as usize;
    let col = parse_col(&s[7..]) as usize;

    row * 8 + col
}

fn first(boarding_passes: &[&str]) {
    let mut highest = 0;
    for pass in boarding_passes {
        let id = get_pass_id(pass);
        if id > highest {
            highest = id;
        }
    }

    println!("First: Highest boarding pass id is {}", highest)
}

fn second(boarding_passes: &[&str]) {
    let ids: Vec<usize> = boarding_passes.iter().map(|x| get_pass_id(x) as usize).sorted().collect();
    let first_id = ids[0];

    for (i, &id) in ids.iter().enumerate() {
        if first_id + i != id {
            println!("Second: Your seat id is {}", first_id + i);
            break;
        }
    }
}

fn second2(boarding_passes: &[&str]) {
    let mut found = vec![false; 1024];
    for pass in boarding_passes {
        let id = get_pass_id(pass) as usize;
        found[id] = true;
    }
    let mut i = 0;
    while !found[i] { i += 1 }
    while found[i] { i += 1 }
    println!("Second2: Your seat id is {}", i);

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = fs::read_to_string(filename).expect("Failed to load boarding passes from file.");
    let boarding_passes: Vec<&str> = data.lines().collect();

    let before_first = OffsetDateTime::now_utc();
    first(&boarding_passes);
    let after_first = OffsetDateTime::now_utc();
    second(&boarding_passes);
    let after_second = OffsetDateTime::now_utc();
    second2(&boarding_passes);
    let after_second2 = OffsetDateTime::now_utc();

    println!("First time: {}ms", (after_first - before_first).as_seconds_f64() * 1000.0);
    println!("Second time: {}ms", (after_second - after_first).as_seconds_f64() * 1000.0);
    println!("Second2 time: {}ms", (after_second2 - after_second).as_seconds_f64() * 1000.0);
    println!("Total time: {}ms", (after_second2 - before_first).as_seconds_f64() * 1000.0);
}