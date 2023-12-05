use std::env;
use std::fs;

fn callibration_value_sum(contents: &str) -> i32 {
    let mut sum = 0;

    for line in contents.lines() {
        let first = first_number(line);
        let last: i32 = last_number(line);
        sum += first * 10 + last;
    }

    sum
}

fn first_number(line: &str) -> i32 {
    let mut first = 0;

    for c in line.chars() {
        if c.is_digit(10) {
            first = c.to_digit(10).unwrap() as i32;
            break;
        }
    }

    first
}

fn last_number(line: &str) -> i32 {
    let mut last = 0;

    for c in line.chars().rev() {
        if c.is_digit(10) {
            last = c.to_digit(10).unwrap() as i32;
            break;
        }
    }

    last
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let sum = callibration_value_sum(&contents);
    println!("Sum: {}", sum);
}
