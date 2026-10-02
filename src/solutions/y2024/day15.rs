use std::fmt::Display;

use crate::{AoCSolution, direction::Direction, grid::Grid, point::Point};

#[derive(Default)]
pub struct Solution {}

pub struct ParseData {
    grid: Grid,
    instructions: Vec<Direction>,
    robot: Point,
}

fn shufle_crates(grid: &mut Grid, dir: Direction, robot: Point) {
    let next = robot + dir;
    if grid[next] != 'O' {
        return;
    }

    let mut p = next;
    loop {
        match grid[p] {
            '.' => {
                grid[p] = 'O';
                grid[next] = '.';
                return;
            }
            '#' => return,
            _ => p += dir,
        }
    }
}

fn move_robot(grid: &mut Grid, dir: Direction, robot: Point) -> Point {
    shufle_crates(grid, dir, robot);
    let next = robot + dir;
    if grid[next] == '.' { next } else { robot }
}

/// Builds the part 2 warehouse where every tile is twice as wide
fn widen(grid: &Grid) -> Grid {
    let mut wide = Grid::new(grid.width() * 2, grid.height(), '.');
    for p in grid.positions() {
        let (left, right) = match grid[p] {
            '#' => ('#', '#'),
            'O' => ('[', ']'),
            _ => ('.', '.'),
        };
        let l = Point::new(p.x * 2, p.y);
        wide[l] = left;
        wide[l + Direction::East] = right;
    }
    wide
}

/// `l` is the position of the crate's left half, '['
fn can_crate_move(grid: &Grid, dir: Direction, l: Point) -> bool {
    match dir {
        Direction::West => match grid[l + Point::new(-1, 0)] {
            '.' => true,
            ']' => can_crate_move(grid, dir, l + Point::new(-2, 0)),
            _ => false,
        },
        Direction::East => match grid[l + Point::new(2, 0)] {
            '.' => true,
            '[' => can_crate_move(grid, dir, l + Point::new(2, 0)),
            _ => false,
        },
        Direction::North | Direction::South => {
            // The two tiles in front of the crate
            let a = l + dir;
            match (grid[a], grid[a + Direction::East]) {
                ('.', '.') => true,
                ('[', _) => can_crate_move(grid, dir, a),
                (']', '.') => can_crate_move(grid, dir, a + Direction::West),
                ('.', '[') => can_crate_move(grid, dir, a + Direction::East),
                (']', '[') => {
                    let crate_a_moveable = can_crate_move(grid, dir, a + Direction::West);
                    let crate_b_moveable = can_crate_move(grid, dir, a + Direction::East);
                    crate_a_moveable && crate_b_moveable
                }
                _ => false,
            }
        }
    }
}

fn move_crate(grid: &mut Grid, dir: Direction, l: Point) {
    match dir {
        Direction::West => {
            if grid[l + Point::new(-1, 0)] == ']' {
                move_crate(grid, dir, l + Point::new(-2, 0));
            }
            grid[l + Point::new(-1, 0)] = '[';
            grid[l] = ']';
            grid[l + Point::new(1, 0)] = '.';
        }
        Direction::East => {
            if grid[l + Point::new(2, 0)] == '[' {
                move_crate(grid, dir, l + Point::new(2, 0));
            }
            grid[l + Point::new(1, 0)] = '[';
            grid[l + Point::new(2, 0)] = ']';
            grid[l] = '.';
        }
        Direction::North | Direction::South => {
            // The two tiles in front of the crate
            let a = l + dir;
            match (grid[a], grid[a + Direction::East]) {
                ('[', _) => move_crate(grid, dir, a),
                (']', '.') => move_crate(grid, dir, a + Direction::West),
                ('.', '[') => move_crate(grid, dir, a + Direction::East),
                (']', '[') => {
                    move_crate(grid, dir, a + Direction::West);
                    move_crate(grid, dir, a + Direction::East);
                }
                _ => {}
            }

            if grid[a] == '.' && grid[a + Direction::East] == '.' {
                grid[a] = '[';
                grid[a + Direction::East] = ']';
                grid[l] = '.';
                grid[l + Direction::East] = '.';
            }
        }
    }
}

fn move_robot_2(grid: &mut Grid, dir: Direction, robot: Point) -> Point {
    let next = robot + dir;
    let crate_left = match grid[next] {
        '[' => Some(next),
        ']' => Some(next + Direction::West),
        _ => None,
    };
    if let Some(l) = crate_left
        && can_crate_move(grid, dir, l)
    {
        move_crate(grid, dir, l);
    }

    if grid[next] == '.' { next } else { robot }
}

impl AoCSolution for Solution {
    type Parsed = ParseData;

    fn parse(&self, input: &str) -> Self::Parsed {
        let (grid, instructions) = input
            .trim()
            .split_once("\n\n")
            .expect("Expected a blank line between the grid and the instructions");

        let mut grid: Grid = grid.parse().expect("Failed to parse the warehouse grid");
        let instructions = instructions
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| Direction::try_from(c).unwrap())
            .collect();

        let robot = grid.find('@').expect("Robot is missing from the grid");
        grid[robot] = '.';

        ParseData {
            grid,
            instructions,
            robot,
        }
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut grid = data.grid.clone();
        let mut robot = data.robot;

        for &dir in &data.instructions {
            robot = move_robot(&mut grid, dir, robot);
        }

        grid.find_all('O').map(|p| 100 * p.y + p.x).sum::<isize>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut grid = widen(&data.grid);
        let mut robot = Point::new(data.robot.x * 2, data.robot.y);

        for &dir in &data.instructions {
            robot = move_robot_2(&mut grid, dir, robot);
        }

        grid.find_all('[').map(|p| 100 * p.y + p.x).sum::<isize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const LARGER_EXAMPLE: &str = r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "2028");
    }

    #[test]
    fn test_part1_larger() {
        assert_eq!(Solution::default().solve1(LARGER_EXAMPLE), "10092");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(LARGER_EXAMPLE), "9021");
    }
}
