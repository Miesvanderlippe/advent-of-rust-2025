// Headroom please

use nom::sequence::pair;
use nom::{IResult, Parser};
use nom::{
    bytes::tag, character::complete::newline, multi::separated_list1, sequence::separated_pair,
};

use nom::character::complete::usize as nom_usize;

pub fn parse_range(s: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(nom_usize, tag("-"), nom_usize).parse(s)
}

pub fn parse_input(s: &str) -> IResult<&str, (Vec<(usize, usize)>, Vec<usize>)> {
    separated_pair(
        separated_list1(newline, parse_range),
        pair(newline, newline),
        separated_list1(newline, nom_usize),
    )
    .parse(s)
}

pub fn solve_part_1(fresh_ranges: &Vec<(usize, usize)>, ingredients: &Vec<usize>) -> usize {
    ingredients
        .iter()
        .filter(|ing| {
            fresh_ranges
                .iter()
                .any(|(fresh_start, fresh_end)| *ing >= fresh_start && *ing <= fresh_end)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let example_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(
            parse_input(example_input),
            Ok((
                "",
                (
                    vec![
                        (3_usize, 5_usize),
                        (10_usize, 14_usize),
                        (16_usize, 20_usize),
                        (12_usize, 18_usize),
                    ],
                    vec![1_usize, 5_usize, 8_usize, 11_usize, 17_usize, 32_usize,]
                )
            ))
        )
    }

    #[test]
    fn test_part1_example() {
        let example_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let (_, (fresh_ranges, ingredients)) =
            parse_input(example_input).expect("Changed the parser?");
        assert_eq!(solve_part_1(&fresh_ranges, &ingredients), 3)
    }
}
