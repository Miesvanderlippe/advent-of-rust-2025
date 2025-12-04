use std::{fs, path::PathBuf};

use clap::Parser;
use day04::{count_liftable_rolls, solve_part_2};

#[derive(Parser, Debug)]
struct Cli {
    puzzle_input: PathBuf,
}

fn main() {
    let input = Cli::parse();

    match fs::read_to_string(input.puzzle_input) {
        Ok(text) => {
            if !text.is_ascii() {
                panic!("Code only works for ascii chars");
            }
            let part_1_ans = count_liftable_rolls(&text).expect("Failed to parse input");
            let part_2_ans = solve_part_2(&text).expect("Failed to parse input");

            println!("Liftable rolls: {}", part_1_ans);
            println!("Liftable rolls (cleaning the warehouse): {}", part_2_ans);
        }
        Err(file_error) => println!("Failed to open puzzle input with error {}", file_error),
    }
}
