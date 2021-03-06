use std::{fs, env};

static TARGET_VALUE: u32 = 2020;

fn load_expenses(filename: &str) -> Vec<u32> {
    let mut expenses: Vec<u32> = fs::read_to_string(filename)
        .expect("Heck.")
        .lines()
        .map(|x| x.parse().expect("Failed to parse expense as u32"))
        .collect();

    expenses.sort_unstable();

    expenses
}

fn find_two_combo(expenses: &[u32]) -> Option<(u32, u32)> {
    for first_index in 0..expenses.len()  {
        let first_value = expenses[first_index];
        let diff = TARGET_VALUE - first_value;

        if expenses[first_index+1..].binary_search(&diff).is_ok() {
            return Some((first_value, diff))
        }
    }

    None
}

fn first(filename: &str) {
    let expenses = load_expenses(filename);

    if let Some((first_value, second_value)) = find_two_combo(&expenses) {
        let product = first_value * second_value;
        println!("{} * {} = {}", first_value, second_value, product);
    } else {
        println!("Didn't find a matching combo for first task.")
    }
}

fn find_three_combo(expenses: &[u32]) -> Option<(u32, u32, u32)> {
    for first_index in 0..expenses.len() {
        let first_value = expenses[first_index];

        for second_index in 0..expenses[first_index+1..].len() {
            let second_value = expenses[second_index];
            let sum = first_value + second_value;
            if sum >= TARGET_VALUE {
                continue;
            }

            let diff = TARGET_VALUE - sum;
            if expenses[first_index + 1 ..].binary_search(&diff).is_ok() {
                return Some((first_value, second_value, diff));
            }
        }
    }

    None
}

fn second(filename: &str) {
    let expenses = load_expenses(filename);

    if let Some((first_value, second_value, third_value)) = find_three_combo(&expenses) {
        let product = first_value * second_value * third_value;
        println!("{} * {} * {} = {}", first_value, second_value, third_value, product)
    } else {
        println!("Didn't find a matching combo for second task.")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    first(filename);
    second(filename);
}
