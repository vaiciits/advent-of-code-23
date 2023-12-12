use std::{env, fs, os::windows::process};

struct Connection {
    destination: u32,
    source: u32,
    range: u32,
}

fn main() {
    let contents: String = load_input();
    let lowest_location_number: u32 = lowest_location_number(&contents);
    println!("Lowest location number: {}", lowest_location_number);
}

fn find_destination(lines: &Vec<&str>, key: &str, source: u32) -> u32 {
    let connections = load_connections(&lines, key);

    for connection in connections {
        if connection.source > source {
            continue;
        }

        if connection.source <= source
            && (connection.source as u64 + connection.range as u64 - 1) >= source as u64
        {
            return source - connection.source + connection.destination;
        }
    }

    source
}

fn load_connections(lines: &Vec<&str>, key: &str) -> Vec<Connection> {
    let mut connections: Vec<Connection> = Vec::new();
    let mut processing: bool = false;

    for line in lines {
        if !line.contains(key) {
            if !processing {
                continue;
            }
        }

        if !processing {
            processing = true;
            continue;
        }

        if line.is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split(" ").collect();
        connections.push(Connection {
            destination: parts[0].parse::<u32>().unwrap(),
            source: parts[1].parse::<u32>().unwrap(),
            range: parts[2].parse::<u32>().unwrap(),
        });
    }

    connections
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn location_from_seed(seed: u32, lines: &Vec<&str>) -> u32 {
    let mut value: u32 = seed;
    // println!("Seed: {}", value);
    value = find_destination(&lines, "seed-to-soil", value);
    // println!("Soil: {}", value);
    value = find_destination(&lines, "soil-to-fertilizer", value);
    // println!("Fertilizer: {}", value);
    value = find_destination(&lines, "fertilizer-to-water", value);
    // println!("Water: {}", value);
    value = find_destination(&lines, "water-to-light", value);
    // println!("Light: {}", value);
    value = find_destination(&lines, "light-to-temperature", value);
    // println!("Temperature: {}", value);
    value = find_destination(&lines, "temperature-to-humidity", value);
    // println!("Humidity: {}", value);
    value = find_destination(&lines, "humidity-to-location", value);
    // println!("Location: {}", value);
    // println!();

    value
}

fn lowest_location_number(contents: &str) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();
    let mut lowest_location: u32 = std::u32::MAX;

    let line: &str = lines[0];
    let parts: Vec<&str> = line.split(":").collect();
    let values: Vec<&str> = parts[1].trim().split(" ").collect();

    for pair in values.chunks(2) {
        let min: u32 = pair[0].parse::<u32>().unwrap();
        let limit: u32 = pair[1].parse::<u32>().unwrap();

        for value in min..=min + limit - 1 {
            let location = location_from_seed(value, &lines);

            if location < lowest_location {
                lowest_location = location;
            }
        }
    }

    lowest_location
}
