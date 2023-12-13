use std::{env, fs};

struct Connection {
    destination: u32,
    source: u32,
    range: u32,
}

impl Clone for Connection {
    fn clone(&self) -> Connection {
        Connection {
            destination: self.destination.clone(),
            source: self.source.clone(),
            range: self.range.clone(),
        }
    }
}

struct Seed {
    seed: u32,
    soil: u32,
    fertilizer: u32,
    water: u32,
    light: u32,
    temperature: u32,
    humidity: u32,
    location: u32,
}

fn main() {
    let contents: String = load_input();
    let lowest_location_number: u32 = lowest_location_number(&contents);
    println!("Lowest location number: {}", lowest_location_number);
}

fn build_seed(value: u32) -> Seed {
    Seed {
        seed: value,
        soil: 0,
        fertilizer: 0,
        water: 0,
        light: 0,
        temperature: 0,
        humidity: 0,
        location: 0,
    }
}

fn find_destination(connections: &Vec<Connection>, source: u32) -> u32 {
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

fn load_seeds(lines: Vec<&str>) -> Vec<Seed> {
    let mut seeds: Vec<Seed> = Vec::new();
    let line: &str = lines[0];
    let value_part: Vec<&str> = line.split(":").collect();
    let values: Vec<&str> = value_part[1].trim().split(" ").collect();

    for pair in values.chunks(2) {
        let min: u32 = pair[0].parse::<u32>().unwrap();
        let limit: u32 = pair[1].parse::<u32>().unwrap();

        for value in min..=min + limit - 1 {
            seeds.push(build_seed(value));
        }
    }

    println!("Loaded {} seeds", seeds.len());
    seeds
}

fn lowest_location_number(contents: &str) -> u32 {
    let seeds: Vec<Seed> = process_seeds(contents);

    // for seed in seeds {
    //     println!();
    //     println!("Seed {}", seed.seed);
    //     println!("Soil {}", seed.soil);
    //     println!("Fertilizer {}", seed.fertilizer);
    //     println!("Water {}", seed.water);
    //     println!("Light {}", seed.light);
    //     println!("Temperature {}", seed.temperature);
    //     println!("Humidity {}", seed.humidity);
    //     println!("Location {}", seed.location);
    // }

    seeds.iter().min_by_key(|&s| s.location).unwrap().location
}

fn process_fertilizer(seeds: &mut Vec<Seed>, lines: &Vec<&str>) {
    let connections: Vec<Connection> = load_connections(lines, "soil-to-fertilizer");

    for seed in seeds {
        seed.fertilizer = find_destination(&connections, seed.soil);
    }
}

fn process_humidity(seeds: &mut Vec<Seed>, lines: &Vec<&str>) {
    let connections: Vec<Connection> = load_connections(lines, "temperature-to-humidity");

    for seed in seeds {
        seed.humidity = find_destination(&connections, seed.temperature);
    }
}

fn process_light(seeds: &mut Vec<Seed>, lines: &Vec<&str>) {
    let connections: Vec<Connection> = load_connections(lines, "water-to-light");

    for seed in seeds {
        seed.light = find_destination(&connections, seed.water);
    }
}

fn process_location(seeds: &mut Vec<Seed>, lines: &Vec<&str>) {
    let connections: Vec<Connection> = load_connections(lines, "humidity-to-location");

    for seed in seeds {
        seed.location = find_destination(&connections, seed.humidity);
    }
}

fn process_seeds(contents: &str) -> Vec<Seed> {
    let lines: Vec<&str> = contents.lines().collect();
    let mut seeds: Vec<Seed> = load_seeds(lines.clone());
    process_soil(&mut seeds, &lines);
    println!("soil processed");
    process_fertilizer(&mut seeds, &lines);
    println!("fertilizer processed");
    process_water(&mut seeds, &lines);
    println!("water processed");
    process_light(&mut seeds, &lines);
    println!("light processed");
    process_temperature(&mut seeds, &lines);
    println!("temperature processed");
    process_humidity(&mut seeds, &lines);
    println!("humidity processed");
    process_location(&mut seeds, &lines);
    println!("Processed {} seeds", seeds.len());
    seeds
}

fn process_soil(seeds: &mut Vec<Seed>, lines: &Vec<&str>) {
    let connections: Vec<Connection> = load_connections(lines, "seed-to-soil");

    for seed in seeds {
        seed.soil = find_destination(&connections, seed.seed);
    }
}

fn process_temperature(seeds: &mut Vec<Seed>, lines: &Vec<&str>) {
    let connections: Vec<Connection> = load_connections(lines, "light-to-temperature");

    for seed in seeds {
        seed.temperature = find_destination(&connections, seed.light);
    }
}

fn process_water(seeds: &mut Vec<Seed>, lines: &Vec<&str>) {
    let connections: Vec<Connection> = load_connections(lines, "fertilizer-to-water");

    for seed in seeds {
        seed.water = find_destination(&connections, seed.fertilizer);
    }
}
