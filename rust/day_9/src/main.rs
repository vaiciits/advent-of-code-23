use std::{env, fs};

fn main() {
    let contents: String = load_input();
    let oasis_report_sum: i32 = oasis_report_sum(&contents);
    println!("Oasis report sum: {}", oasis_report_sum);
}

fn all_zero(values: &Vec<i32>) -> bool {
    for value in values {
        if *value != 0 {
            return false;
        }
    }

    true
}

fn find_next(values: &Vec<i32>) -> i32 {
    if all_zero(values) {
        return 0;
    }

    let mut next_sequence: Vec<i32> = Vec::new();
    let mut previous: i32 = values[0];
    let length: usize = values.len();
    let mut index: usize = 1;

    while index < length {
        next_sequence.push(values[index] - previous);
        previous = values[index];
        index += 1;
    }

    let next: i32 = previous + find_next(&next_sequence);
    // println!("Next: {}", next);
    next
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn oasis_report_sum(contents: &String) -> i32 {
    let lines: Vec<&str> = contents.lines().collect();
    let mut sum: i32 = 0;

    for line in lines {
        // println!("{}", line);
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        // println!("{:?}", values);
        let next: i32 = find_next(&values);
        // println!("Next: {}", next);
        sum += next;
    }

    sum
}
