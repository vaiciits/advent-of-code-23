use std::env;
use std::fs;

fn main() {
    let contents: String = load_input();
    let sum: u32 = part_number_sum(&contents);
    println!("Sum: {}", sum);
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn last_digit_index(line: &str, start: usize) -> usize {
    let mut last_digit_index: usize = start;
    let mut char_index: usize = start + 1;

    while char_index < line.len() {
        if !line.chars().nth(char_index).unwrap().is_digit(10) {
            break;
        }

        last_digit_index = char_index;
        char_index += 1;
    }

    last_digit_index
}

fn number_is_part_number(
    lines: Vec<&str>,
    line_index: usize,
    start_char_index: usize,
    end_char_index: usize,
) -> bool {
    let mut current_line_index: usize = if (line_index as i32 - 1) >= 0 {
        line_index - 1
    } else {
        0
    };
    let last_line_index: usize = if line_index + 1 >= lines.len() {
        lines.len() - 1
    } else {
        line_index + 1
    };

    while current_line_index <= last_line_index {
        let line = lines[current_line_index];
        let mut current_char_index: usize = if (start_char_index as i32 - 1) <= 0 {
            start_char_index
        } else {
            start_char_index - 1
        };
        let last_char_index: usize = if end_char_index + 1 >= line.len() {
            end_char_index
        } else {
            end_char_index + 1
        };

        while current_char_index <= last_char_index {
            // Skip if current char is part of the number
            if current_line_index == line_index
                && current_char_index >= start_char_index
                && current_char_index <= end_char_index
            {
                current_char_index += 1;
                continue;
            }

            let c = line.chars().nth(current_char_index).unwrap();
            if !c.is_digit(10) && !c.is_alphabetic() && c != '.' {
                // println!("{}: {}", current_char_index, c);
                return true;
            }

            current_char_index += 1;
        }

        current_line_index += 1;
    }

    false
}

fn part_number_sum(contents: &str) -> u32 {
    let mut sum: u32 = 0;
    let lines: Vec<&str> = contents.lines().collect();
    let mut line_index: usize = 0;

    let lines_length: usize = lines.len();

    while line_index < lines_length {
        let line = lines[line_index];
        // println!("{}: {}", line_index, line);
        let line_length: usize = line.len();
        let mut char_index: usize = 0;

        while char_index < line_length {
            let c = line.chars().nth(char_index).unwrap();
            // println!("{}: {}", char_index, c);

            if c.is_digit(10) {
                // println!("digit");
                let last_digit_index = last_digit_index(line, char_index);
                // println!("last_digit_index: {}", last_digit_index);
                if number_is_part_number(lines.clone(), line_index, char_index, last_digit_index) {
                    let number = line[char_index..last_digit_index + 1]
                        .parse::<u32>()
                        .unwrap();
                    // println!("number: {}", number);
                    sum += number;
                }
                char_index = last_digit_index;
            }

            char_index += 1;
        }

        line_index += 1;
    }

    sum
}
