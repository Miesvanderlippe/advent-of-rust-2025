//

use crate::nr_of_digits_in_number;

pub fn gen_repeating_numbers(start: usize, end: usize) -> Vec<usize> {
    let number_of_digits_in_start = nr_of_digits_in_number(start);
    let number_of_digits_in_end = nr_of_digits_in_number(end);

    let mut result = vec![];

    // in ranges like 11 ..= 9999 we have posibilities for repeats in the 11
    // and the 2121 ranges.
    for cur_digits in number_of_digits_in_start..=number_of_digits_in_end {
        // Part one only asks for two repeating numbers, we can skip every
        // number with an odd number of digits.
        if cur_digits % 2 == 0 {
            let desired_digits = cur_digits / 2;

            // Used to find the first and last valid repeating number (100..=999)
            // and to lift the first half of the repeat to the apropiate positon
            // 100 -> 100_000, 100_000 + 100 = 100_100 (repeating)
            let power = 10_usize.pow(desired_digits as u32);

            for repeating in power / 10..=power - 1 {
                let t = power * repeating + repeating;

                if t > end {
                    return result;
                } else if t >= start {
                    result.push(t);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_examples() {
        assert_eq!(gen_repeating_numbers(11, 22), vec![11, 22]);
        assert_eq!(gen_repeating_numbers(95, 115), vec![99]);
        assert_eq!(gen_repeating_numbers(998, 1012), vec![1010]);
        assert_eq!(
            gen_repeating_numbers(1188511880, 1188511890),
            vec![1188511885]
        );
        assert_eq!(gen_repeating_numbers(222220, 222224), vec![222222]);
        assert_eq!(gen_repeating_numbers(1698522, 1698528), vec![]);
        assert_eq!(gen_repeating_numbers(446443, 446449), vec![446446]);
        assert_eq!(gen_repeating_numbers(38593856, 38593862), vec![38593859]);
    }
}
