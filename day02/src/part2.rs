use crate::{nr_of_digits_in_number, pattern::find_suitable_patterns};

pub fn gen_repeating_numbers(start: usize, end: usize) -> Vec<usize> {
    let number_of_digits_in_start = nr_of_digits_in_number(start);
    let number_of_digits_in_end = nr_of_digits_in_number(end);

    let mut result = vec![];

    // in ranges like 11 ..= 9999 we have posibilities for repeats in the 11
    // and the 2121 ranges.
    for cur_digits in number_of_digits_in_start..=number_of_digits_in_end {
        for pattern in find_suitable_patterns(cur_digits) {
            let desired_digits = pattern.size;

            // Used to find the first and last valid repeating number (100..=999)
            // and to lift the first half of the repeat to the apropiate positon
            // 100 -> 100_000, 100_000 + 100 = 100_100 (repeating)
            let power = 10_usize.pow(desired_digits as u32);

            // We could calculate a better starting position but this was annoying
            // to do keeping rolling over to the next power of of 10 in mind.
            for repeat in power / 10..=power - 1 {
                let mut t = repeat;

                for section in 1..pattern.repeats {
                    t += repeat * 10_usize.pow((desired_digits * section) as u32);
                }
                if t > end {
                    break;
                } else if t >= start {
                    result.push(t);
                }
            }
        }
    }

    // Remove duplicate numbers resulting from patterns generating the same numbers.
    // 2 2 2 2 2 2
    // 22  22  22
    // 222     222
    // Whoops, dedup only removes consecutive elements. Glad I did not spend 100 years debugging that before remembering..
    result.sort();
    result.dedup();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_examples() {
        assert_eq!(gen_repeating_numbers(11, 22), vec![11, 22]);
        assert_eq!(gen_repeating_numbers(95, 115), vec![99, 111]);
        assert_eq!(gen_repeating_numbers(998, 1012), vec![999, 1010]);
        assert_eq!(
            gen_repeating_numbers(1188511880, 1188511890),
            vec![1188511885]
        );
        assert_eq!(gen_repeating_numbers(222220, 222224), vec![222222]);
        assert_eq!(gen_repeating_numbers(1698522, 1698528), vec![]);
        assert_eq!(gen_repeating_numbers(446443, 446449), vec![446446]);
        assert_eq!(gen_repeating_numbers(38593856, 38593862), vec![38593859]);
        assert_eq!(gen_repeating_numbers(565653, 565659), vec![565656]);
        assert_eq!(gen_repeating_numbers(824824821, 824824827), vec![824824824]);
        assert_eq!(
            gen_repeating_numbers(2121212118, 2121212124),
            vec![2121212121]
        );
    }
}
