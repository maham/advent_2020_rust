use std::{fs, env};
use itertools::Itertools;
use time::OffsetDateTime;
use std::collections::HashMap;

struct Bag<'a> {
    color: &'a str,
    capacity: Vec<(usize, usize)>
}

impl<'a> Bag<'a> {
    fn new(data: &'a str, bag_ids: &HashMap<&str, usize>) -> Self {
        let color = extract_bag_color(data);
        let mut capacity: Vec<(usize, usize)> = Vec::new();
        let mut it = data.match_indices(char::is_numeric).map(|(i, _)| i);

        if let Some(mut start) = it.next() {
            for next in it {
                let cap_color = &data[start..next];
                let cap = extract_bag_capacity(cap_color, bag_ids);

                capacity.push(cap);
                start = next;
            }
            let cap_color = &data[start..data.len()];
            let cap = extract_bag_capacity(cap_color, bag_ids);

            capacity.push(cap);
        }

        Bag{color, capacity}
    }
}

fn first(data: &str) {
    let mut counter: u32 = 0;
    let found = count_bag_color("shiny gold", &data, &mut counter);
    println!("Bags holding shiny gold bags: {}, {} iterations", found.iter().unique().count(), counter);
}

fn extract_bag_capacity<'a>(bag_data: &'a str, bag_ids: &HashMap<&'a str, usize>) -> (usize, usize) {
    let i = bag_data.find(' ').expect("Failed to extract amount from bag_data.");
    let amount: usize = bag_data[..i].parse().expect("Failed to parse bag capacity amount");
    let j = bag_data.find("bag").expect("Failed to extract bag color from bag_data");
    let name = &bag_data[i..j];

    (amount, bag_ids[&name.trim()])
}

fn first2(data: &str) {
    let bag_lines: Vec<&str> = data.lines().collect();
    let bag_indices: HashMap<&str, usize> = bag_lines.iter().enumerate().map(|(i, x)| (extract_bag_color(x), i)).collect();
    let bags: Vec<Bag> = bag_lines.iter().map(|x| Bag::new(x, &bag_indices)).collect();
    let mut counter: u32 = 0;
    let found_bags = count_bag_color2(bag_indices["shiny gold"], &bag_indices, &bags, &mut counter).iter().unique().count();

    println!("2: Bags holding shiny gold bags: {}, {} iterations", found_bags, counter);

}

fn count_bag_color2(color: usize, ids: &HashMap<&str, usize>, bags: &[Bag], counter: &mut u32) -> Vec<usize> {
    let mut colors: Vec<usize> = Vec::new();
    for bag in bags {
        for c in bag.capacity.iter() {
            if c.1 == color {
                colors.push(ids[bag.color]);
                *counter += 1;
                colors.extend(count_bag_color2(ids[bag.color], ids, bags, counter));
            }
        }
    }

    colors
}

fn second(data: &str) {
    let bag_lines: Vec<&str> = data.lines().collect();
    let bag_indices: HashMap<&str, usize> = bag_lines.iter().enumerate().map(|(i, x)| (extract_bag_color(x), i)).collect();
    let bags: Vec<Bag> = bag_lines.iter().map(|x| Bag::new(x, &bag_indices)).collect();
    let mut iterations: u32 = 0;
    let required_bags = count_required_bags(bag_indices["shiny gold"], &bags, &mut iterations) - 1;

    println!("Second: {} bags are required, {} iterations", required_bags, iterations);
}

fn count_required_bags(bag_id: usize, bags: &[Bag], iterations: &mut u32) -> usize {
    let mut counter = 1;
    for c in bags[bag_id].capacity.iter() {
        *iterations += 1;
        let sub_count = count_required_bags(c.1, bags, iterations);
        counter += c.0 * sub_count;
    }

    counter
}

fn extract_bag_color(line: &str) -> &str {
    let (i, _) = line.match_indices(char::is_whitespace).nth(1).expect("Failed to extract color from line.");
    &line[..i]
}

fn count_bag_color<'a>(color: &'a str, data: &'a str, counter: &mut u32) -> Vec<&'a str> {
    let mut colors: Vec<&str> = Vec::new();
    for line in data.lines() {
        if let Some(i) = line.find(color) {
            if i == 0 {
                continue
            }

            let bag_color = extract_bag_color(line);
            colors.push(bag_color);
            *counter += 1;
            colors.extend(count_bag_color(bag_color, data, counter));
        }
    }

    colors
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = fs::read_to_string(filename).expect("Failed to load bag rules.");


    let before_first = OffsetDateTime::now_utc();
    first(&data);
    let after_first = OffsetDateTime::now_utc();
    first2(&data);
    let after_first2 = OffsetDateTime::now_utc();
    second(&data);
    let after_second = OffsetDateTime::now_utc();

    println!("First time: {}ms", (after_first - before_first).as_seconds_f64() * 1000.0);
    println!("First2 time: {}ms", (after_first2 - after_first).as_seconds_f64() * 1000.0);
    println!("Second time: {}ms", (after_second - after_first2).as_seconds_f64() * 1000.0);
    println!("Total time: {}ms", (after_second - before_first).as_seconds_f64() * 1000.0);
}