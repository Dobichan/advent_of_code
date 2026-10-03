use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    fmt::Display,
};

use crate::{AoCSolution, direction::Direction, grid::Grid, point::Point};

#[derive(Default)]
pub struct Solution {}

pub struct ParseData {
    maze: Grid,
    start: Point,
    end: Point,
}

impl ParseData {
    fn index(&self, pos: Point, dir: Direction) -> usize {
        (pos.y as usize * self.maze.width() + pos.x as usize) * 4 + dir as usize
    }

    fn state_count(&self) -> usize {
        self.maze.width() * self.maze.height() * 4
    }
}

impl AoCSolution for Solution {
    type Parsed = ParseData;

    fn parse(&self, input: &str) -> Self::Parsed {
        let grid: Grid = input.parse().unwrap();

        let start = grid.find('S').expect("Start position not found!");
        let end = grid.find('E').expect("End position not found!");

        ParseData {
            maze: grid,
            start,
            end,
        }
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut lowest_cost = vec![isize::MAX; data.state_count()];
        let mut next_steps: BinaryHeap<Reverse<(isize, Point, Direction)>> = BinaryHeap::new();

        lowest_cost[data.index(data.start, Direction::East)] = 0;
        next_steps.push(Reverse((0, data.start, Direction::East)));

        while let Some(Reverse((cost, pos, dir))) = next_steps.pop() {
            // Stale entry, a cheaper way here was already found
            if cost > lowest_cost[data.index(pos, dir)] {
                continue;
            }

            if pos == data.end {
                return cost;
            }

            let mut options = vec![];

            // Go forward if ok
            let next = pos + dir.delta();
            if data.maze[next] != '#' {
                options.push((cost + 1, next, dir));
            }

            // Try both sides also
            options.push((cost + 1000, pos, dir.turn_left()));
            options.push((cost + 1000, pos, dir.turn_right()));

            for (new_cost, new_pos, new_dir) in options {
                let i = data.index(new_pos, new_dir);
                if new_cost < lowest_cost[i] {
                    lowest_cost[i] = new_cost;
                    next_steps.push(Reverse((new_cost, new_pos, new_dir)));
                }
            }
        }

        0
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut best_cost = isize::MAX;
        let mut lowest_cost = vec![isize::MAX; data.state_count()];
        let mut ends = Vec::new();
        let mut next_steps: BinaryHeap<Reverse<(isize, Point, Direction)>> = BinaryHeap::new();

        lowest_cost[data.index(data.start, Direction::East)] = 0;
        next_steps.push(Reverse((0, data.start, Direction::East)));

        while let Some(Reverse((cost, pos, dir))) = next_steps.pop() {
            // Stale entry, a cheaper way here was already found
            if cost > lowest_cost[data.index(pos, dir)] {
                continue;
            }

            if pos == data.end {
                if cost > best_cost {
                    break;
                }
                best_cost = cost;
                ends.push((pos, dir));
            }

            let mut options = vec![];

            // Go forward if ok
            let next = pos + dir.delta();
            if data.maze[next] != '#' {
                options.push((cost + 1, next, dir));
            }

            // Try both sides also
            options.push((cost + 1000, pos, dir.turn_left()));
            options.push((cost + 1000, pos, dir.turn_right()));

            for (new_cost, new_pos, new_dir) in options {
                let i = data.index(new_pos, new_dir);
                if new_cost < lowest_cost[i] {
                    lowest_cost[i] = new_cost;
                    next_steps.push(Reverse((new_cost, new_pos, new_dir)));
                }
            }
        }

        let mut backtracking: VecDeque<_> = ends.iter().copied().collect();
        let mut visited = vec![false; data.state_count()];
        let mut tiles = vec![false; data.maze.width() * data.maze.height()];

        for &(pos, dir) in &ends {
            visited[data.index(pos, dir)] = true;
        }

        while let Some((pos, dir)) = backtracking.pop_front() {
            let i = data.index(pos, dir);
            // 4 states per cell, so dividing by 4 gives the cell
            tiles[i / 4] = true;
            let cost = lowest_cost[i];

            // A state is a predecessor if its cost plus the move cost equals ours.
            // Walls are never reached, so their cost never matches.
            let prevs = [
                (cost - 1, pos - dir.delta(), dir),
                (cost - 1000, pos, dir.turn_left()),
                (cost - 1000, pos, dir.turn_right()),
            ];

            for (prev_cost, prev_pos, prev_dir) in prevs {
                let j = data.index(prev_pos, prev_dir);
                if lowest_cost[j] == prev_cost && !visited[j] {
                    visited[j] = true;
                    backtracking.push_back((prev_pos, prev_dir));
                }
            }
        }

        tiles.iter().filter(|&&on_path| on_path).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: &str = r"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const EXAMPLE_INPUT2: &str = r"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn test_part1_1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT1), "7036");
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT2), "11048");
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT1), "45");
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT2), "64");
    }
}
