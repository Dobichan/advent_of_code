use priority_queue::PriorityQueue;
use std::collections::HashMap;

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
    let output = part1(input, 71);
    dbg!(output); // Don't include start and end
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();

            // println!("{a} {b}");
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn create_grid(size: usize) -> Vec<Vec<char>> {
    vec![vec!['.'; size]; size]
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

fn part1(input: &str, grid_size: usize) -> i32 {
    let blocks = parse(&input);
    let mut grid = create_grid(grid_size);

    for i in 0..=1024 {
        let row = blocks[i].1;
        let col = blocks[i].0;

        grid[row][col] = '#';
    }
    print_grid(&grid);

    let start = (0, 0);
    let end = (grid_size-1, grid_size-1);
    let mut visited = HashMap::new();
    let mut next_steps: PriorityQueue<(usize, usize, Direction), i32> = PriorityQueue::new();

    next_steps.push((start.0, start.1, Direction::East), 0);

    while let Some(((r, c, dir), cost)) = next_steps.pop() {
        if visited.contains_key(&(r, c, dir.clone())) {
            continue;
        }
        println!("at pos: {:?} {:?} - cost: {}", (r, c), dir, -cost);

        visited.insert((r, c, dir.clone()), true);

        if (r, c) == end {
            println!("Found the way {:?}", (r, c));
            return -cost;
        }

        // Add optional paths to the queue
        match dir {
            Direction::East => {
                if c < grid_size - 1 {
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
                        next_steps.push((r - 1, c, Direction::North), cost - 1);
                    }
                }
                if r < grid_size - 1 {
                    // going south
                    if grid[r + 1][c] != '#' && !visited.contains_key(&(r + 1, c, Direction::South))
                    {
                        next_steps.push((r + 1, c, Direction::South), cost - 1);
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
                        next_steps.push((r, c - 1, Direction::West), cost - 1);
                    }
                }
                if c < grid_size - 1 {
                    // going east
                    if grid[r][c + 1] != '#' && !visited.contains_key(&(r, c + 1, Direction::East))
                    {
                        next_steps.push((r, c + 1, Direction::East), cost - 1);
                    }
                }
            }
            Direction::South => {
                if r < grid_size - 1 {
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
                        next_steps.push((r, c - 1, Direction::West), cost - 1);
                    }
                }
                if c < grid_size - 1 {
                    // going east
                    if grid[r][c + 1] != '#' && !visited.contains_key(&(r, c + 1, Direction::East))
                    {
                        next_steps.push((r, c + 1, Direction::East), cost - 1);
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
                if r < grid_size - 1 {
                    // going south
                    if grid[r + 1][c] != '#' && !visited.contains_key(&(r + 1, c, Direction::South))
                    {
                        next_steps.push((r + 1, c, Direction::South), cost - 1);
                    }
                }
                if r > 0 {
                    // going north
                    if grid[r - 1][c] != '#' && !visited.contains_key(&(r - 1, c, Direction::North))
                    {
                        next_steps.push((r - 1, c, Direction::North), cost - 1);
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
    fn test_part1() {
        let dummy = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

        assert_eq!(part1(dummy, 7), 24);
    }
}
