use std::{fs, path::PathBuf};

use clap::Parser;
use day04::count_liftable_rolls;

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

            println!("Liftable rolls: {}", part_1_ans);
        }
        Err(file_error) => println!("Failed to open puzzle input with error {}", file_error),
    }
}
