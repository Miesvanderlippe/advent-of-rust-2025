use std::{fs, path::PathBuf};

use clap::Parser;
use day02::{parse_input, repeats::gen_repeating_numbers};

#[derive(Parser, Debug)]
struct Cli {
    puzzle_input: PathBuf,
}

fn main() {
    let input = Cli::parse();

    match fs::read_to_string(input.puzzle_input) {
        Ok(text) => {
            let (_, ranges) = parse_input(&text).expect("Failed to parse input.");

            let sum = ranges.iter().fold(0, |acc: usize, r| {
                acc + gen_repeating_numbers(r.0, r.1).iter().sum::<usize>()
            });
            println!("{}", sum)
        }
        Err(file_error) => println!("Failed to open puzzle input with error {}", file_error),
    }
}
