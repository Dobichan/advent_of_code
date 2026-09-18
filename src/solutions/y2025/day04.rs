use std::{collections::VecDeque, fmt::Display};

use crate::{AoCSolution, grid::Grid, point::Point};

#[derive(Debug, Default)]
pub struct Solution {}

fn is_roll(grid: &Grid, p: Point) -> bool {
    grid.get(p) == Some('@')
}

fn removable_paper_roll(grid: &Grid, p: Point) -> bool {
    Point::NEIGHBORS
        .iter()
        .filter(|&&d| is_roll(grid, p + d))
        .count()
        < 4
}

impl AoCSolution for Solution {
    type Parsed = Grid;

    fn parse(&self, input: &str) -> Self::Parsed {
        input.parse().expect("Failed to parse input grid")
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .filter(|&(p, r)| r == '@' && removable_paper_roll(data, p))
            .count()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut counts: Grid<u8> = Grid::new(data.width(), data.height(), 0);
        for p in data.find_all('@') {
            counts[p] = Point::NEIGHBORS
                .iter()
                .filter(|&&d| is_roll(data, p + d))
                .count() as u8;
        }

        let mut removed: Grid<bool> = Grid::new(data.width(), data.height(), false);
        let mut queue: VecDeque<Point> = data.find_all('@').filter(|&p| counts[p] < 4).collect();
        let mut total = 0;

        while let Some(p) = queue.pop_front() {
            if removed[p] {
                continue;
            }
            removed[p] = true;
            total += 1;

            for &d in &Point::NEIGHBORS {
                let q = p + d;
                if is_roll(data, q) && !removed[q] {
                    counts[q] -= 1;
                    if counts[q] == 3 {
                        queue.push_back(q);
                    }
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "13");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "43");
    }
}
