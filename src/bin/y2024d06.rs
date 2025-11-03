use std::{collections::HashMap, usize};

use advent_of_code::{grid::Grid, parsing::*};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Guard {
    position: (i32, i32),
    direction: Direction,
    positions: HashMap<((i32, i32), Direction), i32>,
    repeating: bool,
}

fn move_guard(world: &mut Grid, guard: &mut Guard) {
    world[guard.position.1 as usize][guard.position.0 as usize] = 'X';

    let pos_dir = (guard.position, guard.direction);
    if guard.positions.contains_key(&pos_dir) {
        guard.repeating = true;
        return;
    }
    guard.positions.insert(pos_dir, 1);

    let mut new_pos = guard.position;
    let new_direction: Direction;

    match guard.direction {
        Direction::North => {
            new_pos.1 = new_pos.1 - 1;
            new_direction = Direction::East;
        }

        Direction::East => {
            new_pos.0 = new_pos.0 + 1;
            new_direction = Direction::South;
        }

        Direction::South => {
            new_pos.1 = new_pos.1 + 1;
            new_direction = Direction::West;
        }

        Direction::West => {
            new_pos.0 = new_pos.0 - 1;
            new_direction = Direction::North;
        }
    }
    if world.inside((new_pos.0 as usize, new_pos.1 as usize)) {
        if world[new_pos.1 as usize][new_pos.0 as usize] == '#' {
            // Need to rotate
            guard.direction = new_direction;
        } else {
            guard.position = new_pos;
        }
    } else {
        // We are done
        guard.position = new_pos;
    }
}

fn guard_inside_grid(guard: &Guard, world: &Grid) -> bool {
    world.inside((guard.position.0 as usize, guard.position.1 as usize))
}

fn part1(input: &str) -> i64 {
    let mut world: Grid = input.parse().unwrap();

    let guard_pos = world
        .find('^')
        .expect("Could not find initial guard position");
    let mut guard = Guard {
        position: (guard_pos.0 as i32, guard_pos.1 as i32),
        direction: Direction::North,
        positions: HashMap::new(),
        repeating: false,
    };

    while guard_inside_grid(&guard, &world) {
        move_guard(&mut world, &mut guard);
    }

    world.into_iter().filter(|e| e.1 == &'X').count() as i64
}

fn part2(input: &str) -> i64 {
    let start_grid: Grid = input.parse().unwrap();

    let mut ret = 0;

    let mut base_world = start_grid.clone();
    let guard_pos = base_world
        .find('^')
        .expect("Could not find initial guard position");
    let mut guard = Guard {
        position: (guard_pos.0 as i32, guard_pos.1 as i32),
        direction: Direction::North,
        positions: HashMap::new(),
        repeating: false,
    };

    while base_world.inside((guard.position.0 as usize, guard.position.1 as usize))
        && !guard.repeating
    {
        move_guard(&mut base_world, &mut guard);
    }

    for test_pos in base_world.into_iter().filter(|p| p.1 == &'X') {
        let mut world = start_grid.clone();

        let guard_pos = world
            .find('^')
            .expect("Could not find initial guard position");

        if test_pos.0.0 == guard_pos.0 && test_pos.0.1 == guard_pos.1 {
            dbg!(test_pos);
            println!("Skipping start {:?}", guard_pos);
            continue;
        }

        // Add obstacle
        world[test_pos.0.1][test_pos.0.0] = '#';

        let mut guard = Guard {
            position: (guard_pos.0 as i32, guard_pos.1 as i32),
            direction: Direction::North,
            positions: HashMap::new(),
            repeating: false,
        };

        while world.inside((guard.position.0 as usize, guard.position.1 as usize))
            && !guard.repeating
        {
            move_guard(&mut world, &mut guard);
        }

        if guard.repeating {
            ret += 1;
        }
    }

    ret
}

fn main() {
    const YEAR: u16 = 2024;
    const DAY: u8 = 6;

    let input = read_input(YEAR, DAY);

    let start = std::time::Instant::now();
    let answer1 = part1(&input);
    let end = std::time::Instant::now();

    println!("");
    println!("Answer part 1: {answer1}");
    println!("Elapsed: {:.3} ms", (end - start).as_secs_f64() * 1000.0);
    println!("");

    let start = std::time::Instant::now();
    let answer2 = part2(&input);
    let end = std::time::Instant::now();

    println!("Answer part 2: {answer2}");
    println!("Elapsed: {:.3} ms", (end - start).as_secs_f64() * 1000.0);
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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
        ......#...";

        assert_eq!(part1(EXAMPLE_INPUT.trim()), 41);
    }

    #[test]
    fn test_part2() {
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
        ......#...";

        assert_eq!(part2(EXAMPLE_INPUT.trim()), 6);
    }
}
