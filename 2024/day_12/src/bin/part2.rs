use std::collections::{HashMap, VecDeque};

fn main() {
    println!("Part2!");

    let input = include_str!("../input.txt");
    let output = part2(input);
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

fn part2(input: &str) -> i32 {
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
            println!("Checking {r}x{c} {:?}", grid[r][c]);

            let mut area = 0;
            let mut direction_change = 0;

            let mut to_check: VecDeque<(usize, usize)> = VecDeque::new();
            to_check.push_back((r, c));

            while to_check.len() > 0 {
                let (r1, c1) = to_check.pop_front().unwrap();
                println!("checking {r1},{c1}");
                if processed.contains_key(&(r1, c1)) {
                    continue;
                }
                area += 1;
                processed.insert((r1, c1), grid[r1][c1]);

                // Check right&up
                if r1 > 0 && c1 < cols - 1 {
                    println!("Check right&up ({area}-{direction_change})");
                    if (grid[r1][c1 + 1] == grid[r1][c1]) && (grid[r1 - 1][c1 + 1] == grid[r1][c1])
                    {
                        to_check.push_back((r1 - 1, c1 + 1));
                        direction_change += 1;
                    }
                }
                // Check right
                if c1 < cols - 1 {
                    println!("Check right ({area}-{direction_change})");
                    if grid[r1][c1 + 1] == grid[r1][c1] {
                        to_check.push_back((r1, c1 + 1));
                    }
                }

                // Check right&down
                if r1 < rows - 1 && c1 < cols - 1 {
                    println!("Check right&down ({area}-{direction_change})");
                    if grid[r1][c1 + 1] == grid[r1][c1] && grid[r1 + 1][c1 + 1] == grid[r1][c1] {
                        to_check.push_back((r1 + 1, c1 + 1));
                        direction_change += 1;
                    }
                }

                // Check down
                if r1 < rows - 1 {
                    println!("Check down ({area}-{direction_change})");
                    if grid[r1 + 1][c1] == grid[r1][c1] {
                        to_check.push_back((r1 + 1, c1));
                    }
                }

                // Check down&left
                if r1 > rows - 1 && c1 > 0 {
                    println!("Check down&left ({area}-{direction_change})");

                    if grid[r1 + 1][c1] == grid[r1][c1] && grid[r1 + 1][c1 - 1] == grid[r1][c1] {
                        to_check.push_back((r1 + 1, c1 + 1));
                        direction_change += 1;
                    }
                }

                // Check left
                if c1 > 0 {
                    if grid[r1][c1 - 1] == grid[r1][c1] {
                        println!("Check left ({area}-{direction_change})");
                        to_check.push_back((r1, c1 - 1));
                    }
                }

                // Check left&up
                if c1 > 0 && r1 > 0 {
                    println!("Check left&up ({area}-{direction_change})");

                    if grid[r1][c1 - 1] == grid[r1][c1] && grid[r1 - 1][c1 - 1] == grid[r1][c1] {
                        to_check.push_back((r1 - 1, c1 - 1));
                        direction_change += 1;
                    }
                }

                // println!("to check: {:?}", to_check);
            }
            println!(
                "Process area {:?} - {area} with {direction_change}",
                grid[r][c]
            );
            total_price += area * direction_change;
            return total_price;
        }
    }

    total_price
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part2_1() {
        let dummy = "AAAA
BBCD
BBCC
EEEC";

        assert_eq!(part2(dummy), 80);
    }

    #[test]
    fn test_part2_2() {
        let dummy = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

        assert_eq!(part2(dummy), 236);
    }

    #[test]
    fn test_part2_3() {
        let dummy = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

        assert_eq!(part2(dummy), 368);
    }

    #[test]
    fn test_part2_4() {
        let dummy = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

        assert_eq!(part2(dummy), 236);
    }

    #[test]
    fn test_part2_larger() {
        let dummy = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        assert_eq!(part2(dummy), 1206);
    }
}
