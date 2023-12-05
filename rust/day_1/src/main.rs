use std::env;
use std::fs;

fn callibration_value_sum(contents: &str) -> u32 {
    let mut sum = 0;

    for line in contents.lines() {
        let first: u32;
        let last: u32;

        match first_number(line) {
            Some(number) => {
                first = number;
            }
            None => continue,
        }

        match last_number(line) {
            Some(number) => {
                last = number;
            }
            None => continue,
        }

        sum += (first * 10) + last;
    }

    sum
}

fn first_number(line: &str) -> Option<u32> {
    for c in line.chars() {
        if c.is_digit(10) {
            return Some(c.to_digit(10).unwrap() as u32);
        }
    }

    None
}

fn last_number(line: &str) -> Option<u32> {
    for c in line.chars().rev() {
        if c.is_digit(10) {
            return Some(c.to_digit(10).unwrap() as u32);
        }
    }

    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let sum = callibration_value_sum(&contents);
    println!("Sum: {}", sum);
}
