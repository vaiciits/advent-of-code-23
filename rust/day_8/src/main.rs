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
    let step_count: u32 = step_count(&contents);
    println!("Step count: {}", step_count);
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

fn step_count(contents: &String) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();
    let turns: Vec<Turn> = load_turns(&lines[0]);

    let nodes: HashMap<&str, Node> = load_nodes(&lines);
    let steps: u32 = step_counter(&turns, &nodes);
    steps
}

fn step_counter(turns: &Vec<Turn>, nodes: &HashMap<&str, Node>) -> u32 {
    let mut steps: u32 = 0;
    let mut index: usize = 0;
    let mut current_node: &str = "AAA";
    // println!("Begin: {}", current_node);

    loop {
        let turn: &Turn = &turns[index];

        current_node = match turn {
            Turn::Left => &nodes[current_node].left,
            Turn::Right => &nodes[current_node].right,
        };
        steps += 1;
        // println!("{}: {}", turn.to_string(), current_node);

        if current_node == "ZZZ" {
            break;
        }

        // Move to the next index (looping back to the beginning if at the end)
        index = (index + 1) % turns.len();
    }

    // println!("count {}", steps);
    steps
}
