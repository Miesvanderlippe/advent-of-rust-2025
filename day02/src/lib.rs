pub mod repeats;

use nom::Parser;
use nom::character::complete::usize;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, bytes::complete::tag};

pub fn nr_of_digits_in_number(number: usize) -> usize {
    (number.checked_ilog10().unwrap_or(0) + 1) as usize
}

pub fn parse_input(s: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(tag(","), separated_pair(usize, tag("-"), usize)).parse(s)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(
            parse_input(test_input),
            Ok((
                "",
                vec![
                    (11, 22),
                    (95, 115),
                    (998, 1012),
                    (1188511880, 1188511890),
                    (222220, 222224),
                    (1698522, 1698528),
                    (446443, 446449),
                    (38593856, 38593862),
                    (565653, 565659),
                    (824824821, 824824827),
                    (2121212118, 2121212124)
                ]
            ))
        );
    }

    #[test]
    fn nr_of_digits() {
        assert_eq!(nr_of_digits_in_number(1), 1);
        assert_eq!(nr_of_digits_in_number(22), 2);
        assert_eq!(nr_of_digits_in_number(333), 3);
        assert_eq!(nr_of_digits_in_number(4_444), 4);
        assert_eq!(nr_of_digits_in_number(55_555), 5);
        assert_eq!(nr_of_digits_in_number(666_666), 6);
        assert_eq!(nr_of_digits_in_number(7_777_777), 7);
        assert_eq!(nr_of_digits_in_number(88_888_888), 8);
        assert_eq!(nr_of_digits_in_number(999_999_999), 9);
    }
}
