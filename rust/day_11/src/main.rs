use std::{env, fs};

const EXPANSION_FACTOR: u32 = 1000000;

fn main() {
    let contents: String = load_input();
    let result: u64 = get_result(contents);
    println!("Result: {}", result);
}

fn col_steps(col: usize, col2: usize, empty_col_indexes: &Vec<usize>) -> u32 {
    let mut steps: u32 = 0;
    // println!("col_steps {} {} {:?}", col, col2, empty_col_indexes);
    let low: usize = if col < col2 { col } else { col2 };
    let high: usize = if col < col2 { col2 } else { col };

    for index in low..high {
        let mut step: u32 = 1;

        if empty_col_indexes.contains(&index) {
            // println!("Expanding");
            step *= EXPANSION_FACTOR;
        }

        steps += step;
        // println!("{} ", index);
    }

    // panic!("Not implemented");
    // println!("steps {}", steps);
    steps
}

fn column_empty(col_index: usize, grid: &Vec<Vec<char>>) -> bool {
    for row in grid {
        if row[col_index] != '.' {
            return false;
        }
    }

    true
}

fn empty_columns(grid: &Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_col_indexes: Vec<usize> = Vec::new();
    let mut col_index: usize = 0;
    let col_length: usize = grid[0].len();

    while col_index < col_length {
        if column_empty(col_index, &grid) {
            empty_col_indexes.push(col_index);
        }

        col_index += 1;
    }

    empty_col_indexes
}

fn get_result(contents: String) -> u64 {
    let grid: Vec<Vec<char>> = load_grid(contents);
    // print_grid(&grid);
    let empty_row_indexes: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row_empty(row))
        .map(|(index, _)| index)
        .collect();
    let empty_col_indexes: Vec<usize> = empty_columns(&grid);
    println!("Empty row indexes: {:?}", empty_row_indexes);
    println!("Empty col indexes: {:?}", empty_col_indexes);
    let shortest_path_sum: u64 = shortest_path_sum(&grid, &empty_row_indexes, &empty_col_indexes);
    println!("Shortest path sum: {}", shortest_path_sum);

    let result: u64 = shortest_path_sum;
    result
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

fn row_steps(row: usize, row2: usize, empty_row_indexes: &Vec<usize>) -> u32 {
    let mut steps: u32 = 0;
    // println!("row_steps {} {} {:?}", row, row2, empty_row_indexes);

    for index in row + 1..=row2 {
        let mut step: u32 = 1;

        if empty_row_indexes.contains(&index) {
            step *= EXPANSION_FACTOR;
        }

        // println!("{} ", index);
        steps += step;
    }

    // panic!("Not implemented");
    // println!("steps {}", steps);
    steps
}

fn shortest_path_between(
    row: usize,
    col: usize,
    row2: usize,
    col2: usize,
    empty_row_indexes: &Vec<usize>,
    empty_col_indexes: &Vec<usize>,
) -> u32 {
    let row_steps: u32 = row_steps(row, row2, empty_row_indexes);
    let col_steps: u32 = col_steps(col, col2, empty_col_indexes);
    let steps: u32 = row_steps + col_steps;
    // println!("{} {} - {} {} shortest {}", row, col, row2, col2, steps);
    steps
}

fn shortest_path_sum(
    grid: &Vec<Vec<char>>,
    empty_row_indexes: &Vec<usize>,
    empty_col_indexes: &Vec<usize>,
) -> u64 {
    let mut sum: u64 = 0;
    let mut row: usize = 0;
    let row_length: usize = grid.len();
    let col_length: usize = grid[0].len();

    while row < row_length {
        let mut col: usize = 0;

        while col < col_length {
            if grid[row][col] != '.' {
                sum +=
                    shortest_paths_from_sum(row, col, grid, empty_row_indexes, empty_col_indexes);
            }

            col += 1;
        }

        row += 1;
    }

    sum
}

fn shortest_paths_from_sum(
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>,
    empty_row_indexes: &Vec<usize>,
    empty_col_indexes: &Vec<usize>,
) -> u64 {
    let mut sum: u64 = 0;
    let mut row_index: usize = row;
    let mut col_index: usize = col + 1; // Start with next element
    let row_length: usize = grid.len();
    let col_length: usize = grid[0].len();

    while row_index < row_length {
        while col_index < col_length {
            if grid[row_index][col_index] != '.' {
                sum += shortest_path_between(
                    row,
                    col,
                    row_index,
                    col_index,
                    empty_row_indexes,
                    empty_col_indexes,
                ) as u64;
            }

            col_index += 1;
        }

        col_index = 0;
        row_index += 1;
    }

    // println!("row: {}, col: {}, sum: {}", row, col, sum);
    sum
}
