use std::{fs, env};
use std::ops::AddAssign;

struct Size {
    width: u32,
    height: u32,
}

struct Map {
    data: Vec<bool>,
    size: Size,
}

struct Point {
    x: u32,
    y: u32,
}

impl AddAssign<&Size> for Point {
    fn add_assign(&mut self, other: &Size) {
        self.x += other.width;
        self.y += other.height;
    }
}

fn load_map(filename: &String) -> Map {
    let data = fs::read_to_string(filename).expect("Failed to load map from file.");
    let mut height = 0;
    let width = data
        .lines()
        .next()
        .expect("Failed to read first line from map data.")
        .chars()
        .count() as u32;

    let data = data
        .lines()
        .flat_map(|x| {height += 1; x.chars()})
        .map(|x| x == '#')
        .collect();

    Map{data, size: Size{ width, height}}
}

fn trees_for_slope(map: &Map, slope: &Size) -> u32 {
    let mut tree_count = 0;
    let mut position = Point { x: 0, y: 0 };

    while position.y < map.size.height {
        let index = (position.y * map.size.width + position.x % map.size.width) as usize;
        tree_count += if map.data[index] { 1 } else { 0 };
        position += slope;
    }

    tree_count
}

fn first(filename: &String) {
    let map = load_map(filename);
    let slope = Size{width: 3, height: 1};

    let tree_count = trees_for_slope(&map, &slope);

    println!("First: Found {} trees for slope ({}, {}).", tree_count, slope.width, slope.height);
}

fn second(filename: &String) {
    let map = load_map(filename);
    let slopes: Vec<Size> = vec![Size{ width: 1, height: 1},
                                 Size{ width: 2, height: 1},
                                 Size{ width: 4, height: 1},
                                 Size{ width: 6, height: 1},
                                 Size{ width: 0, height: 2}];
    let product: u64 = slopes.iter().map(|x| trees_for_slope(&map, x) as u64).product();

    println!("Second: The product of encountered trees for slopes is {}.", product)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    first(filename);
    second(filename);
}
