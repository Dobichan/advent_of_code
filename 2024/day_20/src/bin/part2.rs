use std::collections::HashMap;

fn main() {
    println!("Part1!");

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

fn create_length_map(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<Vec<i32>> {
    // The maze has only one path. Create a map of the distance from start.
    let rows = grid.len();
    let cols = grid[0].len();
    let mut pos = start;
    let mut distance = 0;

    let mut distance_map = vec![vec![-1; cols]; rows];

    loop {
        distance_map[pos.0][pos.1] = distance;
        if pos == end {
            break;
        }

        // north
        if grid[pos.0 - 1][pos.1] != '#' && distance_map[pos.0 - 1][pos.1] == -1 {
            pos.0 -= 1;
        } else if grid[pos.0][pos.1 + 1] != '#' && distance_map[pos.0][pos.1 + 1] == -1 {
            pos.1 += 1;
        } else if grid[pos.0 + 1][pos.1] != '#' && distance_map[pos.0 + 1][pos.1] == -1 {
            pos.0 += 1;
        } else if grid[pos.0][pos.1 - 1] != '#' && distance_map[pos.0][pos.1 - 1] == -1 {
            pos.1 -= 1;
        } else {
            panic!("We are stuck!! {:?}", pos);
        }
        distance += 1;
    }
    distance_map
}

fn get_cheat_saving(
    distance_map: &Vec<Vec<i32>>,
    row: usize,
    col: usize,
    cheat_row_offset: i32,
    cheat_col_offset: i32,
    cheat_steps: usize,
) -> i32 {
    let rows = distance_map.len();
    let cols = distance_map[0].len();

    let current_distance = distance_map[row][col];

    // println!("Cheat check {row},{col} - {cheat_row_offset},{cheat_col_offset} - {cheat_steps}");

    if row as i32 + cheat_row_offset <= 0
        || row as i32 + cheat_row_offset >= rows as i32
        || col as i32 + cheat_col_offset <= 0
        || col as i32 + cheat_col_offset >= cols as i32
    {
        // println!("out of bounds");
        return 0;
    }

    let cheat_row: usize = (row as i32 + cheat_row_offset) as usize;
    let cheat_col: usize = (col as i32 + cheat_col_offset) as usize;
    // println!("{cheat_row},{cheat_col}");

    if distance_map[cheat_row][cheat_col] == -1 {
        return 0;
    }

    let cheat_distance = distance_map[cheat_row][cheat_col];

    let saving = cheat_distance - current_distance - cheat_steps as i32;
    // println!("At {row},{col} - cheating {cheat_steps} to {cheat_row},{cheat_col} {cheat_distance}-{current_distance} = {saving}");
    saving
}

fn part2(input: &str) -> i32 {
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

    let (grid, start, end) = parse(input);
    // let (grid, start, end) = parse(&dummy);
    let rows = grid.len();
    let cols = grid[0].len();

    let distance_map = create_length_map(&grid, start, end);
    let full_distance = distance_map
        .iter()
        .map(|r| r.iter().max().unwrap())
        .max()
        .unwrap();
    println!("{:?}", full_distance);

    for r in &distance_map {
        println!("{:?}", r);
    }

    let mut savings = HashMap::new();

    for row in 0..rows {
        println!("{row}");
        for col in 0..cols {
            if distance_map[row][col] == -1 {
                continue;
            }
            // println!("check {row},{col} - {}", distance_map[row][col]);

            for r in 2..=20 {
                for cr in 0..=r {
                    let cc = r - cr;
                    // println!("{r} - {cr},{cc}");
                    let saving = get_cheat_saving(&distance_map, row, col, cr, cc, r as usize);
                    if saving > 0 {
                        *savings.entry(saving).or_insert(0i32) += 1i32;
                    }
                    if cc > 0 {
                        let saving = get_cheat_saving(&distance_map, row, col, cr, -cc, r as usize);
                        if saving > 0 {
                            *savings.entry(saving).or_insert(0i32) += 1i32;
                        }
                    }
                    if cr > 0 {
                        let saving = get_cheat_saving(&distance_map, row, col, -cr, cc, r as usize);
                        if saving > 0 {
                            *savings.entry(saving).or_insert(0i32) += 1i32;
                        }
                    }
                    if cr > 0 && cc > 0 {
                        let saving =
                            get_cheat_saving(&distance_map, row, col, -cr, -cc, r as usize);
                        if saving > 0 {
                            *savings.entry(saving).or_insert(0i32) += 1i32;
                        }
                    }
                }
            }
        }
    }

    let mut sorted_savings: Vec<(&i32, &i32)> = savings.iter().collect();
    sorted_savings.sort_by_key(|&(k, _)| k);

    println!("{:?}", sorted_savings);

    let mut ret = 0;
    for (saving, num) in sorted_savings {
        if *saving >= 100 {
            ret += num;
        }
    }

    println!("{ret}");

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

        assert_eq!(
            part2(dummy),
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }
}
