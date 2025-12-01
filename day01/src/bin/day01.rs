use std::{fs, path::PathBuf};

use clap::Parser;
use day01::{dial::Dial, rotation::Rotation};

#[derive(Parser, Debug)]
struct Cli {
    puzzle_input: PathBuf,
}

fn main() {
    let input = Cli::parse();

    match fs::read_to_string(input.puzzle_input) {
        Ok(text) => {
            let rotations = text
                .lines()
                .map(|l| Rotation::try_from(l).expect("Failed to parse line as rotation."));
            let mut dial = Dial::default();
            for rotation in rotations {
                dial.rotate(rotation);
            }
            println!("The part1 password is {}", dial.get_part1_password());
            println!("The part2 password is {}", dial.get_part2_password());
        }
        Err(file_error) => println!("Failed to open puzzle input with error {}", file_error),
    }
}
