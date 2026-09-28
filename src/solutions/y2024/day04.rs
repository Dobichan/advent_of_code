use std::fmt::Display;

use crate::AoCSolution;
use crate::grid::Grid;
use crate::point::Point;

const MAS: [char; 3] = ['M', 'A', 'S'];

fn check_xmas(grid: &Grid, dir: Point, start: Point) -> bool {
    let end = start + dir * 3;
    if !grid.in_bounds(end) {
        return false;
    }
    for (i, &c) in MAS.iter().enumerate() {
        let p = start + dir * (i as isize + 1);
        if grid[p] != c {
            return false;
        }
    }
    true
}

fn is_ms(a: char, b: char) -> bool {
    matches!((a, b), ('M', 'S') | ('S', 'M'))
}

fn check_mas(grid: &Grid, center: Point) -> bool {
    let [ur, dr, dl, ul] = Point::DIAGONAL.map(|d| grid[center + d]);
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
        let (w, h) = (data.width() as isize, data.height() as isize);
        (1..h - 1)
            .flat_map(|y| (1..w - 1).map(move |x| Point::new(x, y)))
            .filter(|&p| data[p] == 'A' && check_mas(data, p))
            .count()
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
