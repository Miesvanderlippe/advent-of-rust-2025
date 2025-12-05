use std::{fs, path::PathBuf};

use clap::Parser;
use day05::{parse_input, solve_part_1, solve_part_2};

#[derive(Parser, Debug)]
struct Cli {
    puzzle_input: PathBuf,
}

fn main() {
    let input = Cli::parse();

    match fs::read_to_string(input.puzzle_input) {
        Ok(text) => {
            let (_, (fresh_ranges, ingredients)) =
                parse_input(&text).expect("Failed to parse the input.");
            let part_1_ans = solve_part_1(&fresh_ranges, &ingredients);
            let part_2_ans = solve_part_2(&fresh_ranges);

            println!("Fresh ingredients in inventory: {}", part_1_ans);
            println!("All fresh ingredients: {}", part_2_ans);
        }
        Err(file_error) => println!("Failed to open puzzle input with error {}", file_error),
    }
}
