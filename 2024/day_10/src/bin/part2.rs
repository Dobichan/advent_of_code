use itertools::Itertools;

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut ret = vec![];

    for line in input.lines() {
        let mut rl: Vec<u8> = vec![];
        for ch in line.chars() {
            rl.push(ch.to_digit(10).unwrap() as u8);
        }
        ret.push(rl);
    }
    ret
}

fn num_possible(grid: &Vec<Vec<u8>>, num: u8, r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = vec![];
    let rows = grid.len();
    let cols = grid[0].len();

    if num != grid[r][c] {
        return ret;
    }
    println!("Looking for {num} @ {r}x{c}");

    if num == 9 {
        println!("9 @ {r}x{c}");
        ret.push((r, c));
    } else {
        // Check left for one bigger
        if c > 0 {
            let temp = num_possible(grid, num + 1, r, c - 1);
            for l in temp {
                ret.push(l);
            }
        }

        // Check above for one bigger
        if r > 0 {
            let temp = num_possible(grid, num + 1, r - 1, c);
            for l in temp {
                ret.push(l);
            }
        }

        // Check right for one bigger
        if c < cols - 1 {
            let temp = num_possible(grid, num + 1, r, c + 1);
            for l in temp {
                ret.push(l);
            }
        }

        // Check below for one bigger
        if r < rows - 1 {
            let temp = num_possible(grid, num + 1, r + 1, c);
            for l in temp {
                ret.push(l);
            }
        }
    }
    ret
}

fn trailhead(grid: &Vec<Vec<u8>>, r: usize, c: usize) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();

    if grid[r][c] == 0 {
        let temp = num_possible(grid, 0, r, c);
        println!("found: {:?}", temp);
        return temp.len() as u32;
    }
    0
}

fn part2(input: &str) -> u32 {
    let grid = parse(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut sum = 0;

    for r in 0..rows {
        for c in 0..cols {
            sum += trailhead(&grid, r, c);
        }
    }
    sum
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let dummy = "9990999
9991999
9992999
6543456
7111117
8111118
9111119";

        assert_eq!(
            parse(dummy),
            [
                [9, 9, 9, 0, 9, 9, 9],
                [9, 9, 9, 1, 9, 9, 9],
                [9, 9, 9, 2, 9, 9, 9],
                [6, 5, 4, 3, 4, 5, 6],
                [7, 1, 1, 1, 1, 1, 7],
                [8, 1, 1, 1, 1, 1, 8],
                [9, 1, 1, 1, 1, 1, 9]
            ]
        );
    }

    #[test]
    fn test_part2() {
        let dummy = "9990999
9991999
9992999
6543456
7111117
8111118
9111119";

        let grid = parse(&dummy);

        assert_eq!(trailhead(&grid, 0, 3), 2);
    }
}
