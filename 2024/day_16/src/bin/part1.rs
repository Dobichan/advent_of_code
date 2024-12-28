use std::{collections::HashMap, usize};

use priority_queue::PriorityQueue;

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

fn print_grid_and_path(
    grid: &Vec<Vec<char>>,
    cost: &Vec<Vec<i32>>,
    start: (usize, usize),
    end: (usize, usize),
) {
    let rows = grid.len();
    let cols = grid[0].len();
    let dirs = vec!['^', '>', 'v', '<'];

    for r in 0..rows {
        for c in 0..cols {
            if (r, c) == start {
                print!("S");
            } else if (r, c) == end {
                print!("E");
            } else if cost[r][c] != i32::MIN {
                let mut costs = vec![];
                if r > 0 {
                    costs.push(cost[r - 1][c]);
                }
                if c < cols - 1 {
                    costs.push(cost[r][c + 1]);
                }
                if r < rows + 1 {
                    costs.push(cost[r + 1][c]);
                }
                if c > 0 {
                    costs.push(cost[r][c - 1]);
                }
                if let Some((index, _)) = costs.iter().enumerate().max_by_key(|&(_, &value)| value)
                {
                    print!("{}", dirs[index]);
                } else {
                    print!("*");
                }
            } else {
                print!("{}", grid[r][c]);
            }
        }
        println!()
    }
    println!()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for r in grid {
        for c in r {
            print!("{c}");
        }
        println!("");
    }

    println!("");
}

fn part1(input: &str) -> i32 {
    let (grid, start, end) = parse(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = HashMap::new();

    let mut next_steps: PriorityQueue<(usize, usize, Direction), i32> = PriorityQueue::new();
    next_steps.push((start.0, start.1, Direction::East), 0);

    while let Some(((r, c, dir), cost)) = next_steps.pop() {
        println!("at pos: {:?} {:?} - cost: {}", (r, c), dir, -cost);
        if visited.contains_key(&(r, c, dir.clone())) {
            continue;
        }

        visited.insert((r, c, dir.clone()), true);

        if (r, c) == end {
            println!("Found the way {:?}", (r, c));
            return -cost;
        }

        // Add optional paths to the queue
        match dir {
            Direction::East => {
                if c < cols - 1 {
                    // going east
                    if grid[r][c + 1] != '#' && !visited.contains_key(&(r, c + 1, Direction::East))
                    {
                        next_steps.push((r, c + 1, Direction::East), cost - 1);
                    }
                }
                if r > 0 {
                    // going north
                    if grid[r - 1][c] != '#' && !visited.contains_key(&(r - 1, c, Direction::North))
                    {
                        next_steps.push((r - 1, c, Direction::North), cost - 1001);
                    }
                }
                if r < rows - 1 {
                    // going south
                    if grid[r + 1][c] != '#' && !visited.contains_key(&(r + 1, c, Direction::South))
                    {
                        next_steps.push((r + 1, c, Direction::South), cost - 1001);
                    }
                }
            }
            Direction::North => {
                if r > 0 {
                    // going north
                    if grid[r - 1][c] != '#' && !visited.contains_key(&(r - 1, c, Direction::North))
                    {
                        next_steps.push((r - 1, c, Direction::North), cost - 1);
                    }
                }
                if c > 0 {
                    // going west
                    if grid[r][c - 1] != '#' && !visited.contains_key(&(r, c - 1, Direction::West))
                    {
                        next_steps.push((r, c - 1, Direction::West), cost - 1001);
                    }
                }
                if c < cols - 1 {
                    // going east
                    if grid[r][c + 1] != '#' && !visited.contains_key(&(r, c + 1, Direction::East))
                    {
                        next_steps.push((r, c + 1, Direction::East), cost - 1001);
                    }
                }
            }
            Direction::South => {
                if r < rows - 1 {
                    // going south
                    if grid[r + 1][c] != '#' && !visited.contains_key(&(r + 1, c, Direction::South))
                    {
                        next_steps.push((r + 1, c, Direction::South), cost - 1);
                    }
                }
                if c > 0 {
                    // going west
                    if grid[r][c - 1] != '#' && !visited.contains_key(&(r, c - 1, Direction::West))
                    {
                        next_steps.push((r, c - 1, Direction::West), cost - 1001);
                    }
                }
                if c < cols - 1 {
                    // going east
                    if grid[r][c + 1] != '#' && !visited.contains_key(&(r, c + 1, Direction::East))
                    {
                        next_steps.push((r, c + 1, Direction::East), cost - 1001);
                    }
                }
            }
            Direction::West => {
                if c > 0 {
                    // going west
                    if grid[r][c - 1] != '#' && !visited.contains_key(&(r, c - 1, Direction::West))
                    {
                        next_steps.push((r, c - 1, Direction::West), cost - 1);
                    }
                }
                if r < rows - 1 {
                    // going south
                    if grid[r + 1][c] != '#' && !visited.contains_key(&(r + 1, c, Direction::South))
                    {
                        next_steps.push((r + 1, c, Direction::South), cost - 1001);
                    }
                }
                if r > 0 {
                    // going north
                    if grid[r - 1][c] != '#' && !visited.contains_key(&(r - 1, c, Direction::North))
                    {
                        next_steps.push((r - 1, c, Direction::North), cost - 1001);
                    }
                }
            }
        }
    }

    0
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

        let (grid, start, end) = parse(dummy);
        print_grid(&grid);
        println!("{:?}", start);
        println!("{:?}", end);

        assert_eq!(part1(dummy), 7036);
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

        let (grid, start, end) = parse(dummy);
        print_grid(&grid);
        println!("{:?}", start);
        println!("{:?}", end);

        assert_eq!(part1(dummy), 11048);
    }
}
