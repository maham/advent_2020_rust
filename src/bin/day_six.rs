use std::{env, fs};
use itertools::Itertools;
use time::OffsetDateTime;

fn first(data: &str) {
    let group_answers: Vec<String> = data.split("\r\n\r\n")
        .map(|x| x.chars().unique().filter(|z| z.is_alphabetic() ).collect())
        .collect();
    let answer_counts: u32 = group_answers.iter().map(|x| x.len() as u32).sum();
    println!("Total answers: {}", answer_counts);
}

fn second(data: &str) {
    let mut sum = 0;
    for group in data.split("\r\n\r\n") {
        let mut counter = 0;
        let num_persons = group.lines().count();
        let unique_answers: String = group.chars().filter(|x| x.is_alphabetic()).unique().collect();
        for c in unique_answers.chars() {
            if group.chars().filter(|x| *x == c).count() >= num_persons {
                counter += 1;
            }
        }
        sum += counter;
    }

    println!("Second: The answer is: {}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = fs::read_to_string(filename).expect("Failed to load boarding passes from file.");

    let before_first = OffsetDateTime::now_utc();
    first(&data);
    let after_first = OffsetDateTime::now_utc();
    second(&data);
    let after_second = OffsetDateTime::now_utc();

    println!("First time: {}ms", (after_first - before_first).as_seconds_f64() * 1000.0);
    println!("Second time: {}ms", (after_second - after_first).as_seconds_f64() * 1000.0);
    println!("Total time: {}ms", (after_second - before_first).as_seconds_f64() * 1000.0);
}

