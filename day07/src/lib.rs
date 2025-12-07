//

pub fn solve_part_1(puzzle_input: &str) -> Result<usize, ()> {
    let lines: Vec<&[u8]> = puzzle_input.lines().map(|l| l.as_bytes()).collect();
    let width = lines[0].len();

    let mut beams: Vec<bool> = lines[0].iter().map(|c| *c == b'S').collect();
    let mut splits = 0;

    for row in 1..lines.len() {
        let mut next_beams = vec![false; width];

        for col in 0..width {
            if beams[col] && lines[row][col] == b'^' {
                next_beams[col - 1] = true;
                next_beams[col + 1] = true;
                splits += 1;
            } else if beams[col] && lines[row][col] == b'.' {
                next_beams[col] = true;
            }
        }

        beams = next_beams;
    }

    Ok(splits)
}

//           S
//           |               1
//           ^
//          | |              2
//          ^ ^
//         | | |             4 (routes leading to centre: 2, routes leading to edges: 1)
//         ^ | |
//        | || |
//
//
//

pub fn solve_part_2(puzzle_input: &str) -> Result<usize, ()> {
    let lines: Vec<&[u8]> = puzzle_input.lines().map(|l| l.as_bytes()).collect();
    let width = lines[0].len();

    let mut routes_leading_here: Vec<usize> = lines[0]
        .iter()
        .map(|b| if *b == b'S' { 1 } else { 0 })
        .collect();

    for row in 1..lines.len() {
        let mut next_routes_leading_here = vec![0_usize; width];

        for col in 0..width {
            if routes_leading_here[col] != 0 && lines[row][col] == b'^' {
                next_routes_leading_here[col - 1] += routes_leading_here[col];
                next_routes_leading_here[col + 1] += routes_leading_here[col];
            } else if routes_leading_here[col] != 0 && lines[row][col] == b'.' {
                next_routes_leading_here[col] += routes_leading_here[col];
            }
        }

        routes_leading_here = next_routes_leading_here;
    }

    Ok(routes_leading_here.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(solve_part_1(example_input), Ok(21));
    }

    #[test]
    fn test_example_part2() {
        let example_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(solve_part_2(example_input), Ok(40));
    }
}
