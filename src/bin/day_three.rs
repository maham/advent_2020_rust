use std::{fs, env};

static JUMP_SIZE: (u32, u32) = (3, 1);

fn load_map(filename: &String) -> ((u32, u32), Vec<bool>) {
    let data = fs::read_to_string(filename).expect("Failed to load map from file.");
    let mut height = 0;
    let width = data
        .lines()
        .next()
        .expect("Failed to read first line from map data.")
        .chars()
        .count() as u32;

    let map = data
        .lines()
        .flat_map(|x| {height += 1; x.chars()})
        .map(|x| x == '#')
        .collect();

    ((width, height), map)
}

// fn display_map(map: &Vec<bool>, width: u32) {
//     let mut counter = 0;
//     for current in map.iter() {
//         if counter >= width {
//             println!();
//             counter = 0;
//         }
//
//         print!("{}", if *current {'#'} else {'.'});
//         counter += 1;
//     }
// }

fn trees_for_slope(map: &Vec<bool>, map_size: &(u32, u32), slope: &(u32, u32)) -> u32 {
    let mut tree_count = 0;
    let mut position: (u32, u32) = (0, 0);

    while position.1 < map_size.1 {
        let index = (position.1 * map_size.0 + position.0 % map_size.0) as usize;
        tree_count += if map[index] { 1 } else { 0 };
        position.0 += slope.0;
        position.1 += slope.1;
    }

    tree_count
}

fn first(filename: &String) {
    let (size, map) = load_map(filename);

    let tree_count = trees_for_slope(&map, &size, &JUMP_SIZE);

    println!("First: Found {} trees for slope ({}, {}).", tree_count, JUMP_SIZE.0, JUMP_SIZE.1);
}

fn second(filename: &String) {
    let (size, map) = load_map(filename);
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let tree_counts: Vec<u64> = slopes.iter().map(|x| trees_for_slope(&map, &size, x) as u64).collect();
    let product = tree_counts.iter().fold(1, |acc, x| acc * x);

    println!("Second: The product of encountered trees for slopes is {}.", product)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    first(filename);
    second(filename);
}
