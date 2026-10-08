use std::{collections::HashMap, fmt::Display, iter::once};

use crate::{AoCSolution, direction::Direction, parsing::input_lines};

#[derive(Default)]
pub struct Solution {}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

#[derive(Debug)]
pub struct Keypad {
    keys: String,
    unlock_code: u64,
}

impl Keypad {
    fn position_of(key: char) -> (u8, u8) {
        match key {
            '0' => (3, 1),
            '1' => (2, 0),
            '2' => (2, 1),
            '3' => (2, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            'A' => (3, 2),
            _ => panic!("Illegal keypad key {key}"),
        }
    }

    fn paths(from: (u8, u8), to: (u8, u8)) -> Vec<Vec<Direction>> {
        const GAP: (u8, u8) = (3, 0);

        let vertical_dir = if to.0 > from.0 {
            Direction::South
        } else {
            Direction::North
        };
        let horizontal_dir = if to.1 > from.1 {
            Direction::East
        } else {
            Direction::West
        };

        let verticals = vec![vertical_dir; from.0.abs_diff(to.0) as usize];
        let horizontals = vec![horizontal_dir; from.1.abs_diff(to.1) as usize];

        let mut paths = Vec::new();

        // Check horizontal first
        if (from.0, to.1) != GAP {
            paths.push([horizontals.as_slice(), verticals.as_slice()].concat());
        }

        // Check vertical path
        if !horizontals.is_empty() && !verticals.is_empty() && (to.0, from.1) != GAP {
            paths.push([verticals.as_slice(), horizontals.as_slice()].concat());
        }

        paths
    }
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
#[derive(Debug)]
pub struct DirectionPad {}

impl DirectionPad {
    fn position_of(key: char) -> (u8, u8) {
        match key {
            'A' => (0, 2),
            '^' => (0, 1),
            '<' => (1, 0),
            'v' => (1, 1),
            '>' => (1, 2),
            _ => panic!("Illegal direction pad key {key}"),
        }
    }

    fn paths(from: (u8, u8), to: (u8, u8)) -> Vec<Vec<Direction>> {
        const GAP: (u8, u8) = (0, 0);

        let vertical_dir = if to.0 > from.0 {
            Direction::South
        } else {
            Direction::North
        };
        let horizontal_dir = if to.1 > from.1 {
            Direction::East
        } else {
            Direction::West
        };

        let verticals = vec![vertical_dir; from.0.abs_diff(to.0) as usize];
        let horizontals = vec![horizontal_dir; from.1.abs_diff(to.1) as usize];

        let mut paths = Vec::new();

        // Check horizontal first
        if (from.0, to.1) != GAP {
            paths.push([horizontals.as_slice(), verticals.as_slice()].concat());
        }

        // Check vertical path
        if !horizontals.is_empty() && !verticals.is_empty() && (to.0, from.1) != GAP {
            paths.push([verticals.as_slice(), horizontals.as_slice()].concat());
        }

        paths
    }
}

fn dir_key(d: &Direction) -> char {
    match d {
        Direction::North => '^',
        Direction::East => '>',
        Direction::South => 'v',
        Direction::West => '<',
    }
}

/// Calculates the cost of going a path followed by 'A' press at `depth`.
/// Assumes the pointer starts at 'A'.
fn dir_pad_cost(
    path: &[Direction],
    depth: usize,
    cache: &mut HashMap<(char, char, usize), u64>,
) -> u64 {
    let mut total = 0;

    let mut prev = 'A';

    for key in path.iter().map(dir_key).chain(once('A')) {
        total += direction_cost(prev, key, depth, cache);
        prev = key;
    }

    total
}

fn direction_cost(
    from: char,
    to: char,
    depth: usize,
    cache: &mut HashMap<(char, char, usize), u64>,
) -> u64 {
    if depth == 0 {
        return 1;
    }
    if let Some(&cost) = cache.get(&(from, to, depth)) {
        return cost;
    }

    let best = DirectionPad::paths(
        DirectionPad::position_of(from),
        DirectionPad::position_of(to),
    )
    .iter()
    .map(|path| dir_pad_cost(path, depth - 1, cache))
    .min()
    .unwrap();

    cache.insert((from, to, depth), best);
    best
}

fn keypad_code_cost(
    code: &str,
    depth: usize,
    cache: &mut HashMap<(char, char, usize), u64>,
) -> u64 {
    let mut total = 0;
    let mut prev = 'A';
    for ch in code.chars() {
        total += Keypad::paths(Keypad::position_of(prev), Keypad::position_of(ch))
            .iter()
            .map(|path| dir_pad_cost(path, depth, cache))
            .min()
            .unwrap();
        prev = ch;
    }

    total
}

impl AoCSolution for Solution {
    type Parsed = Vec<Keypad>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input_lines(input)
            .map(|l| Keypad {
                keys: l.to_string(),
                unlock_code: l[..3].parse::<u64>().unwrap(),
            })
            .collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut ret = 0;
        let mut cache = HashMap::new();
        for keypad in data {
            let min_cost = keypad_code_cost(&keypad.keys, 2, &mut cache);
            ret += min_cost * keypad.unlock_code
        }
        ret
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut ret = 0;
        let mut cache = HashMap::new();
        for keypad in data {
            let min_cost = keypad_code_cost(&keypad.keys, 25, &mut cache);
            ret += min_cost * keypad.unlock_code
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
029A
980A
179A
456A
379A
";

    #[test]
    fn check_keypads() {
        assert_eq!(
            Keypad::paths((3, 1), (0, 0)),
            [[
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::West
            ]]
        );

        assert_eq!(
            Keypad::paths((1, 0), (3, 2)),
            [[
                Direction::East,
                Direction::East,
                Direction::South,
                Direction::South
            ]]
        );

        assert_eq!(
            Keypad::paths((3, 2), (1, 1)),
            [
                [Direction::West, Direction::North, Direction::North,],
                [Direction::North, Direction::North, Direction::West]
            ]
        );
    }

    #[test]
    fn test_direction_pad() {
        assert_eq!(
            DirectionPad::paths((0, 2), (1, 1)),
            [
                [Direction::West, Direction::South],
                [Direction::South, Direction::West]
            ]
        );

        assert_eq!(
            DirectionPad::paths((0, 2), (1, 0)),
            [[Direction::South, Direction::West, Direction::West]]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "126384");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "154115708116294");
    }
}
