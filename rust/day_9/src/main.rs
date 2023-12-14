use std::{env, fs};

fn main() {
    let contents: String = load_input();
    let oasis_report_sum: u32 = oasis_report_sum(&contents);
    println!("Oasis report sum: {}", oasis_report_sum);
}

fn all_zero(values: &Vec<u32>) -> bool {
    for value in values {
        if *value != 0 {
            return false;
        }
    }

    true
}

fn find_next(values: &Vec<u32>) -> u32 {
    if all_zero(values) {
        return 0;
    }

    let mut next_sequence: Vec<u32> = Vec::new();
    let mut previous: u32 = values[0];
    let length: usize = values.len();
    let mut index: usize = 1;

    while index < length {
        next_sequence.push(values[index] - previous);
        previous = values[index];
        index += 1;
    }

    let next: u32 = previous + find_next(&next_sequence);
    // println!("Next: {}", next);
    next
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn oasis_report_sum(contents: &String) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();
    let mut sum: u32 = 0;

    for line in lines {
        // println!("{}", line);
        let values: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        // println!("{:?}", values);
        let next: u32 = find_next(&values);
        // println!("Next: {}", next);
        sum += next;
    }

    sum
}
