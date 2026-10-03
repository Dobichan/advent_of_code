use crate::point::Point;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const ALL: [Direction; 4] = [Self::North, Self::East, Self::South, Self::West];

    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    /// returns a point with a unit step in it's direction
    pub const fn delta(self) -> Point {
        match self {
            Direction::North => Point::new(0, -1),
            Direction::East => Point::new(1, 0),
            Direction::South => Point::new(0, 1),
            Direction::West => Point::new(-1, 0),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            '^' | 'N' | 'U' => Ok(Direction::North),
            '>' | 'E' | 'R' => Ok(Direction::East),
            'V' | 'S' | 'D' => Ok(Direction::South),
            '<' | 'W' | 'L' => Ok(Direction::West),
            _ => Err(format!(
                "Illegal character to create a direction from {value}"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_right_cycle() {
        for dir in Direction::ALL {
            assert_eq!(dir.turn_right().turn_right().turn_right().turn_right(), dir);
        }
    }

    #[test]
    fn test_turn_left_undoes_turn_right() {
        for dir in Direction::ALL {
            assert_eq!(dir.turn_right().turn_left(), dir);
        }
    }

    #[test]
    fn test_reverse_twice_is_identity() {
        for dir in Direction::ALL {
            assert_eq!(dir.reverse().reverse(), dir);
            assert_eq!(dir.reverse(), dir.turn_right().turn_right());
        }
    }

    #[test]
    fn test_delta() {
        assert_eq!(Direction::North.delta(), Point::new(0, -1));
        assert_eq!(Direction::East.delta(), Point::new(1, 0));
        assert_eq!(Direction::South.delta(), Point::new(0, 1));
        assert_eq!(Direction::West.delta(), Point::new(-1, 0));
    }

    #[test]
    fn test_reverse_delta_is_negated() {
        for dir in Direction::ALL {
            assert_eq!(dir.reverse().delta(), -dir.delta());
        }
    }

    #[test]
    fn test_try_from_char() {
        for (chars, expected) in [
            ("^NUnu", Direction::North),
            (">ERer", Direction::East),
            ("vVSDsd", Direction::South),
            ("<WLwl", Direction::West),
        ] {
            for c in chars.chars() {
                assert_eq!(Direction::try_from(c), Ok(expected), "char {c:?}");
            }
        }
    }

    #[test]
    fn test_try_from_invalid_char() {
        assert!(Direction::try_from('x').is_err());
        assert!(Direction::try_from('.').is_err());
    }

    #[test]
    fn test_all_is_clockwise_and_distinct() {
        for pair in Direction::ALL.windows(2) {
            assert_eq!(pair[0].turn_right(), pair[1]);
        }
        assert_eq!(
            Direction::ALL.last().unwrap().turn_right(),
            Direction::ALL[0]
        );
    }
}
