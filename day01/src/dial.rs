use crate::rotation::Rotation;

pub struct Dial {
    position: usize,
    size: usize,
    password: usize,
}

impl Dial {
    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::L { count } => {
                let actual_rotation = count % (self.size + 1);
                if actual_rotation > self.position {
                    self.position = self.size - (actual_rotation - self.position) + 1;
                } else {
                    self.position -= actual_rotation;
                }
            }
            Rotation::R { count } => {
                let actual_rotation = count % (self.size + 1);

                if self.position + actual_rotation > self.size {
                    self.position = actual_rotation - (self.size - self.position) - 1;
                } else {
                    self.position += actual_rotation
                }
            }
        }
        if self.position == 0 {
            self.password += 1;
        }
    }

    pub fn get_password(&self) -> usize {
        self.password
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            size: 99,
            position: 50,
            password: 0,
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
    }

    #[test]
    fn rotate_left_without_passing_zero() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::L { count: 5 });
        assert_eq!(dial.position, 45);
    }

    #[test]
    fn rotate_left_passing_zero() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::L { count: 68 });
        assert_eq!(dial.position, 82);
    }

    #[test]
    fn rotate_left_landing_at_zero() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::R { count: 50 });
        assert_eq!(dial.position, 0);
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
    fn test_example() {
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

        assert_eq!(dial.get_password(), 3);
    }
}
