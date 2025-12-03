use day03::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part_2_main() {
    mega_joltage(divan::black_box(
        include_str!("../../puzzle_inputs/day03/input.txt").as_bytes(),
    ));
}

#[divan::bench]
fn part_2_alt() {
    mega_joltage_alt(divan::black_box(
        include_str!("../../puzzle_inputs/day03/input.txt").as_bytes(),
    ));
}
