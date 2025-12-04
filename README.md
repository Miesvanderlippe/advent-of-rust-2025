# Advent of Rust 2025

My repository for advent of code 2025. May or may not finish it this year!

## Day01

Read the puzzle story in [day01/README.md](day01/README.md)

```bash
cargo run --bin day01 puzzle_inputs/day01/part1.txt
```

The correct output for my puzzle input was: 

```
The part1 password is 992
The part2 password is 6133
```

## Day02

Read the puzzle story in [day02/README.md](day02/README.md)

```bash
cargo run --bin day02 puzzle_inputs/day02/input.txt
```

```
Part 1: 28146997880
Part 2: 40028128307
```


## Day03

Read the puzzle story in [day03/README.md](day03/README.md)


```bash
cargo run --bin day03 puzzle_inputs/day03/input.txt
```

The correct output for my puzzle input was:
```
Combined joltage: 16946
MEGA joltage: 168627047606506
```

A fiend showed me their solution which seemed much faster. I benched it and it is;

```bash
cargo bench
```

```
Running benches/benchmarks.rs (target/release/deps/day_03_bench-484f332befbee662)
Timer precision: 41 ns
day_03_bench    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part_2_alt   36.79 µs      │ 94.79 µs      │ 37.08 µs      │ 44.52 µs      │ 100     │ 100
╰─ part_2_main  103.1 µs      │ 223.8 µs      │ 103.9 µs      │ 119.3 µs      │ 100     │ 100
```

The benchmark iterates over the entire input file.
