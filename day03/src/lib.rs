pub fn highest_joltage(cells: &[u8]) -> usize {
    let mut left_joltage = cells[0];
    let mut right_joltage = 0_u8;

    let number_of_cells = cells.len();
    // Already set the first cell to be the left_joltage
    for i in 1..number_of_cells {
        // Only set left_joltage if there is a cell to be right_joltage.
        // Because left_joltage is multiplied by 10, right_joltage can be whatever
        // and still be higher.
        if cells[i] > left_joltage && i + 1 != number_of_cells {
            left_joltage = cells[i];
            right_joltage = cells[i + 1];
        } else if cells[i] > right_joltage {
            right_joltage = cells[i];
        }
    }

    // Dirty ascii to number magic.
    ((left_joltage - 48) * 10 + (right_joltage - 48)) as usize
}

pub fn mega_joltage(cells: &[u8], series_len: usize) -> usize {
    let mut joltages: Vec<u8> = vec![0; series_len];

    let number_of_cells = cells.len();

    // 0 - 100 in the input data.
    for cell_index in 0..=number_of_cells - series_len {
        // 0 - 12 in the part 2 exercise.
        for series_index in 0..series_len {
            if cells[cell_index + series_index] > joltages[series_index] {
                for ji in series_index..series_len {
                    joltages[ji] = cells[cell_index + ji]
                }
            }
        }
    }

    let mut combined_joltage: usize = joltages[series_len - 1] as usize - 48_usize;

    for si in 1..series_len {
        combined_joltage +=
            10_usize.pow(si as u32) * (joltages[series_len - si - 1] as usize - 48_usize);
    }

    combined_joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(highest_joltage("987654321111111".as_bytes()), 98);
        assert_eq!(highest_joltage("811111111111119".as_bytes()), 89);
        assert_eq!(highest_joltage("234234234234278".as_bytes()), 78);
        assert_eq!(highest_joltage("818181911112111".as_bytes()), 92);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(mega_joltage("987654321111111".as_bytes(), 12), 987654321111);
        assert_eq!(mega_joltage("811111111111119".as_bytes(), 12), 811111111119);
        assert_eq!(mega_joltage("234234234234278".as_bytes(), 12), 434234234278);
        assert_eq!(mega_joltage("818181911112111".as_bytes(), 12), 888911112111);
    }
}
