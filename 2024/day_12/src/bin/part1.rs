use std::collections::{HashMap, VecDeque};

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut ret: Vec<Vec<char>> = Vec::new();
    for (r, l) in input.lines().enumerate() {
        let mut line: Vec<char> = Vec::new();
        for (c, ch) in l.chars().enumerate() {
            line.push(ch);
        }
        ret.push(line);
    }
    ret
}

fn part1(input: &str) -> i32 {
    let grid = parse(input);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut total_price = 0;

    let mut processed: HashMap<(usize, usize), char> = HashMap::new();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if processed.contains_key(&(r, c)) {
                continue;
            }
            // println!("Checking {r}x{c} {:?}", grid[r][c]);

            let mut area = 0;
            let mut perimeter = 0;

            let mut to_check: VecDeque<(usize, usize)> = VecDeque::new();
            to_check.push_back((r, c));

            while to_check.len() > 0 {
                let (r1, c1) = to_check.pop_front().unwrap();
                if processed.contains_key(&(r1, c1)) {
                    continue;
                }
                area += 1;
                processed.insert((r1, c1), grid[r1][c1]);

                // Check above
                if r1 > 0 {
                    if grid[r1 - 1][c1] == grid[r1][c1] {
                        to_check.push_back((r1 - 1, c1));
                    } else {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }

                // Check right
                if c1 < cols - 1 {
                    if grid[r1][c1 + 1] == grid[r1][c1] {
                        to_check.push_back((r1, c1 + 1));
                    } else {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }

                // Check below
                if r1 < rows - 1 {
                    if grid[r1 + 1][c1] == grid[r1][c1] {
                        to_check.push_back((r1 + 1, c1));
                    } else {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }

                // Check left
                if c1 > 0 {
                    if grid[r1][c1 - 1] == grid[r1][c1] {
                        to_check.push_back((r1, c1 - 1));
                    } else {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }

                // println!("to check: {:?}", to_check);
            }
            println!("Process area {:?} - {area} with {perimeter}", grid[r][c]);
            total_price+= area*perimeter;
        }
    }

    total_price
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "AAAA
BBCD
BBCC
EEEC";

        assert_eq!(part1(dummy), 140);
    }
}
