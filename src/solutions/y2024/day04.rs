use std::fmt::Display;

use crate::AoCSolution;
use crate::grid::Grid;
use crate::point::Point;

const MAS: [char; 3] = ['M', 'A', 'S'];

fn check_xmas(grid: &Grid, dir: Point, start: Point) -> bool {
    MAS.iter()
        .enumerate()
        .all(|(i, &c)| grid.get(start + dir * (i as isize + 1)) == Some(c))
}

fn is_ms(a: Option<char>, b: Option<char>) -> bool {
    matches!((a, b), (Some('M'), Some('S')) | (Some('S'), Some('M')))
}

fn check_mas(grid: &Grid, center: Point) -> bool {
    let [ur, dr, dl, ul] = Point::DIAGONAL.map(|d| grid.get(center + d));
    is_ms(ul, dr) && is_ms(ur, dl)
}

#[derive(Debug, Default)]
pub struct Solution {}

impl AoCSolution for Solution {
    type Parsed = Grid;

    fn parse(&self, input: &str) -> Self::Parsed {
        input.parse().expect("Failed to parse grid")
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.find_all('X')
            .map(|p| {
                Point::NEIGHBORS
                    .iter()
                    .filter(|&&d| check_xmas(data, d, p))
                    .count()
            })
            .sum::<usize>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        data.find_all('A').filter(|&p| check_mas(data, p)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "18");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "9");
    }
}
