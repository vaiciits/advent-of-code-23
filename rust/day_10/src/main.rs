use std::{env, fs};

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn next(self, direction: Direction) -> Coord {
        match direction {
            Direction::North => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Coord {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Coord {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn main() {
    let contents: String = load_input();
    let steps_to_furthest: usize = find_steps_to_furthest(contents);
    println!("Steps to furthest: {}", steps_to_furthest);
}

fn create_grid(contents: String) -> Vec<Vec<char>> {
    let lines: Vec<&str> = contents.lines().collect();
    let rows: usize = lines.len();
    let cols: usize = lines[0].len();
    let mut grid: Vec<Vec<char>> = vec![vec![' '; cols]; rows];

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid[row][col] = c;
        }
    }

    grid
}

fn find_inital_direction(grid: &Vec<Vec<char>>, start: Coord) -> Direction {
    if start.y > 0 && ['|', '7', 'F'].contains(&grid[start.y - 1][start.x]) {
        return Direction::North;
    }

    if start.y < grid.len() - 1 && ['|', 'L', 'J'].contains(&grid[start.y + 1][start.x]) {
        return Direction::South;
    }

    if start.x > 0 && ['-', 'F', 'L'].contains(&grid[start.y][start.x - 1]) {
        return Direction::West;
    }

    if start.x < grid[0].len() - 1 && ['-', '7', 'J'].contains(&grid[start.y][start.x + 1]) {
        return Direction::East;
    }

    panic!("No direction found");
}

fn find_start(grid: &Vec<Vec<char>>) -> Coord {
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();

    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == 'S' {
                return Coord { x: x, y: y };
            }
        }
    }

    panic!("No start found");
}

fn find_steps_to_furthest(contents: String) -> usize {
    let grid: Vec<Vec<char>> = create_grid(contents);
    // print_grid(&grid);
    let start = find_start(&grid);
    println!("Start: {} {}", start.y, start.x);
    // grid[start.y as usize][start.x as usize] = '0';
    // print_grid(&grid);
    let direction: Direction = find_inital_direction(&grid, start);
    println!("Direction: {:?}", direction);

    let steps: Vec<Coord> = walk_grid(&grid, start, direction);
    let furthest: usize = steps.len() / 2;
    furthest
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn next_direction(value: char, direction: Direction) -> Direction {
    // println!("Value: {}", value);
    // println!("Direction: {:?}", direction);
    let directions: Vec<Direction> = match value {
        '|' => vec![Direction::North, Direction::South],
        '-' => vec![Direction::East, Direction::West],
        'L' => vec![Direction::North, Direction::East],
        'J' => vec![Direction::North, Direction::West],
        '7' => vec![Direction::South, Direction::West],
        'F' => vec![Direction::South, Direction::East],
        _ => panic!("Unknown value: {}", value),
    };
    // println!("Directions: {:?}", directions);

    for d in directions {
        if d != direction {
            // println!("Matched {:?} {:?}", d, direction);
            return d;
        }
    }

    panic!("No direction found");
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for &c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn walk_grid(grid: &Vec<Vec<char>>, start: Coord, direction: Direction) -> Vec<Coord> {
    let mut steps: Vec<Coord> = Vec::new();
    steps.push(start);
    let mut current: Coord = start;
    // println!("Direction: {:?}", direction);
    let mut next_dir: Direction = direction;

    loop {
        // println!("current: {:?}", current);
        current = current.next(next_dir);

        if current.x == start.x && current.y == start.y {
            break;
        }

        steps.push(current);
        // print_grid(grid);
        next_dir = next_direction(
            grid[current.y as usize][current.x as usize],
            next_dir.opposite(),
        );
        // println!("Next direction: {:?}", next_dir);
    }

    // println!("{:?}", steps);
    steps
}
