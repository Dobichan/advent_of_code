use std::{collections::HashSet, fmt::Display};

use crate::{AoCSolution, direction::Direction, point::Point};

#[derive(Debug, Default)]
pub struct Solution {}

fn visited(moves: impl Iterator<Item = Direction>) -> HashSet<Point> {
    let mut pos = Point::ORIGIN;
    let mut houses = HashSet::from([pos]);

    for dir in moves {
        pos += dir;
        houses.insert(pos);
    }

    houses
}

impl AoCSolution for Solution {
    type Parsed = Vec<Direction>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input
            .trim()
            .chars()
            .map(|c| Direction::try_from(c).expect("Illegal character in input"))
            .collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        visited(data.iter().copied()).len()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let santa = visited(data.iter().copied().step_by(2));
        let robo = visited(data.iter().copied().skip(1).step_by(2));
        santa.union(&robo).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sol = Solution::default();

        const INPUT1: &str = ">";
        assert_eq!(sol.solve1(INPUT1), "2");

        const INPUT2: &str = "^>v<";
        assert_eq!(sol.solve1(INPUT2), "4");

        const INPUT3: &str = "^v^v^v^v^v";
        assert_eq!(sol.solve1(INPUT3), "2");
    }

    #[test]
    fn test_part2() {
        let sol = Solution::default();

        const INPUT1: &str = "^v";
        assert_eq!(sol.solve2(INPUT1), "3");

        const INPUT2: &str = "^>v<";
        assert_eq!(sol.solve2(INPUT2), "3");

        const INPUT3: &str = "^v^v^v^v^v";
        assert_eq!(sol.solve2(INPUT3), "11");
    }
}
