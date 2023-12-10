use std::{collections::HashMap, env, fs};

fn main() {
    let contents: String = load_input();
    let sum_of_points: u32 = sum_of_points(&contents);
    println!("Sum of points: {}", sum_of_points);
    let sum_of_cards: u32 = sum_of_cards(&contents);
    println!("Sum of cards: {}", sum_of_cards);
}

fn card_numbers(line: &str) -> Vec<u32> {
    let parts: Vec<&str> = line.split(':').collect();
    let all_numbers: Vec<&str> = parts[1].split('|').collect();
    all_numbers[1]
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| x.parse::<u32>().unwrap())
        .collect()
}

fn card_points(line: &str) -> u32 {
    let winning_numbers: Vec<u32> = winning_numbers(line);
    // println!("winning_numbers: {:?}", winning_numbers);
    let card_numbers: Vec<u32> = card_numbers(line);
    // println!("card_numbers: {:?}", card_numbers);
    let common_numbers: Vec<u32> = common_numbers(winning_numbers, card_numbers);
    // println!("common_numbers: {:?}", common_numbers);
    let points: u32 = if common_numbers.len() == 0 {
        0
    } else {
        2u32.pow(common_numbers.len() as u32 - 1)
    };
    points
}

fn common_numbers(winning: Vec<u32>, card: Vec<u32>) -> Vec<u32> {
    winning
        .iter()
        .filter(|&x| card.contains(x))
        .cloned()
        .collect()
}

/// Adds initial cards to cards HashMap. Each line is a card.
fn init_cards(lines: Vec<&str>) -> HashMap<usize, u32> {
    let mut line_index: usize = 0;
    let mut cards: HashMap<usize, u32> = HashMap::new();
    lines.iter().for_each(|&_x| {
        cards.insert(line_index + 1, 1);
        line_index += 1;
    });

    cards
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn sum_of_cards(contents: &str) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();
    let lines_length: usize = lines.len();
    let mut line_index: usize = 0;
    let mut cards: HashMap<usize, u32> = init_cards(lines.clone());

    while line_index < lines_length {
        let line: &str = lines[line_index];
        // println!("{}: {}", line_index, line);
        // let card_count: u32 = card_count(line);
        let winning_numbers: Vec<u32> = winning_numbers(line);
        // println!("winning_numbers: {:?}", winning_numbers);
        let card_numbers: Vec<u32> = card_numbers(line);
        // println!("card_numbers: {:?}", card_numbers);
        let common_numbers: Vec<u32> = common_numbers(winning_numbers, card_numbers);
        // println!("common_numbers: {:?}", common_numbers);
        let mut card_index: usize = line_index + 2;
        let card_count: usize = common_numbers.len();

        while card_index < line_index + 2 + card_count {
            let card_value: u32 = cards[&(line_index + 1)];
            // println!("card_index: {}", card_index);
            cards.insert(card_index, cards[&card_index] + card_value);
            card_index += 1;
        }

        // println!("cards: {:?}", cards);
        line_index += 1;
    }

    total_cards(cards)
}

fn sum_of_points(contents: &str) -> u32 {
    let mut sum: u32 = 0;
    let lines: Vec<&str> = contents.lines().collect();
    let lines_length: usize = lines.len();
    let mut line_index: usize = 0;

    while line_index < lines_length {
        let line: &str = lines[line_index];
        // println!("{}: {}", line_index, line);
        let card_points: u32 = card_points(line);
        // println!("card_points: {}", card_points);

        sum += card_points;
        line_index += 1;
    }

    sum
}

fn total_cards(cards: HashMap<usize, u32>) -> u32 {
    cards.values().sum()
}

fn winning_numbers(line: &str) -> Vec<u32> {
    let parts: Vec<&str> = line.split(':').collect();
    let all_numbers: Vec<&str> = parts[1].split('|').collect();
    all_numbers[0]
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| x.parse::<u32>().unwrap())
        .collect()
}
