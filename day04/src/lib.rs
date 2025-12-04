// Let me have some headspace formatter

pub fn count_liftable_rolls(rolls: &str) -> Result<usize, &'static str> {
    let mut liftable_rolls = 0;
    let roll_width = match rolls.find('\n') {
        Some(w) => w,
        None => return Err("Use multiple lines please."),
    };

    let mut all_rolls = vec!['.'; roll_width];
    all_rolls.extend(rolls.chars().filter(|x| !x.is_whitespace()));
    all_rolls.extend(vec!['.'; roll_width]);

    for i in roll_width..all_rolls.len() - roll_width {
        if all_rolls[i] != '@' {
            continue;
        }

        let mut adjacent_rolls = 0;
        // No rolls to the left
        if i % roll_width != 0 {
            // To the left and above
            if all_rolls[i - roll_width - 1] == '@' {
                adjacent_rolls += 1;
            }
            // Directly to the left
            if all_rolls[i - 1] == '@' {
                adjacent_rolls += 1;
            }
            // Below and to the left
            if all_rolls[i + roll_width - 1] == '@' {
                adjacent_rolls += 1;
            }
        }

        if all_rolls[i - roll_width] == '@' {
            adjacent_rolls += 1;
        }
        if all_rolls[i + roll_width] == '@' {
            adjacent_rolls += 1;
        }

        // No rolls to the right
        if i % roll_width != roll_width - 1 {
            // To the right and above
            if all_rolls[i - roll_width + 1] == '@' {
                adjacent_rolls += 1;
            }
            // Directly to the right
            if all_rolls[i + 1] == '@' {
                adjacent_rolls += 1;
            }
            // Below and to the right
            if all_rolls[i + roll_width + 1] == '@' {
                adjacent_rolls += 1;
            }
        }

        if adjacent_rolls < 4 {
            liftable_rolls += 1;
        }
    }

    Ok(liftable_rolls)
}

pub fn solve_part_2(rolls: &str) -> Result<usize, &'static str> {
    let (roll_width, all_rolls) = fix_input(rolls)?;
    Ok(clean_the_damn_warehouse(all_rolls, roll_width, 0))
}

pub fn fix_input(rolls: &str) -> Result<(usize, Vec<char>), &'static str> {
    let roll_width = match rolls.find('\n') {
        Some(w) => w,
        None => return Err("Use multiple lines please."),
    };

    let mut all_rolls = vec!['.'; roll_width];
    all_rolls.extend(rolls.chars().filter(|x| !x.is_whitespace()));
    all_rolls.extend(vec!['.'; roll_width]);

    Ok((roll_width, all_rolls))
}

pub fn clean_the_damn_warehouse(
    mut all_rolls: Vec<char>,
    roll_width: usize,
    previously_liftable_rolls: usize,
) -> usize {
    let mut liftable_rolls = 0;

    for i in roll_width..all_rolls.len() - roll_width {
        if all_rolls[i] != '@' {
            continue;
        }

        let mut adjacent_rolls = 0;
        // No rolls to the left
        if i % roll_width != 0 {
            // To the left and above
            if all_rolls[i - roll_width - 1] == '@' {
                adjacent_rolls += 1;
            }
            // Directly to the left
            if all_rolls[i - 1] == '@' {
                adjacent_rolls += 1;
            }
            // Below and to the left
            if all_rolls[i + roll_width - 1] == '@' {
                adjacent_rolls += 1;
            }
        }

        if all_rolls[i - roll_width] == '@' {
            adjacent_rolls += 1;
        }
        if all_rolls[i + roll_width] == '@' {
            adjacent_rolls += 1;
        }

        // No rolls to the right
        if i % roll_width != roll_width - 1 {
            // To the right and above
            if all_rolls[i - roll_width + 1] == '@' {
                adjacent_rolls += 1;
            }
            // Directly to the right
            if all_rolls[i + 1] == '@' {
                adjacent_rolls += 1;
            }
            // Below and to the right
            if all_rolls[i + roll_width + 1] == '@' {
                adjacent_rolls += 1;
            }
        }

        if adjacent_rolls < 4 {
            liftable_rolls += 1;
            all_rolls[i] = '.';
        }
    }

    // Discovered no new rolls
    if liftable_rolls == 0 {
        previously_liftable_rolls + liftable_rolls
    } else {
        clean_the_damn_warehouse(
            all_rolls,
            roll_width,
            previously_liftable_rolls + liftable_rolls,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let rolls = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(count_liftable_rolls(rolls), Ok(13));
    }

    #[test]
    fn test_part2_example() {
        let rolls = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(solve_part_2(rolls), Ok(43));
    }
}
