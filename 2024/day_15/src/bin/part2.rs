fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part2(input);
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
                if ch == 'O' {
                    grid_line.push('[');
                    grid_line.push(']');
                } else if ch == '@' {
                    grid_line.push(ch);
                    grid_line.push('.');
                } else {
                    grid_line.push(ch);
                    grid_line.push(ch);
                }
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

fn can_crate_move(grid: &Vec<Vec<char>>, dir: &Direction, lx: usize, ly: usize) -> bool {
    match dir {
        Direction::left => {
            if grid[ly][lx - 1] == '.' {
                return true;
            }
            if grid[ly][lx - 1] == ']' {
                return can_crate_move(grid, dir, lx - 2, ly);
            }
        }
        Direction::up => {
            if grid[ly - 1][lx] == '.' && grid[ly - 1][lx + 1] == '.' {
                return true;
            }
            if grid[ly - 1][lx] == '[' {
                return can_crate_move(grid, dir, lx, ly - 1);
            }
            if grid[ly - 1][lx - 1] == '[' && grid[ly - 1][lx + 1] == '.' {
                return can_crate_move(grid, dir, lx - 1, ly - 1);
            }
            if grid[ly - 1][lx] == '.' && grid[ly - 1][lx + 1] == '[' {
                return can_crate_move(grid, dir, lx + 1, ly - 1);
            }
            if grid[ly - 1][lx - 1] == '['
                && grid[ly - 1][lx] == ']'
                && grid[ly - 1][lx + 1] == '['
                && grid[ly - 1][lx + 2] == ']'
            {
                let crate_a_moveable = can_crate_move(grid, dir, lx - 1, ly - 1);
                let crate_b_moveable = can_crate_move(grid, dir, lx + 1, ly - 1);
                return crate_a_moveable && crate_b_moveable;
            }
        }
        Direction::right => {
            if grid[ly][lx + 2] == '.' {
                return true;
            }
            if grid[ly][lx + 2] == '[' {
                return can_crate_move(grid, dir, lx + 2, ly);
            }
        }
        Direction::down => {
            if grid[ly + 1][lx] == '.' && grid[ly + 1][lx + 1] == '.' {
                return true;
            }
            if grid[ly + 1][lx] == '[' {
                return can_crate_move(grid, dir, lx, ly + 1);
            }
            if grid[ly + 1][lx - 1] == '[' && grid[ly + 1][lx + 1] == '.' {
                return can_crate_move(grid, dir, lx - 1, ly + 1);
            }
            if grid[ly + 1][lx] == '.' && grid[ly + 1][lx + 1] == '[' {
                return can_crate_move(grid, dir, lx + 1, ly + 1);
            }
            if grid[ly + 1][lx - 1] == '['
                && grid[ly + 1][lx] == ']'
                && grid[ly + 1][lx + 1] == '['
                && grid[ly + 1][lx + 2] == ']'
            {
                let crate_a_moveable = can_crate_move(grid, dir, lx - 1, ly + 1);
                let crate_b_moveable = can_crate_move(grid, dir, lx + 1, ly + 1);
                return crate_a_moveable && crate_b_moveable;
            }
        }
        _ => todo!(),
    }
    false
}

