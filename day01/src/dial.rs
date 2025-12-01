use num::traits::Euclid;

use crate::rotation::Rotation;

pub struct Dial {
    position: usize,
    size: usize,
    password: usize,
    rotations: usize,
}

impl Dial {
    pub fn rotate(&mut self, rotation: Rotation) {
        let steps = match rotation {
            Rotation::L { count } => count,
            Rotation::R { count } => count,
        };

        let (full_rotations, remaining_steps) = Euclid::div_rem_euclid(&steps, &(self.size + 1));

        // Full rotations does not happen in the provided example but will probably happen with
        // the given input file.
        self.rotations += full_rotations;

        match rotation {
            Rotation::L { count: _ } => {
                if remaining_steps > self.position {
                    if self.position != 0 {
                        self.rotations += 1;
                    }
                    self.position = self.size - (remaining_steps - self.position) + 1;
                } else {
                    self.position -= remaining_steps;
                }
            }
            Rotation::R { count: _ } => {
                if self.position + remaining_steps > self.size {
                    self.position = remaining_steps - (self.size - self.position) - 1;
                    if self.position != 0 {
                        self.rotations += 1;
                    }
                } else {
                    self.position += remaining_steps
                }
            }
        }
        if self.position == 0 {
            self.password += 1;
        }
    }

    pub fn get_part1_password(&self) -> usize {
        self.password
    }

    pub fn get_part2_password(&self) -> usize {
        self.rotations + self.password
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            size: 99,
            position: 50,
            password: 0,
            rotations: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_left_99_to_0() {
        let mut dial = Dial::default();
        dial.position = 99;
        dial.rotate(Rotation::L { count: 99 });
        assert_eq!(dial.position, 0);
        assert_eq!(dial.get_part2_password(), 1);
    }

    #[test]
    fn rotate_left_without_passing_zero() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::L { count: 5 });
        assert_eq!(dial.position, 45);
        assert_eq!(dial.get_part2_password(), 0);
    }

    #[test]
    fn rotate_left_passing_zero() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::L { count: 68 });
        assert_eq!(dial.position, 82);
        assert_eq!(dial.get_part2_password(), 1);
    }

    #[test]
    fn rotate_left_landing_at_zero() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::L { count: 50 });
        assert_eq!(dial.position, 0);
        assert_eq!(dial.get_part2_password(), 1);
    }

    #[test]
    fn rotate_right_without_passing_zero() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::R { count: 5 });
        assert_eq!(dial.position, 55);

        let mut dial = Dial::default();
        dial.rotate(Rotation::R { count: 49 });
        assert_eq!(dial.position, 99);
    }

    #[test]
    fn rotate_right_passing_zero() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::R { count: 55 });
        assert_eq!(dial.position, 5);
    }

    #[test]
    fn rotate_right_landing_at_zero() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::R { count: 50 });
        assert_eq!(dial.position, 0);
    }

    #[test]
    fn rotate_right_0_to_99() {
        let mut dial = Dial::default();
        dial.position = 0;
        dial.rotate(Rotation::R { count: 99 });
        assert_eq!(dial.position, 99);
    }

    #[test]
    fn test_part1_example() {
        let mut dial = Dial::default();

        dial.rotate(Rotation::L { count: 68 });
        assert_eq!(dial.position, 82);

        dial.rotate(Rotation::L { count: 30 });
        assert_eq!(dial.position, 52);

        dial.rotate(Rotation::R { count: 48 });
        assert_eq!(dial.position, 0);

        dial.rotate(Rotation::L { count: 5 });
        assert_eq!(dial.position, 95);

        dial.rotate(Rotation::R { count: 60 });
        assert_eq!(dial.position, 55);

        dial.rotate(Rotation::L { count: 55 });
        assert_eq!(dial.position, 0);

        dial.rotate(Rotation::L { count: 1 });
        assert_eq!(dial.position, 99);

        dial.rotate(Rotation::L { count: 99 });
        assert_eq!(dial.position, 0);

        dial.rotate(Rotation::R { count: 14 });
        assert_eq!(dial.position, 14);

        dial.rotate(Rotation::L { count: 82 });
        assert_eq!(dial.position, 32);

        assert_eq!(dial.get_part1_password(), 3);
    }

    #[test]
    fn test_part2_example() {
        let mut dial = Dial::default();

        dial.rotate(Rotation::L { count: 68 });
        assert_eq!(dial.position, 82);

        dial.rotate(Rotation::L { count: 30 });
        assert_eq!(dial.position, 52);

        dial.rotate(Rotation::R { count: 48 });
        assert_eq!(dial.position, 0);

        dial.rotate(Rotation::L { count: 5 });
        assert_eq!(dial.position, 95);

        dial.rotate(Rotation::R { count: 60 });
        assert_eq!(dial.position, 55);

        dial.rotate(Rotation::L { count: 55 });
        assert_eq!(dial.position, 0);

        dial.rotate(Rotation::L { count: 1 });
        assert_eq!(dial.position, 99);

        dial.rotate(Rotation::L { count: 99 });
        assert_eq!(dial.position, 0);

        dial.rotate(Rotation::R { count: 14 });
        assert_eq!(dial.position, 14);

        dial.rotate(Rotation::L { count: 82 });
        assert_eq!(dial.position, 32);

        assert_eq!(dial.get_part2_password(), 6);
    }

    #[test]
    fn test_part2_example_2() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::L { count: 1000 });
        assert_eq!(dial.get_part2_password(), 10);
        assert_eq!(dial.position, 50);

        let mut dial = Dial::default();
        dial.rotate(Rotation::R { count: 1000 });
        assert_eq!(dial.get_part2_password(), 10);
        assert_eq!(dial.position, 50);
    }
}
