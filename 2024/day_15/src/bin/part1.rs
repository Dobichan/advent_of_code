fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
enum Direction {
    left,
    up,
    right,
    down,
}

fn print_grid(grid: &Vec<Vec<char>>, rx: usize, ry: usize) {
    for (y, line) in grid.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if x == rx && y == ry {
                print!("@");
                continue;
            }
            print!("{ch}");
        }
        println!("");
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Direction>) {
    let mut parsing_grid = true;

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<Direction> = Vec::new();

    for line in input.lines() {
        if parsing_grid {
            if line.is_empty() {
                parsing_grid = false;
                continue;
            }

            let mut grid_line: Vec<char> = vec![];

            for ch in line.chars() {
                grid_line.push(ch);
            }
            grid.push(grid_line);
        } else {
            for i in line.chars() {
                match i {
                    '<' => instructions.push(Direction::left),
                    '^' => instructions.push(Direction::up),
                    '>' => instructions.push(Direction::right),
                    'v' => instructions.push(Direction::down),
                    _ => continue,
                }
            }
        }
    }
    (grid, instructions)
}

fn extract_robot(grid: &mut Vec<Vec<char>>) -> Option<(usize, usize)> {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                grid[y][x] = '.';
                return Some((x, y));
            }
        }
    }
    None
}

fn shufle_crates(grid: &mut Vec<Vec<char>>, dir: &Direction, rx: usize, ry: usize) {
    match dir {
        Direction::left => {
            if grid[ry][rx - 1] == 'O' {
                for x in (0..rx).rev() {
                    if grid[ry][x] == '.' {
                        grid[ry][x] = 'O';
                        grid[ry][rx - 1] = '.';
                        return;
                    }
                    if grid[ry][x] == '#' {
                        return;
                    }
                }
            }
        }
        Direction::up => {
            if grid[ry - 1][rx] == 'O' {
                for y in (0..ry).rev() {
                    if grid[y][rx] == '.' {
                        grid[y][rx] = 'O';
                        grid[ry - 1][rx] = '.';
                        return;
                    }
                    if grid[y][rx] == '#' {
                        return;
                    }
                }
            }
        }
        Direction::right => {
            if grid[ry][rx + 1] == 'O' {
                for x in rx + 1..grid[0].len() {
                    if grid[ry][x] == '.' {
                        grid[ry][x] = 'O';
                        grid[ry][rx + 1] = '.';
                        return;
                    }
                    if grid[ry][x] == '#' {
                        return;
                    }
                }
            }
        }
        Direction::down => {
            if grid[ry + 1][rx] == 'O' {
                for y in ry + 1..grid.len() {
                    if grid[y][rx] == '.' {
                        grid[y][rx] = 'O';
                        grid[ry + 1][rx] = '.';
                        return;
                    }
                    if grid[y][rx] == '#' {
                        return;
                    }
                }
            }
        }
        _ => todo!(),
    }
}

fn move_robot(grid: &mut Vec<Vec<char>>, dir: &Direction, rx: usize, ry: usize) -> (usize, usize) {
    shufle_crates(grid, dir, rx, ry);
    match dir {
        Direction::left => {
            if grid[ry][rx - 1] == '.' {
                return (rx - 1, ry);
            }
        }
        Direction::up => {
            if grid[ry - 1][rx] == '.' {
                return (rx, ry - 1);
            }
        }
        Direction::right => {
            if grid[ry][rx + 1] == '.' {
                return (rx + 1, ry);
            }
        }
        Direction::down => {
            if grid[ry + 1][rx] == '.' {
                return (rx, ry + 1);
            }
        }
        _ => return (rx, ry),
    }
    (rx, ry)
}

fn part1(input: &str) -> usize {
    let (mut grid, instructions) = parse(input);

    let (mut robot_x, mut robot_y) = extract_robot(&mut grid).unwrap();
    print_grid(&grid, robot_x, robot_y);

    for i in instructions {
        // println!("{:?} {robot_x}.{robot_y}", i);
        (robot_x, robot_y) = move_robot(&mut grid, &i, robot_x, robot_y);
        // print_grid(&grid, robot_x, robot_y);
        // println!("new pos: {robot_x}.{robot_y}");
        // println!("--------------------------------------------------------------");
    }
    print_grid(&grid, robot_x, robot_y);

    let mut crate_sum = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'O' {
                crate_sum += 100 * r + c;
            }
        }
    }
    crate_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        // let (grid, instructions) = parse(&dummy);

        // print_grid(&grid);
        // println!("{:?}", instructions);

        assert_eq!(part1(dummy), 2028);
    }

    #[test]
    fn test_part1_larger() {
        let dummy = "##########
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
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        assert_eq!(part1(dummy), 10092);
    }
}
