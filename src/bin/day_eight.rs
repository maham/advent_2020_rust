use std::{fs, env};
use std::num::ParseIntError;
use std::str::FromStr;
use time::OffsetDateTime;

enum OpCode {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for OpCode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let value: i32 = parts[1].parse()?;
        let code = match parts[0] {
            "acc" => OpCode::Acc(value),
            "jmp" => OpCode::Jmp(value),
            "nop" => OpCode::Nop(value),
            _ => panic!("Unknown op-code encountered."),
        };

        Ok(code)
    }
}

fn first(data: &str) {
    let program: Vec<OpCode> = data.lines().map(|x| x.parse::<OpCode>().expect("Failed to parse op-code")).collect();
    let mut log = vec![false; program.len()];

    let mut instruction: i32 = 0;
    let mut accumulator: i32 = 0;
    loop {
        if log[instruction as usize] {
            break
        }

        log[instruction as usize] = true;

        match program[instruction as usize] {
            OpCode::Acc(val) => {
                accumulator += val;
                instruction += 1;
            },
            OpCode::Jmp(offset) => instruction += offset,
            OpCode::Nop(_) => instruction += 1,
        }
    }

    println!("First: Final accumulator value: {}", accumulator);
}

fn first2(data: &str) {
    let program: Vec<OpCode> = data.lines().map(|x| x.parse::<OpCode>().expect("Failed to parse op-code")).collect();
    let mut log = vec![false; program.len()];
    search_first2(&program, &mut log, 0, 0);
}

fn search_first2(program: &Vec<OpCode>, log: &mut Vec<bool>, inst: i32, acc: i32) -> i32 {
    if log[inst as usize] {
        return acc;
    }

    log[inst as usize] = true;

    return match program[inst as usize] {
        OpCode::Acc(val) => search_first2(program, log, inst + 1, acc + val),
        OpCode::Jmp(offset) => search_first2(program, log, inst + offset, acc),
        OpCode::Nop(_) => search_first2(program, log, inst + 1, acc),
    }
}

fn search_second(program: &Vec<OpCode>, log: &mut Vec<bool>, inst: i32, acc: i32) -> Option<i32> {
    if inst == program.len() as i32 {
        return Some(acc)
    } else if inst > program.len() as i32 || inst < 0 || log[inst as usize] {
        return None
    }

    let mut res: Option<i32>;

    match program[inst as usize] {
        OpCode::Acc(val) => {
            log[inst as usize] = true;
            res = search_second(program, log, inst + 1, acc + val);
            log[inst as usize] = false;
        },
        OpCode::Jmp(offset) => {
            log[inst as usize] = true;
            res = search_second(program, log, inst + offset, acc);
            if res == None && !log[program.len()] {
                log[program.len()] = true;
                res = search_second(program, log, inst + 1, acc);
                log[program.len()] = false;
            }
            log[inst as usize] = false;
        },
        OpCode::Nop(offset) => {
            log[inst as usize] = true;
            res = search_second(program, log, inst + 1, acc);
            if res == None && !log[program.len()] {
                log[program.len()] = true;
                res = search_second(program, log, inst + offset, acc);
                log[program.len()] = false;
            }
            log[inst as usize] = false;
        }
    }

    res
}

fn second(data: &str) {
    let program: Vec<OpCode> = data.lines().map(|x| x.parse::<OpCode>().expect("Failed to parse op-code")).collect();
    let mut call_log = vec![false; program.len() + 1];

    if let Some(res) = search_second(&program, &mut call_log, 0, 0) {
        println!("Found end. Accumulator value is {}", res);
    } else {
        println!("Daf?");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = fs::read_to_string(filename).expect("Failed to load boot code.");

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