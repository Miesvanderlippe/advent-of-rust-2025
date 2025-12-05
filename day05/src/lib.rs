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

pub fn solve_part_2(fresh_ranges: &Vec<(usize, usize)>) -> usize {
    let mut condensed_ranges: Vec<(usize, usize)> = vec![];

    for (fr_start, fr_end) in fresh_ranges {
        let overlapping_condensed_ranges: Vec<(usize, (usize, usize))> = condensed_ranges
            .iter()
            .enumerate()
            .filter(|(_, (cr_start, cr_end))| {
                // The start or end of fresh range is within an existing cr_range.
                (
                    // Start
                    (fr_start >= cr_start && fr_start <= cr_end)
                    ||
                    // End
                    (fr_end >= cr_start && fr_end <= cr_end)
                )
                // The fresh range overlaps the entire condensed range
                || (fr_start <= cr_start && fr_end >= cr_end)
            })
            .map(|(index, (s, e))| (index, (*s, *e)))
            .collect();

        let mut new_start = *fr_start;
        let mut new_end = *fr_end;
        let mut delete_indexes = vec![];

        for (old_index, (cr_start, cr_end)) in overlapping_condensed_ranges {
            delete_indexes.push(old_index);
            if cr_start < new_start {
                new_start = cr_start
            }
            if cr_end > new_end {
                new_end = cr_end
            }
        }
        delete_indexes.sort();
        delete_indexes.reverse();
        for di in delete_indexes {
            condensed_ranges.remove(di);
        }
        condensed_ranges.push((new_start, new_end));
    }

    condensed_ranges
        .iter()
        .fold(0, |acc, (cr_start, cr_end)| acc + (cr_end - cr_start) + 1)
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

    #[test]
    fn test_part2_example() {
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
        let (_, (fresh_ranges, _)) = parse_input(example_input).expect("Changed the parser?");
        assert_eq!(solve_part_2(&fresh_ranges), 14)
    }

    #[test]
    fn test_part2_folding_1() {
        let example_input = "1-10
15-20
9-16

1";
        let (_, (fresh_ranges, _)) = parse_input(example_input).expect("Changed the parser?");
        assert_eq!(solve_part_2(&fresh_ranges), 20)
    }

    #[test]
    fn test_part2_folding_2() {
        let example_input = "1-10
9-12
12-13

1";
        let (_, (fresh_ranges, _)) = parse_input(example_input).expect("Changed the parser?");
        assert_eq!(solve_part_2(&fresh_ranges), 13)
    }

    #[test]
    fn test_part2_folding_3() {
        let example_input = "3-6
8-11
13-15
1-20

1";
        let (_, (fresh_ranges, _)) = parse_input(example_input).expect("Changed the parser?");
        assert_eq!(solve_part_2(&fresh_ranges), 20)
    }
}
