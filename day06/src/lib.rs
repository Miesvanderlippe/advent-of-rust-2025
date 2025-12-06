use nom::character::streaming::one_of;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded};
use nom::{IResult, Parser};
use std::str::FromStr;

use nom::character::complete::{multispace1, space1, usize as nom_usize};

pub fn parse_factorials(s: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, nom_usize).parse(s)
}

pub fn parse_input(s: &str) -> IResult<&str, (Vec<Vec<usize>>, Vec<char>)> {
    pair(
        separated_list1(multispace1, parse_factorials),
        preceded(multispace1, separated_list1(space1, one_of("*+"))),
    )
    .parse(s)
}

pub fn solve_part_1(factorials: Vec<Vec<usize>>, operators: Vec<char>) -> usize {
    let mut grand_total = 0;
    for i in 0..operators.len() {
        match operators[i] {
            '+' => grand_total += factorials.iter().map(|r| r[i]).fold(0, |acc, u| acc + u),
            '*' => grand_total += factorials.iter().map(|r| r[i]).fold(1, |acc, u| acc * u),
            c => panic!("Unexpected operator {} in input", c),
        }
    }

    grand_total
}

pub fn solve_part_2(input: &str) -> usize {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut operator = lines[height - 1][0];
    let mut current_sum = 0;
    let mut grand_total = 0;

    for column in 0..width {
        let mut vertical_slice: Vec<u8> = Vec::with_capacity(height);
        vertical_slice = lines.iter().map(|l| l[column]).collect();

        if vertical_slice.iter().all(|c| *c == b' ') {
            grand_total += current_sum;
            current_sum = 0;
            continue;
        }

        if vertical_slice[height - 1] != b' ' {
            operator = vertical_slice[height - 1]
        }

        let number_str = str::from_utf8(&vertical_slice[0..height - 1])
            .expect("Failed to parse vertical slice as str")
            .trim();
        let number = usize::from_str(number_str).expect("Failed to parse str as number");

        match operator {
            b'*' => {
                current_sum = if current_sum == 0 {
                    number
                } else {
                    current_sum * number
                };
            }
            b'+' => current_sum += number,
            _ => panic!("Unknown operator"),
        }
    }

    // Last iteration problem.
    grand_total + current_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let example_input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";
        assert_eq!(
            parse_input(example_input),
            Ok((
                "",
                (
                    vec![
                        vec![123, 328, 51, 64],
                        vec![45, 64, 387, 23],
                        vec![6, 98, 215, 314]
                    ],
                    vec!['*', '+', '*', '+',]
                )
            ))
        )
    }

    #[test]
    fn test_example_part1() {
        let example_input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";
        let (_, (factorials, operators)) = parse_input(example_input).expect("Changed the parser?");

        assert_eq!(solve_part_1(factorials, operators), 4277556)
    }

    #[test]
    fn test_example_part2() {
        // Need to preserve trailing whitespace.
        let example_input = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .join("\n");

        assert_eq!(solve_part_2(&example_input), 3263827)
    }
}
