use std::{cmp::Ordering, collections::HashMap, env, fs};

struct Hand {
    hand_type: HandType,
    values: Vec<u8>,
    bid: u32,
}

enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn to_string(&self) -> &'static str {
        match self {
            HandType::HighCard => "HighCard",
            HandType::OnePair => "OnePair",
            HandType::TwoPairs => "TwoPairs",
            HandType::ThreeOfAKind => "ThreeOfAKind",
            HandType::FullHouse => "FullHouse",
            HandType::FourOfAKind => "FourOfAKind",
            HandType::FiveOfAKind => "FiveOfAKind",
        }
    }
    fn value(&self) -> u8 {
        match self {
            HandType::HighCard => 1,
            HandType::OnePair => 2,
            HandType::TwoPairs => 3,
            HandType::ThreeOfAKind => 4,
            HandType::FullHouse => 5,
            HandType::FourOfAKind => 6,
            HandType::FiveOfAKind => 7,
        }
    }
}

fn main() {
    let contents: String = load_input();
    let total_winnings: u32 = total_winnings(&contents);
    println!("Total winnings: {}", total_winnings);
}

fn card_value(ch: char) -> u8 {
    match ch {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

fn compare_values(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
    for i in 0..a.len() {
        if a[i] < b[i] {
            return Ordering::Less;
        } else if a[i] > b[i] {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}

fn hand_type(value: &str) -> HandType {
    let mut char_count: HashMap<char, u8> = HashMap::new();

    for ch in value.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }

    let mut max_key: Option<char> = None;
    let mut max_value: u8 = 0;

    for (&key, &value) in char_count.iter() {
        if value > max_value {
            max_key = Some(key);
            max_value = value;
        }
    }

    // dbg!(&char_count);

    let hand_type: HandType = match max_value {
        1 => HandType::HighCard,
        2 => HandType::OnePair,
        3 => HandType::ThreeOfAKind,
        4 => HandType::FourOfAKind,
        5 => HandType::FiveOfAKind,
        _ => HandType::HighCard,
    };

    // let mut char_count_clone: HashMap<char, u8> = char_count.clone();

    match hand_type {
        HandType::OnePair | HandType::ThreeOfAKind => {
            if let Some(max_key) = max_key {
                char_count.remove(&max_key);

                if char_count.values().any(|&v| v == 2) {
                    return match hand_type {
                        HandType::OnePair => HandType::TwoPairs,
                        HandType::ThreeOfAKind => HandType::FullHouse,
                        _ => hand_type,
                    };
                }
            }
        }
        _ => {}
    }

    // dbg!(hand_type.to_string());
    hand_type
}

fn load_hands(lines: &Vec<&str>) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let value: &str = parts[0];
        let bid: u32 = parts[1].parse::<u32>().unwrap();
        let hand_type = hand_type(value);
        // println!("{:?}", hand_type.to_string());
        let mut values: Vec<u8> = Vec::new();
        values.push(card_value(value.chars().nth(0).unwrap()));
        values.push(card_value(value.chars().nth(1).unwrap()));
        values.push(card_value(value.chars().nth(2).unwrap()));
        values.push(card_value(value.chars().nth(3).unwrap()));
        values.push(card_value(value.chars().nth(4).unwrap()));
        hands.push(Hand {
            hand_type,
            values,
            bid,
        });
    }

    hands
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn total_winnings(contents: &str) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();
    let mut hands: Vec<Hand> = load_hands(&lines);

    hands.sort_by(|a, b| {
        let a_value: u8 = a.hand_type.value();
        let b_value: u8 = b.hand_type.value();
        if a_value < b_value {
            Ordering::Less
        } else if a_value == b_value {
            compare_values(&a.values, &b.values)
        } else {
            Ordering::Greater
        }
    });

    let mut winnings: u32 = 0;

    for (index, hand) in hands.iter().enumerate() {
        // println!(
        //     "{} {} - {} {} {} {} {}",
        //     hand.hand_type.to_string(),
        //     hand.bid,
        //     hand.values[0],
        //     hand.values[1],
        //     hand.values[2],
        //     hand.values[3],
        //     hand.values[4],
        // );
        winnings += hand.bid * (index + 1) as u32;
    }

    winnings
}
