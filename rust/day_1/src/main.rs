use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let sum = callibration_value_sum(&contents);
    println!("Sum: {}", sum);
}

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
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            return Some(c.to_digit(10).unwrap() as u32);
        }

        for (word, number) in numbers() {
            if line[i..].starts_with(word) {
                return Some(number);
            }
        }
    }

    None
}

fn last_number(line: &str) -> Option<u32> {
    for (i, c) in line.chars().rev().enumerate() {
        if c.is_digit(10) {
            return Some(c.to_digit(10).unwrap() as u32);
        }

        let end = line.len() - i;
        for (word, number) in numbers() {
            if line[..end].ends_with(word) {
                return Some(number);
            }
        }
    }

    None
}

fn numbers() -> HashMap<&'static str, u32> {
    let mut numbers = HashMap::new();
    numbers.insert("zero", 0);
    numbers.insert("one", 1);
    numbers.insert("two", 2);
    numbers.insert("three", 3);
    numbers.insert("four", 4);
    numbers.insert("five", 5);
    numbers.insert("six", 6);
    numbers.insert("seven", 7);
    numbers.insert("eight", 8);
    numbers.insert("nine", 9);
    numbers
}
