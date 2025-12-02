use std::{fs, path::PathBuf};

use clap::Parser;
use day02::parse_input;
use day02::part1::gen_repeating_numbers as solve_part_1;
use day02::part2::gen_repeating_numbers as solve_part_2;

#[derive(Parser, Debug)]
struct Cli {
    puzzle_input: PathBuf,
}

fn main() {
    let input = Cli::parse();

    match fs::read_to_string(input.puzzle_input) {
        Ok(text) => {
            let (_, ranges) = parse_input(&text).expect("Failed to parse input.");

            let part_1_ans = ranges.iter().fold(0, |acc: usize, r| {
                acc + solve_part_1(r.0, r.1).iter().sum::<usize>()
            });

            let part_2_ans = ranges.iter().fold(0, |acc: usize, r| {
                acc + solve_part_2(r.0, r.1).iter().sum::<usize>()
            });
            println!("Part 1: {}\nPart 2: {}", part_1_ans, part_2_ans)
        }
        Err(file_error) => println!("Failed to open puzzle input with error {}", file_error),
    }
}
