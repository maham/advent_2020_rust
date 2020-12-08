use std::borrow::Borrow;

fn get_int(data: &str, separator: char) -> (u32, &str) {
    if let Some(pos) = data.find(separator) {
        let (num, rest_s) = data.split_at(pos);
        let num: u32 = num.parse().expect("Failed to parse int from string");
        return (num, rest_s[1..].borrow());
    } else {
        panic!();
    }
}

fn main() {
    let test_string = "1-2 l: asdfzxcv";

    let (min, rest) = get_int(test_string, '-');
    let (max, rest) = get_int(rest, ' ');
    let letter = rest.chars().next().unwrap();
    let password = rest[2..].borrow();
    print!("min: {}, max: {}, letter: {}, password: {}", min, max, letter, password);

    // let (min, rest) = test_string.split_at(test_string.find('-'));
    // print!("min: {}, rest: {}", min, rest);
    // let min: u32 = min.parse().unwrap();

    // let (max, rest) = test_string.split_at(*rest.find(' '));
    // print!("max: {}, rest: {}", min, rest);
    // let max: u32 = max.parse().unwrap();

    // let (letter, password) = test_string.split_at(*rest.find(':'));
    // print!("letter: {}, rest: {}", min, rest);
    // let letter: char = letter.parse().unwrap();
}