fn move_crate(grid: &mut Vec<Vec<char>>, dir: &Direction, lx: usize, ly: usize) {
    match dir {
        Direction::left => {
            if grid[ly][lx - 1] == ']' {
                move_crate(grid, dir, lx - 2, ly)
            }
            grid[ly][lx - 1] = '[';
            grid[ly][lx] = ']';
            grid[ly][lx + 1] = '.';
        }
        Direction::up => {
            if grid[ly - 1][lx] == '[' {
                move_crate(grid, dir, lx, ly - 1);
            }
            if grid[ly - 1][lx] == ']' && grid[ly - 1][lx + 1] == '.' {
                move_crate(grid, dir, lx - 1, ly - 1);
            }
            if grid[ly - 1][lx] == '.' && grid[ly - 1][lx + 1] == '[' {
                move_crate(grid, dir, lx + 1, ly - 1)
            }

            if grid[ly - 1][lx] == ']' && grid[ly - 1][lx + 1] == '[' {
                move_crate(grid, dir, lx - 1, ly - 1);
                move_crate(grid, dir, lx + 1, ly - 1)
            }

            if grid[ly - 1][lx] == '.' && grid[ly - 1][lx + 1] == '.' {
                grid[ly - 1][lx] = '[';
                grid[ly - 1][lx + 1] = ']';
                grid[ly][lx] = '.';
                grid[ly][lx + 1] = '.';
            }
        }
        Direction::right => {
            if grid[ly][lx + 2] == '[' {
                move_crate(grid, dir, lx + 2, ly)
            }
            grid[ly][lx + 1] = '[';
            grid[ly][lx + 2] = ']';
            grid[ly][lx] = '.';
        }
        Direction::down => {
            if grid[ly + 1][lx] == '[' {
                move_crate(grid, dir, lx, ly + 1);
            }
            if grid[ly + 1][lx] == ']' && grid[ly + 1][lx + 1] == '.' {
                move_crate(grid, dir, lx - 1, ly + 1);
            }
            if grid[ly + 1][lx] == '.' && grid[ly + 1][lx + 1] == '[' {
                move_crate(grid, dir, lx + 1, ly + 1)
            }

            if grid[ly + 1][lx] == ']' && grid[ly + 1][lx + 1] == '[' {
                move_crate(grid, dir, lx - 1, ly + 1);
                move_crate(grid, dir, lx + 1, ly + 1)
            }

            if grid[ly + 1][lx] == '.' && grid[ly + 1][lx + 1] == '.' {
                grid[ly + 1][lx] = '[';
                grid[ly + 1][lx + 1] = ']';
                grid[ly][lx] = '.';
                grid[ly][lx + 1] = '.';
            }
        }
    }
}

fn move_robot(grid: &mut Vec<Vec<char>>, dir: &Direction, rx: usize, ry: usize) -> (usize, usize) {
    match dir {
        Direction::left => {
            if grid[ry][rx - 1] == ']' {
                if can_crate_move(grid, dir, rx - 2, ry) {
                    move_crate(grid, dir, rx - 2, ry);
                }
            }
            if grid[ry][rx - 1] == '.' {
                return (rx - 1, ry);
            }
        }
        Direction::up => {
            if grid[ry - 1][rx] == '[' {
                if can_crate_move(grid, dir, rx, ry - 1) {
                    move_crate(grid, dir, rx, ry - 1);
                }
            }
            if grid[ry - 1][rx - 1] == '[' {
                if can_crate_move(grid, dir, rx - 1, ry - 1) {
                    move_crate(grid, dir, rx - 1, ry - 1);
                }
            }
            if grid[ry - 1][rx] == '.' {
                return (rx, ry - 1);
            }
        }
        Direction::right => {
            if grid[ry][rx + 1] == '[' {
                if can_crate_move(grid, dir, rx + 1, ry) {
                    move_crate(grid, dir, rx + 1, ry);
                }
            }
            if grid[ry][rx + 1] == '.' {
                return (rx + 1, ry);
            }
        }
        Direction::down => {
            if grid[ry + 1][rx] == '[' {
                println!("Crate below");
                if can_crate_move(grid, dir, rx, ry + 1) {
                    println!("crate can move");
                    move_crate(grid, dir, rx, ry + 1);
                }
            }
            if grid[ry + 1][rx - 1] == '[' {
                if can_crate_move(grid, dir, rx - 1, ry + 1) {
                    move_crate(grid, dir, rx - 1, ry + 1);
                }
            }
            if grid[ry + 1][rx] == '.' {
                return (rx, ry + 1);
            }
        }
        _ => return (rx, ry),
    }
    (rx, ry)
}

fn part2(input: &str) -> usize {
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
            if grid[r][c] == '[' {
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
    fn test_problem() {
        let dummy = "#######
#...#.#
#...@.#
#..OOO#
#..O.O#
#.....#
#######

vv";
        let (mut grid, instr) = parse(dummy);
        let (rx, ry) = extract_robot(&mut grid).unwrap();

        print_grid(&grid, rx, ry);
        move_robot(&mut grid, &Direction::down, rx, ry);
        print_grid(&grid, rx, ry);
    }

    #[test]
    fn test_part1() {
        let dummy = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

        let (grid, _instr) = parse(dummy);
        print_grid(&grid, 0, 0);

        assert_eq!(part2(dummy), 618);
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

        assert_eq!(part2(dummy), 9021);
    }
}
