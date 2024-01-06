use std::{env, fs};

fn main() {
    let contents: String = load_input();
    let sum_of_notes: usize = sum_of_notes(&contents);
    println!("Sum of notes: {}", sum_of_notes);
}

fn get_patterns(contents: &str) -> Vec<Vec<Vec<char>>> {
    let mut patterns: Vec<Vec<Vec<char>>> = Vec::new();
    let pattern_strings: Vec<&str> = contents.split("\r\n\r\n").collect();

    for pattern_string in pattern_strings {
        let pattern: Vec<Vec<char>> = pattern_string
            .split("\r\n")
            .map(|row: &str| row.chars().collect())
            .collect();
        patterns.push(pattern);
    }

    patterns
}

fn get_value(pattern: &Vec<Vec<char>>) -> usize {
    // let transposed: Vec<Vec<char>> = rotate_2d_vector(pattern.clone());
    // let mut value: usize = pattern_value(&transposed);
    // // println!("Vertical value: {}", value);

    // if value > 0 {
    //     println!("Vertical value: {}", value);
    //     // return value;
    // }

    let horizontal_value: usize = pattern_value(&pattern);
    // println!("Horizontal value: {}", horizontal_value * 100);
    // value += horizontal_value * 100;
    // horizontal_value * 100

    let transposed: Vec<Vec<char>> = rotate_2d_vector(pattern.clone());
    let vertical_value: usize = pattern_value(&transposed);
    // println!("Vertical value: {}", vertical_value);

    let value: usize = horizontal_value * 100 + vertical_value;

    value
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn rotate_2d_vector(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // println!("Input: {:?}", input);
    let rows: usize = input.len();
    let cols: usize = input[0].len();
    // println!("Col count: {}", cols);
    // println!("Col: {:?}", input[0]);
    let mut rotated: Vec<Vec<char>> = vec![vec!['a'; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated[j][i] = input[i][j];
        }
    }

    // println!();
    // for line in &rotated {
    //     let chars: String = line.into_iter().collect();
    //     println!("{}", chars);
    // }
    rotated
}

fn sum_of_notes(contents: &str) -> usize {
    let patterns: Vec<Vec<Vec<char>>> = get_patterns(&contents);
    let mut sum: usize = 0;

    for pattern in patterns {
        // println!("{:?}", pattern);

        // for line in &pattern {
        //     let chars: String = line.into_iter().collect();
        //     println!("{}", chars);
        // }
        let value: usize = get_value(&pattern);
        // println!("Value: {}", value);
        sum += value;
    }

    sum
}

fn symetrical(pattern: &Vec<Vec<char>>, index: usize) -> bool {
    // println!("Index: {}", index);
    let mut first: usize = index;
    let mut second: usize = index + 1;
    let length: usize = pattern.len();

    while second < length {
        if pattern[first] != pattern[second] {
            return false;
        }

        if first == 0 {
            break;
        }

        second += 1;
        first -= 1;
    }

    return true;
}

fn pattern_value(pattern: &Vec<Vec<char>>) -> usize {
    let mut index: usize = 0;
    let length: usize = pattern.len();
    // println!("Length: {}", length);

    while index < length - 1 {
        if symetrical(&pattern, index) {
            return index + 1;
        }

        index += 1;
    }

    return 0;
}
