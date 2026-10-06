use std::fmt::Display;

use crate::{AoCSolution, grid::Grid, point::Point};

#[derive(Default)]
pub struct Solution {}

#[derive(Clone, Debug)]
pub struct ParseData {
    grid: Grid,
    start: Point,
}

fn cheat_offsets(max_steps: isize) -> impl Iterator<Item = Point> {
    (-max_steps..=max_steps)
        .flat_map(move |dy| (-max_steps..=max_steps).map(move |dx| Point::new(dx, dy)))
        .filter(move |p| {
            let steps = p.manhattan_distance(Point::ORIGIN);
            steps >= 2 && steps <= max_steps
        })
}

impl AoCSolution for Solution {
    type Parsed = ParseData;

    fn parse(&self, input: &str) -> Self::Parsed {
        let grid: Grid = input.parse().expect("Failed to parse input grid");
        let start = grid.find('S').expect("Start Point not found");

        Self::Parsed { grid, start }
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let dist = data.grid.distances(data.start, |e| e != '#');

        let mut count = 0;
        for p in data.grid.positions() {
            let Some(distance_at_p) = dist[p] else {
                continue;
            };
            for offset in cheat_offsets(2) {
                let cheated_pos = p + offset;

                // Check that a cheat ends at a valid path
                let Some(Some(cheat_distance)) = dist.get(cheated_pos) else {
                    continue;
                };

                let savings = cheat_distance as isize
                    - distance_at_p as isize
                    - p.manhattan_distance(cheated_pos);

                if savings >= 100 {
                    count += 1;
                }
            }
        }
        count
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let dist = data.grid.distances(data.start, |e| e != '#');

        let mut count = 0;
        for p in data.grid.positions() {
            let Some(distance_at_p) = dist[p] else {
                continue;
            };
            for offset in cheat_offsets(20) {
                let cheated_pos = p + offset;

                // Check that a cheat ends at a valid path
                let Some(Some(cheat_distance)) = dist.get(cheated_pos) else {
                    continue;
                };

                let savings = cheat_distance as isize
                    - distance_at_p as isize
                    - p.manhattan_distance(cheated_pos);

                if savings >= 100 {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "0");
    }
}
