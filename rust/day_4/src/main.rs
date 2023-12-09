use std::{env, fs};

fn main() {
    let contents: String = load_input();
    let sum_of_points = sum_of_points(&contents);
    println!("Sum of points: {}", sum_of_points);
}

fn card_points(line: &str) -> u32 {
    let winning_numbers = winning_numbers(line);
    // println!("winning_numbers: {:?}", winning_numbers);
    let card_numbers = card_numbers(line);
    // println!("card_numbers: {:?}", card_numbers);
    let common_numbers = common_numbers(winning_numbers, card_numbers);
    // println!("common_numbers: {:?}", common_numbers);
    let points: u32 = if common_numbers.len() == 0 {
        0
    } else {
        2u32.pow(common_numbers.len() as u32 - 1)
    };
    points
}

fn common_numbers(winning: Vec<u32>, card: Vec<u32>) -> Vec<u32> {
    winning
        .iter()
        .filter(|&x| card.contains(x))
        .cloned()
        .collect()
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn card_numbers(line: &str) -> Vec<u32> {
    let parts: Vec<&str> = line.split(':').collect();
    let all_numbers: Vec<&str> = parts[1].split('|').collect();
    all_numbers[1]
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| x.parse::<u32>().unwrap())
        .collect()
}

fn sum_of_points(contents: &str) -> u32 {
    let mut sum: u32 = 0;
    let lines: Vec<&str> = contents.lines().collect();
    let lines_length: usize = lines.len();
    let mut line_index: usize = 0;

    while line_index < lines_length {
        let line: &str = lines[line_index];
        // println!("{}: {}", line_index, line);
        let card_points: u32 = card_points(line);
        // println!("card_points: {}", card_points);

        sum += card_points;
        line_index += 1;
    }

    sum
}

fn winning_numbers(line: &str) -> Vec<u32> {
    let parts: Vec<&str> = line.split(':').collect();
    let all_numbers: Vec<&str> = parts[1].split('|').collect();
    all_numbers[0]
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| x.parse::<u32>().unwrap())
        .collect()
}
