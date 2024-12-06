#[derive(Debug, PartialEq)]
enum WorldObject {
    Empty,
    Obstacle,
    Visited,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Guard {
    position: (i32, i32),
    direction: Direction,
}

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse(input: &str) -> (Vec<Vec<WorldObject>>, Guard) {
    let mut world: Vec<Vec<WorldObject>> = vec![];
    let mut guard = Guard {
        position: (-1, -1),
        direction: Direction::North,
    };
    for (row, line) in input.lines().enumerate() {
        let mut grid_line: Vec<WorldObject> = vec![];
        for (col, c) in line.chars().enumerate() {
            grid_line.push(match c {
                '#' => WorldObject::Obstacle,
                '^' => {
                    guard = Guard {
                        position: (row as i32, col as i32),
                        direction: Direction::North,
                    };
                    WorldObject::Empty
                }
                _ => WorldObject::Empty,
            });
        }
        world.push(grid_line);
    }
    (world, guard)
}

fn tick_world(world: &mut Vec<Vec<WorldObject>>, guard: &mut Guard) {
    let (r, c) = guard.position;
    world[r as usize][c as usize] = WorldObject::Visited;

    match guard.direction {
        Direction::North => {
            // println!("north: {:?}", guard);
            let row = world.get((r - 1) as usize);
            if r - 1 >= 0 && row.is_some() {
                let c = row.unwrap().get(c as usize);
                if let WorldObject::Obstacle = c.unwrap() {
                    // Crashing - rotate
                    guard.direction = Direction::East;
                    return;
                }
            }
            guard.position = (r - 1, c);
        }
        Direction::East => {
            // println!("east: {:?}", guard);
            let row = world.get(r as usize);
            if row.is_some() {
                let c = row.unwrap().get((c + 1) as usize);
                if let WorldObject::Obstacle = c.unwrap() {
                    // Crashing - rotate
                    guard.direction = Direction::South;
                    return;
                }
            }
            guard.position = (r, c + 1);
        }
        Direction::South => {
            // println!("south: {:?}", guard);
            let row = world.get((r + 1) as usize);
            if row.is_some() {
                let c = row.unwrap().get(c as usize);
                if let WorldObject::Obstacle = c.unwrap() {
                    // Crashing - rotate
                    guard.direction = Direction::West;
                    return;
                }
            }
            guard.position = (r + 1, c);
        }
        Direction::West => {
            // println!("west: {:?}", guard);
            let row = world.get(r as usize);
            if row.is_some() && c - 1 >= 0 {
                let c = row.unwrap().get((c - 1) as usize);
                if let WorldObject::Obstacle = c.unwrap() {
                    // Crashing - rotate
                    guard.direction = Direction::North;
                    return;
                }
            }
            guard.position = (r, c - 1);
        }
    }
}

fn part1(input: &str) -> usize {
    let (mut world, mut grd) = parse(input);

    println!("{:?}", world);

    while grd.position.0 >= 0
        && grd.position.0 < world.len() as i32
        && grd.position.1 >= 0
        && grd.position.1 < world[0].len() as i32
    {
        tick_world(&mut world, &mut grd);
    }

    println!("{:?}", world);

    world
        .iter()
        .map(|r| r.iter().filter(|c| **c == WorldObject::Visited).count())
        .sum::<usize>()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(part1(dummy), 41);
    }
}
