use std::fmt::Display;

use crate::{AoCSolution, direction::Direction, grid::Grid, point::Point};

pub struct ParseData {
    world: Grid,
    start: Point,
}

#[derive(Clone, Debug)]
struct Guard {
    position: Point,
    direction: Direction,
    seen: Vec<bool>,
    repeating: bool,
}

impl Guard {
    fn new(world: &Grid, start: Point) -> Self {
        Self {
            position: start,
            direction: Direction::North,
            seen: vec![false; world.width() * world.height() * 4],
            repeating: false,
        }
    }

    fn step(&mut self, world: &Grid) {
        let seen_idx = seen_index(world, self.position, self.direction);
        if self.seen[seen_idx] {
            self.repeating = true;
            return;
        }

        self.seen[seen_idx] = true;
        let delta_point = self.direction.delta();
        let next = self.position + delta_point;
        match world.get(next) {
            Some('#') => self.direction = self.direction.turn_right(),
            _ => self.position = next,
        }
    }

    fn walk(world: &Grid, start: Point) -> Guard {
        let mut guard = Self::new(world, start);
        while world.in_bounds(guard.position) && !guard.repeating {
            guard.step(world)
        }
        guard
    }

    fn has_visited(&self, world: &Grid, pos: Point) -> bool {
        Direction::ALL
            .iter()
            .any(|&dir| self.seen[seen_index(world, pos, dir)])
    }

    fn visited(&self, world: &Grid) -> Vec<Point> {
        world
            .iter()
            .map(|(pos, _)| pos)
            .filter(|&pos| self.has_visited(world, pos))
            .collect()
    }
}

fn seen_index(world: &Grid, pos: Point, dir: Direction) -> usize {
    (pos.y as usize * world.width() + pos.x as usize) * 4 + dir as usize
}

#[derive(Debug, Default)]
pub struct Solution {}

impl AoCSolution for Solution {
    type Parsed = ParseData;

    fn parse(&self, input: &str) -> Self::Parsed {
        let world: Grid = input.parse().expect("Failed to parse the world grid");
        let start = world
            .find('^')
            .expect("Initital guard position is missing.");

        ParseData { world, start }
    }
    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let guard = Guard::walk(&data.world, data.start);
        guard.visited(&data.world).len()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let candidates = Guard::walk(&data.world, data.start).visited(&data.world);
        let mut world = data.world.clone();
        let mut ret = 0;

        for p in candidates {
            if p == data.start {
                continue;
            }

            world.set(p, '#');
            if Guard::walk(&world, data.start).repeating {
                ret += 1;
            }
            world.set(p, '.');
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "41");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "6");
    }
}
