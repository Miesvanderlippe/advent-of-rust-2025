use std::{fs, path::PathBuf};

use clap::Parser;
use day03::{highest_joltage, mega_joltage};

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
            let part_1_ans = text
                .lines()
                .fold(0, |acc: usize, line| acc + highest_joltage(line.as_bytes()));

            let part_2_ans = text.lines().fold(0, |acc: usize, line| {
                acc + mega_joltage(line.as_bytes(), 12)
            });

            println!("Combined joltage: {}", part_1_ans);
            println!("MEGA joltage: {}", part_2_ans);
        }
        Err(file_error) => println!("Failed to open puzzle input with error {}", file_error),
    }
}
