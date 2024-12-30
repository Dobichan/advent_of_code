use std::{
    backtrace,
    collections::{HashMap, VecDeque},
    i32, usize,
};

use priority_queue::PriorityQueue;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    println!("Part2!");

    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for (r, line) in input.lines().enumerate() {
        let mut grid_row = vec![];
        for (c, ch) in line.chars().enumerate() {
            if ch == 'E' {
                end = (r, c);
                grid_row.push('.');
                continue;
            }
            if ch == 'S' {
                start = (r, c);
            }
            grid_row.push(ch);
        }
        grid.push(grid_row);
    }

    (grid, start, end)
}

fn part2(input: &str) -> i32 {
    let (grid, start, end) = parse(input);

    get_path(&grid, &start, &end)
}

fn get_path(grid: &Vec<Vec<char>>, start: &(usize, usize), end: &(usize, usize)) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut best_cost = i32::MAX;
    let mut lowest_cost = HashMap::new();
    let mut ends = HashMap::new();
    let mut backtrack_map: HashMap<(usize, usize, Direction), Vec<(usize, usize, Direction)>> =
        HashMap::new();
    let mut next_steps: PriorityQueue<(usize, usize, Direction), i32> = PriorityQueue::new();

    next_steps.push((start.0, start.1, Direction::East), 0);

    while let Some(((r, c, dir), cost)) = next_steps.pop() {
        if (r, c) == *end {
            if -cost > best_cost {
                break;
            }
            best_cost = -cost;
            ends.insert((r, c, dir.clone()), best_cost);
        }

        let mut options = vec![];

        // Optional paths
        match dir {
            Direction::East => {
                // going east
                if grid[r][c + 1] != '#' {
                    options.push((r, c + 1, Direction::East, cost - 1));
                }
                // turn north
                options.push((r, c, Direction::North, cost - 1000));
                // turn south
                options.push((r, c, Direction::South, cost - 1000));
            }
            Direction::North => {
                // going north
                if grid[r - 1][c] != '#' {
                    options.push((r - 1, c, Direction::North, cost - 1));
                }
                // turn east
                options.push((r, c, Direction::East, cost - 1000));
                // turn west
                options.push((r, c, Direction::West, cost - 1000));
            }
            Direction::South => {
                // going south
                if grid[r + 1][c] != '#' {
                    options.push((r + 1, c, Direction::South, cost - 1));
                }
                // turn west
                options.push((r, c, Direction::West, cost - 1000));
                // turn east
                options.push((r, c, Direction::East, cost - 1000));
            }
            Direction::West => {
                // going west
                if grid[r][c - 1] != '#' {
                    options.push((r, c - 1, Direction::West, cost - 1));
                }
                // turn south
                options.push((r, c, Direction::South, cost - 1000));
                // going north
                options.push((r, c, Direction::North, cost - 1000));
            }
        }
        for (new_row, new_col, new_dir, new_cost) in options {
            let key = (new_row, new_col, new_dir.clone());
            let this_lowest = lowest_cost
                .get(&(new_row, new_col, new_dir.clone()))
                .copied()
                .unwrap_or(i32::MIN);
            if new_cost < this_lowest {
                continue;
            }

            if new_cost > this_lowest {
                lowest_cost.insert(key.clone(), new_cost);

                if backtrack_map.contains_key(&key) {
                    backtrack_map.get_mut(&key).unwrap().clear();
                } else {
                    backtrack_map.insert(key.clone(), vec![]);
                }
            }
            backtrack_map
                .get_mut(&key)
                .unwrap()
                .push((r, c, dir.clone()));
            next_steps.push(key.clone(), new_cost);
        }
    }

    let mut backtracking = VecDeque::new();
    let mut visited = HashMap::new();
    let mut position_map = vec![vec![0; cols]; rows];

    for elem in &ends {
        backtracking.push_back(elem.0.clone());
        visited.insert(elem.0, true);
        position_map[elem.0 .0][elem.0 .1] = 1;
    }

    while let Some(pos) = backtracking.pop_front() {
        for back_loc in backtrack_map.get(&(pos.0, pos.1, pos.2.clone())).unwrap() {
            if visited.contains_key(back_loc) {
                continue;
            }
            visited.insert(back_loc, true);
            position_map[back_loc.0][back_loc.1] = 1;
            backtracking.push_back(back_loc.clone());
        }
    }

    let mut ret = 0;
    for r in 0..rows {
        for c in 0..cols {
            ret += position_map[r][c];
        }
    }

    ret
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let dummy = "###############
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
###############";

        assert_eq!(part2(dummy), 45);
    }
    #[test]
    fn test_part1_2() {
        let dummy = "#################
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
#################";

        assert_eq!(part2(dummy), 64);
    }
}
