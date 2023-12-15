use std::{
    char,
    collections::{HashMap, HashSet},
    env, fs,
};

struct Node {
    left: String,
    right: String,
}

enum Turn {
    Left,
    Right,
}

impl Turn {
    fn to_string(&self) -> &'static str {
        match self {
            Turn::Left => "Left",
            Turn::Right => "Right",
        }
    }
}

fn main() {
    let contents: String = load_input();
    let step_count: u128 = step_count(&contents);
    println!("Step count: {}", step_count);
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn initial_keys<'a>(nodes: &'a HashMap<&'a str, Node>) -> HashSet<&'a str> {
    let mut initial_keys: HashSet<&str> = HashSet::new();

    for key in nodes.keys() {
        if key.ends_with("A") {
            initial_keys.insert(key);
        }
    }

    initial_keys
}

fn least_common_multiple(values: &Vec<u32>) -> u128 {
    let mut lcm: u128 = 1;

    for value in values {
        lcm = lcm * (*value as u128) / gcd(lcm, *value as u128);
    }

    lcm
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn load_nodes<'a>(lines: &'a Vec<&'a str>) -> HashMap<&'a str, Node> {
    let mut nodes: HashMap<&str, Node> = HashMap::new();
    let mut count = 0;

    for line in lines {
        if count < 2 {
            count += 1;
            continue;
        }

        let parts: Vec<&str> = line.split(" = ").collect();
        let key: &str = parts[0];
        let value_part: &str = &parts[1][1..parts[1].len() - 1];
        let values: Vec<&str> = value_part.split(", ").collect();
        nodes.insert(
            key,
            Node {
                left: values[0].to_string(),
                right: values[1].to_string(),
            },
        );
    }

    // for (key, node) in nodes.iter() {
    //     println!("{}: {} {}", key, node.left, node.right);
    // }

    nodes
}

fn load_turns(line: &str) -> Vec<Turn> {
    let mut turns: Vec<Turn> = Vec::new();
    let chars: Vec<char> = line.chars().collect();

    for char in chars {
        match char {
            'L' => turns.push(Turn::Left),
            'R' => turns.push(Turn::Right),
            _ => (),
        }
    }

    // for turn in &turns {
    //     println!("{}", turn.to_string());
    // }

    turns
}

fn step_count(contents: &String) -> u128 {
    let lines: Vec<&str> = contents.lines().collect();
    let turns: Vec<Turn> = load_turns(&lines[0]);
    let nodes: HashMap<&str, Node> = load_nodes(&lines);
    let steps: u128 = step_counter(&turns, &nodes);
    steps
}

fn step_counter(turns: &Vec<Turn>, nodes: &HashMap<&str, Node>) -> u128 {
    let mut step: u32 = 0;
    let mut index: usize = 0;
    let mut current_keys: HashSet<&str> = initial_keys(&nodes);
    // println!("{:?}", current_keys);
    let mut steps: Vec<u32> = Vec::new();

    loop {
        current_keys = turn_nodes(&current_keys, &nodes, &turns[index]);
        step += 1;

        // println!("{:?}", current_keys);
        let mut new_keys: HashSet<&str> = HashSet::new();

        for key in &current_keys {
            if key.ends_with("Z") {
                steps.push(step);
            } else {
                new_keys.insert(key);
            }
        }

        current_keys = new_keys;
        if current_keys.len() == 0 {
            break;
        }

        // Move to the next index (looping back to the beginning if at the end)
        index = (index + 1) % turns.len();
    }

    // println!("{:?}", steps);
    let step_count: u128 = least_common_multiple(&steps);
    step_count
}

fn turn_nodes<'a>(
    current_keys: &HashSet<&'a str>,
    nodes: &'a HashMap<&str, Node>,
    turn: &Turn,
) -> HashSet<&'a str> {
    let mut next_keys: HashSet<&str> = HashSet::new();

    for node in current_keys.iter() {
        next_keys.insert(match turn {
            Turn::Left => &nodes[node].left,
            Turn::Right => &nodes[node].right,
        });
    }

    next_keys
}
