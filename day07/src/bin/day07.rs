use std::{fs, path::PathBuf};

use clap::Parser;
use day07::{solve_part_1, solve_part_2};

#[derive(Parser, Debug)]
struct Cli {
    puzzle_input: PathBuf,
}

fn main() {
    let input = Cli::parse();

    match fs::read_to_string(input.puzzle_input) {
        Ok(text) => {
            let part_1_ans = solve_part_1(&text).expect("Expected valid puzzle input.");
            let part_2_ans = solve_part_2(&text).expect("Expected valid puzzle input.");

            println!("Splits in part 1: {}", part_1_ans);
            println!("Splits in part 2: {}", part_2_ans);
        }
        Err(file_error) => println!("Failed to open puzzle input with error {}", file_error),
    }
}
