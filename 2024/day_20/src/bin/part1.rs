use priority_queue::PriorityQueue;
use std::{collections::HashMap, i32};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
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

fn find_path_length(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut visited = HashMap::new();
    let mut next_steps: PriorityQueue<(usize, usize, Direction), i32> = PriorityQueue::new();

    next_steps.push((start.0, start.1, Direction::East), 0);

    while let Some(((r, c, dir), cost)) = next_steps.pop() {
        if visited.contains_key(&(r, c, dir.clone())) {
            continue;
        }
        // println!("at pos: {:?} {:?} - cost: {}", (r, c), dir, -cost);

        visited.insert((r, c, dir.clone()), true);

        if (r, c) == end {
            // println!("Found the way {:?}", (r, c));
            return -cost;
        }

        // Add optional paths to the queue
        match dir {
            Direction::East => {
                // going east
                if grid[r][c + 1] != '#' && !visited.contains_key(&(r, c + 1, Direction::East)) {
                    next_steps.push((r, c + 1, Direction::East), cost - 1);
                }
                // going north
                if grid[r - 1][c] != '#' && !visited.contains_key(&(r - 1, c, Direction::North)) {
                    next_steps.push((r - 1, c, Direction::North), cost - 1);
                }
                // going south
                if grid[r + 1][c] != '#' && !visited.contains_key(&(r + 1, c, Direction::South)) {
                    next_steps.push((r + 1, c, Direction::South), cost - 1);
                }
            }
            Direction::North => {
                // going north
                if grid[r - 1][c] != '#' && !visited.contains_key(&(r - 1, c, Direction::North)) {
                    next_steps.push((r - 1, c, Direction::North), cost - 1);
                }
                // going west
                if grid[r][c - 1] != '#' && !visited.contains_key(&(r, c - 1, Direction::West)) {
                    next_steps.push((r, c - 1, Direction::West), cost - 1);
                }
                // going east
                if grid[r][c + 1] != '#' && !visited.contains_key(&(r, c + 1, Direction::East)) {
                    next_steps.push((r, c + 1, Direction::East), cost - 1);
                }
            }
            Direction::South => {
                // going south
                if grid[r + 1][c] != '#' && !visited.contains_key(&(r + 1, c, Direction::South)) {
                    next_steps.push((r + 1, c, Direction::South), cost - 1);
                }
                // going west
                if grid[r][c - 1] != '#' && !visited.contains_key(&(r, c - 1, Direction::West)) {
                    next_steps.push((r, c - 1, Direction::West), cost - 1);
                }
                // going east
                if grid[r][c + 1] != '#' && !visited.contains_key(&(r, c + 1, Direction::East)) {
                    next_steps.push((r, c + 1, Direction::East), cost - 1);
                }
            }
            Direction::West => {
                // going west
                if grid[r][c - 1] != '#' && !visited.contains_key(&(r, c - 1, Direction::West)) {
                    next_steps.push((r, c - 1, Direction::West), cost - 1);
                }
                // going south
                if grid[r + 1][c] != '#' && !visited.contains_key(&(r + 1, c, Direction::South)) {
                    next_steps.push((r + 1, c, Direction::South), cost - 1);
                }
                // going north
                if grid[r - 1][c] != '#' && !visited.contains_key(&(r - 1, c, Direction::North)) {
                    next_steps.push((r - 1, c, Direction::North), cost - 1);
                }
            }
        }
    }
    i32::MAX
}

fn part1(input: &str) -> i32 {
    let (mut grid, start, end) = parse(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let baseline = find_path_length(&grid, start, end);
    println!("Current path = {baseline}");

    let mut savings = HashMap::new();

    for r in 1..rows - 1 {
        println!("{r}");
        for c in 1..cols - 1 {
            if grid[r][c] == '#' {
                grid[r][c] = '.';

                let test = find_path_length(&grid, start, end);
                if test < baseline {
                    let saving = baseline - test;

                    *savings.entry(saving).or_insert(0i32) += 1i32;
                }
                grid[r][c] = '#';
            }
        }
    }

    let mut sorted_savings: Vec<(&i32, &i32)> = savings.iter().collect();
    sorted_savings.sort_by_key(|&(k, _)| k);

    println!("{:?}", sorted_savings);

    let mut ret  = 0;
    for (saving, num) in sorted_savings{
        if *saving >= 100{
            ret += num;
        }
    }

    ret
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "###############
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
###############";

        assert_eq!(part1(dummy), 1);
    }
}
