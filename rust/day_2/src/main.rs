use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let contents = load_input();
    let sum = callibration_value_sum(&contents);
    println!("Sum: {}", sum);
}

fn callibration_value_sum(contents: &str) -> u32 {
    let mut sum = 0;

    for line in contents.lines() {
        let data = line.split(':').collect::<Vec<&str>>();

        if game_valid(data[1]) {
            sum += data[0].split(' ').collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();
        }
    }

    sum
}

fn colors() -> HashMap<&'static str, u32> {
    let mut colors = HashMap::new();
    colors.insert("red", 12);
    colors.insert("green", 13);
    colors.insert("blue", 14);
    colors
}

fn game_valid(data: &str) -> bool {
    let sets = data.split(';').collect::<Vec<&str>>();

    for set in sets {
        let cubes = set.split(',').collect::<Vec<&str>>();
        for cube in cubes {
            let parts = cube.trim().split(' ').collect::<Vec<&str>>();
            let color = parts[1];
            let count = parts[0].parse::<u32>().unwrap();

            if colors()[color] < count {
                return false;
            }
        }
    }

    true
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}
