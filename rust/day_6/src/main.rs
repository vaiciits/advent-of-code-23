use std::{env, fs};

fn main() {
    let contents: String = load_input();
    let margin_of_errror: u32 = margin_of_error(&contents);
    println!("Margin of error: {}", margin_of_errror);
}

fn extract_values(line: &str) -> Vec<u32> {
    let parts: Vec<&str> = line.split(":").collect();
    let mut values: Vec<&str> = parts[1].trim().split(" ").collect();
    values.retain(|&v| !v.is_empty());
    values
        .iter()
        .map(|v: &&str| v.parse::<u32>().unwrap())
        .collect()
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn margin_of_error(contents: &str) -> u32 {
    // println!("{}", contents);
    let lines: Vec<&str> = contents.lines().collect();
    let times: Vec<u32> = extract_values(lines[0]);
    let distances: Vec<u32> = extract_values(lines[1]);
    // println!("{:?}", times);
    // println!("{:?}", distances);

    let mut margin_of_error: u32 = 1;
    let race_count: usize = times.len();
    let mut count: usize = 0;

    while count < race_count {
        let total_time: u32 = times[count];
        let total_distance: u32 = distances[count];
        // println!("{} {}", total_time, total_distance);
        let number_of_ways: u32 = number_of_ways(total_time, total_distance);

        if number_of_ways > 0 {
            margin_of_error *= number_of_ways;
        }

        count += 1;
    }

    margin_of_error
}

fn number_of_ways(total_time: u32, total_distance: u32) -> u32 {
    let mut number_of_ways: u32 = 0;
    let mut speed: u32 = 1;

    while speed <= total_time {
        if speed * (total_time - speed) > total_distance {
            number_of_ways += 1;
        }

        speed += 1;
    }

    // println!("{}", number_of_ways);
    number_of_ways
}
