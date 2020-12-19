use std::{fs, env};
use time::OffsetDateTime;
use std::cmp::{min, max};

fn first(data: &str) -> usize {
    let numbers: Vec<u64> = data.lines().map(|x| x.parse().expect("Failed to parse u64")).collect();

    let mut start = 0;
    while is_xmas_valid(&numbers[start..start+26]) {
        start += 1;
    }

    println!("The first invalid number is {}[{}]", numbers[start+25], start+25);

    start + 25
}

fn is_xmas_valid(numbers: &[u64]) -> bool {
    let num = numbers[numbers.len() - 1];

    for i in 0..25 {
        if numbers[i] >= num {
            continue
        }

        let diff = num - numbers[i];


        for j in i..25 {
            if j == i {
                continue
            }

            if numbers[j] == diff {
                return true
            }
        }
    }

    false
}

fn second(start_index: usize, data: &str) {
    let numbers: Vec<u64> = data.lines().map(|x| x.parse().expect("Failed to parse u64")).collect();

    let target = numbers[start_index];
    let mut start = start_index - 1;
    let mut end = start;

    loop {
        let sum: u64 = numbers[end..start].iter().sum();
        if sum == target {
            let mut lowest = target;
            let mut highest = 0;
            for num in numbers[end..start].iter() {
                lowest = min(lowest, *num);
                highest = max(highest, *num);
            }
            println!("The weakness is {}+{}={}", lowest, highest, lowest + highest);
            break
        } else if sum > target {
            start -= 1;
            end = min(start, end);
        } else {
            end -= 1
        }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = fs::read_to_string(filename).expect("Failed to load boot code.");

    let before_first = OffsetDateTime::now_utc();
    let invalid_index = first(&data);
    let after_first = OffsetDateTime::now_utc();
    second(invalid_index, &data);
    let after_second = OffsetDateTime::now_utc();

    println!("First time {}ms", (after_first - before_first).as_seconds_f64() * 1000.0);
    println!("Second time {}ms", (after_second - after_first).as_seconds_f64() * 1000.0);
    println!("Total time {}ms", (after_second - before_first).as_seconds_f64() * 1000.0);
}