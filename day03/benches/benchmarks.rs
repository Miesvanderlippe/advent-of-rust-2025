use std::io::BufRead;

use day03::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part_2_main() {
    divan::black_box(include_str!("../../puzzle_inputs/day03/input.txt"))
        .lines()
        .map(|l| mega_joltage(l.as_bytes()))
        .fold(0, |a, u| a + u);
}

#[divan::bench]
fn part_2_alt() {
    divan::black_box(include_str!("../../puzzle_inputs/day03/input.txt"))
        .lines()
        .map(|l| mega_joltage_alt(l.as_bytes()))
        .fold(0, |a, u| a + u);
}
