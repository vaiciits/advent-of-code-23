use std::{env, fs};

fn main() {
    let contents: String = load_input();
    let result: u32 = get_result(contents);
    println!("Result: {}", result);
}

fn column_empty(col_index: usize, grid: &Vec<Vec<char>>) -> bool {
    for row in grid {
        if row[col_index] != '.' {
            return false;
        }
    }

    true
}

fn expand_grid(grid: &mut Vec<Vec<char>>) {
    let mut col_index: usize = 0;

    while col_index < grid[0].len() {
        if column_empty(col_index, &grid) {
            insert_column(col_index, grid);
            col_index += 1; // Inserted a copy
        }

        col_index += 1;
    }

    let mut row_index: usize = 0;

    while row_index < grid.len() {
        if row_empty(&grid[row_index]) {
            grid.insert(row_index, grid[row_index].clone());
            row_index += 1; // Inserted a copy
        }

        row_index += 1;
    }
}

fn get_result(contents: String) -> u32 {
    let mut grid: Vec<Vec<char>> = load_grid(contents);
    // print_grid(&grid);
    expand_grid(&mut grid);
    // print_grid(&grid);
    let shortest_path_sum: u32 = shortest_path_sum(&grid);
    println!("Shortest path sum: {}", shortest_path_sum);

    let result: u32 = shortest_path_sum;
    result
}

fn insert_column(col_index: usize, grid: &mut Vec<Vec<char>>) {
    for row in grid {
        row.insert(col_index, '.');
    }
}

fn load_grid(contents: String) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        grid.push(line.chars().collect());
    }

    grid
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    println!();
}

fn row_empty(row: &Vec<char>) -> bool {
    for c in row {
        if *c != '.' {
            return false;
        }
    }

    true
}

fn shortest_path_between(row: usize, col: usize, row2: usize, col2: usize) -> u32 {
    let row_steps: usize = row2 - row;
    let col_steps: i32 = col2 as i32 - col as i32;
    let steps: u32 = row_steps as u32 + col_steps.abs() as u32;
    // println!("{} {} - {} {} shortest {}", row, col, row2, col2, steps);
    steps
}

fn shortest_path_sum(grid: &Vec<Vec<char>>) -> u32 {
    let mut sum: u32 = 0;
    let mut row: usize = 0;
    let row_length: usize = grid.len();
    let col_length: usize = grid[0].len();

    while row < row_length {
        let mut col: usize = 0;

        while col < col_length {
            if grid[row][col] != '.' {
                sum += shortest_paths_from_sum(row, col, grid);
            }

            col += 1;
        }

        row += 1;
    }

    sum
}

fn shortest_paths_from_sum(row: usize, col: usize, grid: &Vec<Vec<char>>) -> u32 {
    let mut sum: u32 = 0;
    let mut row_index: usize = row;
    let mut col_index: usize = col + 1; // Start with next element
    let row_length: usize = grid.len();
    let col_length: usize = grid[0].len();

    while row_index < row_length {
        while col_index < col_length {
            if grid[row_index][col_index] != '.' {
                sum += shortest_path_between(row, col, row_index, col_index);
            }

            col_index += 1;
        }

        col_index = 0;
        row_index += 1;
    }

    // println!("row: {}, col: {}, sum: {}", row, col, sum);
    sum
}
