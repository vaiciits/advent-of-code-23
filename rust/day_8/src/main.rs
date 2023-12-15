use std::{char, collections::HashMap, env, fs};

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
    let step_count: u64 = step_count(&contents);
    println!("Step count: {}", step_count);
}

fn all_end_keys(current_keys: &Vec<&str>) -> bool {
    for key in current_keys {
        if !key.ends_with("Z") {
            return false;
        }
    }

    true
}

fn initial_nodes<'a>(nodes: &'a HashMap<&'a str, Node>) -> Vec<&str> {
    let mut initial_nodes: Vec<&str> = Vec::new();

    for key in nodes.keys() {
        if key.ends_with("A") {
            initial_nodes.push(key);
        }
    }

    initial_nodes
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

fn step_count(contents: &String) -> u64 {
    let lines: Vec<&str> = contents.lines().collect();
    let turns: Vec<Turn> = load_turns(&lines[0]);
    let nodes: HashMap<&str, Node> = load_nodes(&lines);
    let steps: u64 = step_counter(&turns, &nodes);
    steps
}

fn step_counter(turns: &Vec<Turn>, nodes: &HashMap<&str, Node>) -> u64 {
    let mut steps: u64 = 0;
    let mut index: usize = 0;
    let mut current_keys: Vec<&str> = initial_nodes(&nodes);

    loop {
        turn_nodes(&mut current_keys, &nodes, &turns[index]);
        steps += 1;

        if all_end_keys(&current_keys) {
            break;
        }

        // Move to the next index (looping back to the beginning if at the end)
        index = (index + 1) % turns.len();
    }

    steps
}

fn turn_nodes<'a>(current_keys: &mut Vec<&'a str>, nodes: &'a HashMap<&str, Node>, turn: &Turn) {
    for node in current_keys.iter_mut() {
        *node = match turn {
            Turn::Left => &nodes[node].left,
            Turn::Right => &nodes[node].right,
        };
    }
}
