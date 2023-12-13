use std::{env, fs};

fn main() {
    let contents: String = load_input();
    let margin_of_errror: u32 = margin_of_error(&contents);
    println!("Margin of error: {}", margin_of_errror);
}

fn extract_value(line: &str) -> u64 {
    let parts: Vec<&str> = line.split(":").collect();
    let mut values: Vec<&str> = parts[1].trim().split(" ").collect();
    values.retain(|&v| !v.is_empty());
    let value: String = values.join("");
    value.parse::<u64>().unwrap()
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn margin_of_error(contents: &str) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();
    let time: u64 = extract_value(lines[0]);
    let distance: u64 = extract_value(lines[1]);
    // println!("{:?}", times);
    // println!("{:?}", distances);
    number_of_ways(time, distance)
}

fn number_of_ways(total_time: u64, total_distance: u64) -> u32 {
    let mut number_of_ways: u32 = 0;
    let mut speed: u64 = 1;

    while speed <= total_time {
        if speed * (total_time - speed) > total_distance {
            number_of_ways += 1;
        }

        speed += 1;
    }

    // println!("{}", number_of_ways);
    number_of_ways
}
