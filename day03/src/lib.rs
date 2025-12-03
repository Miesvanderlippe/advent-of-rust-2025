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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(highest_joltage("987654321111111".as_bytes()), 98);
        assert_eq!(highest_joltage("811111111111119".as_bytes()), 89);
        assert_eq!(highest_joltage("234234234234278".as_bytes()), 78);
        assert_eq!(highest_joltage("818181911112111".as_bytes()), 92);
    }
}
