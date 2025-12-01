use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Rotation {
    L { count: usize },
    R { count: usize },
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRotationError<'a> {
    original_line: &'a str,
}

impl Display for ParseRotationError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to parse line {} as a rotation",
            self.original_line
        )
    }
}

impl<'a> TryFrom<&'a str> for Rotation {
    type Error = ParseRotationError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();
        let rotation = chars.next();

        match rotation {
            Some('L') => match usize::from_str(chars.as_str()) {
                Ok(count) => Ok(Rotation::L { count }),
                _ => Err(Self::Error {
                    original_line: &value,
                }),
            },
            Some('R') => match usize::from_str(chars.as_str()) {
                Ok(count) => Ok(Rotation::R { count }),
                _ => Err(Self::Error {
                    original_line: &value,
                }),
            },
            _ => Err(Self::Error {
                original_line: &value,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_l_rotation() {
        let result = Rotation::try_from("L13");
        assert_eq!(result, Ok(Rotation::L { count: 13 }));
    }

    #[test]
    fn parse_valid_r_rotation() {
        let result = Rotation::try_from("R6");
        assert_eq!(result, Ok(Rotation::R { count: 6 }));
    }

    #[test]
    fn parse_invalid_rotations() {
        let errors = [
            Rotation::try_from("r6"),
            Rotation::try_from("R-6"),
            Rotation::try_from("z1"),
            Rotation::try_from("L"),
            Rotation::try_from("1"),
        ];
        assert!(errors.iter().all(|r| r.is_err()));
    }
}
