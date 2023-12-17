use std::{env, fs};

fn main() {
    let contents: String = load_input();
    let sum_of_all_arrangements: usize = sum_of_all_arrangements(&contents);
    println!("Sum of all arrangements: {}", sum_of_all_arrangements);
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn row_arrangment(index: usize, chars: &Vec<char>, groups: &Vec<usize>) -> usize {
    if index >= chars.len() {
        let valid: bool = validate_row(&chars, &groups);
        // println!("Valid: {}", valid);
        return if valid { 1 } else { 0 };
    }

    let statuses: Vec<char> = vec!['.', '#'];
    let next_index: usize = index + 1;

    if chars[index] == '?' {
        let mut sum: usize = 0;

        for status in statuses {
            let mut new_chars: Vec<char> = chars.clone();
            new_chars[index] = status;
            let row_arrangement: usize = row_arrangment(next_index, &new_chars, &groups);
            sum += row_arrangement;
        }

        return sum;
    }

    row_arrangment(next_index, chars, groups)
}

fn row_arrangment_count(row: &str) -> usize {
    let parts: Vec<&str> = row.split_whitespace().collect();
    let chars: Vec<char> = parts[0].chars().collect();
    let groups = parts[1]
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    // println!("Groups: {:?}", groups);

    let row_arrangement: usize = row_arrangment(0, &chars, &groups);
    // println!("Row arrangement: {}", row_arrangement);
    row_arrangement
}

fn sum_of_all_arrangements(contents: &String) -> usize {
    let lines: Vec<&str> = contents.lines().collect();
    let mut sum: usize = 0;

    for line in lines {
        // println!("{}", line);
        let row_value: usize = row_arrangment_count(line);
        // println!("Row value: {}", row_value);
        sum += row_value;
    }

    sum
}

fn validate_row(chars: &Vec<char>, groups: &Vec<usize>) -> bool {
    // println!("Chars: {:?}", chars);
    let mut actual_groups: Vec<usize> = Vec::new();
    let mut broken: usize = 0;

    for char in chars {
        if *char == '#' {
            broken += 1;
        } else {
            if broken > 0 {
                actual_groups.push(broken);
                broken = 0;
            }
        }
    }

    if broken > 0 {
        actual_groups.push(broken);
    }

    // println!("Actual groups: {:?}", actual_groups);
    if actual_groups.len() != groups.len() {
        return false;
    }

    for (index, group) in groups.iter().enumerate() {
        if actual_groups[index] != *group {
            return false;
        }
    }

    true
}
