use std::collections::{HashMap, VecDeque};

fn main() {
    println!("Part2!");

    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut ret: Vec<Vec<char>> = Vec::new();
    for l in input.lines() {
        let mut line: Vec<char> = Vec::new();
        for ch in l.chars() {
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
            // println!("Checking {r}x{c} {:?}", grid[r][c]);

            let mut nodes = vec![vec![0; cols + 1]; rows + 1];

            let mut area = 0;

            let mut to_check: VecDeque<(usize, usize)> = VecDeque::new();
            to_check.push_back((r, c));

            while to_check.len() > 0 {
                let (r1, c1) = to_check.pop_front().unwrap();

                if processed.contains_key(&(r1, c1)) {
                    continue;
                }

                area += 1;

                processed.insert((r1, c1), grid[r1][c1]);
                let current_type = grid[r1][c1];

                //  nodes: (r1,c1) (r1,c1+1)
                //                A
                //       (r1+1,c1) (r+1,c1+1)

                // x1 |  x2  |  x3
                // x4 | -> A |  x5
                // x6 |  x7  |  x8

                // Make sure outside area is not this char
                let mut x1 = if current_type == 'A' { 'B' } else { 'A' };
                let mut x2 = x1;
                let mut x3 = x1;
                let mut x4 = x1;
                let mut x5 = x1;
                let mut x6 = x1;
                let mut x7 = x1;
                let mut x8 = x1;

                if r1 > 0 {
                    if c1 > 0 {
                        x1 = grid[r1 - 1][c1 - 1];
                    }

                    x2 = grid[r1 - 1][c1];

                    if c1 < cols - 1 {
                        x3 = grid[r1 - 1][c1 + 1];
                    }
                }
                if c1 > 0 {
                    x4 = grid[r1][c1 - 1];
                }
                if c1 < cols - 1 {
                    x5 = grid[r1][c1 + 1];
                }

                if r1 < rows - 1 {
                    if c1 > 0 {
                        x6 = grid[r1 + 1][c1 - 1];
                    }

                    x7 = grid[r1 + 1][c1];

                    if c1 < cols - 1 {
                        x8 = grid[r1 + 1][c1 + 1];
                    }
                }

                // println!();
                // println!("checking {r1},{c1}");
                // println!("{x1} {x2} {x3}");
                // println!("{x4} {current_type} {x5}");
                // println!("{x6} {x7} {x8}");

                // Check top left corner
                // -------------------------------------
                //      x1=A  | x2!=A
                //      x4!=A | A
                if x1 == current_type
                    && x2 != current_type
                    && x4 != current_type
                    && nodes[r1][c1] == 1
                {
                    // Set special code if we have visited this node before
                    nodes[r1][c1] = 7;
                } else {
                    nodes[r1][c1] += 1;
                }

                // Check top right corner
                // -------------------------------------
                //      x2!=A | x3=A
                //      A     | x5!=A
                if x2 != current_type
                    && x3 == current_type
                    && x5 != current_type
                    && nodes[r1][c1 + 1] == 1
                {
                    // Set special code if we have visited this node before
                    nodes[r1][c1 + 1] = 7;
                } else {
                    nodes[r1][c1 + 1] += 1;
                }

                // Check bottom right corner
                // -------------------------------------
                //     A     | x5!=A
                //     x7!=A | x8=A
                if x5 != current_type
                    && x7 != current_type
                    && x8 == current_type
                    && nodes[r1 + 1][c1 + 1] == 1
                {
                    // Set special code if we have visited this node before
                    nodes[r1 + 1][c1 + 1] = 7;
                } else {
                    nodes[r1 + 1][c1 + 1] += 1;
                }

                // Check bottom left corner
                // -------------------------------------
                //     x4!= | A
                //     x6=A | x7!=A
                if x4 != current_type
                    && x6 == current_type
                    && x7 != current_type
                    && nodes[r1 + 1][c1] == 1
                {
                    // Set special code if we have visited this node before
                    nodes[r1 + 1][c1] = 7;
                } else {
                    nodes[r1 + 1][c1] += 1;
                }

                // Check up
                if r1 > 0 {
                    if grid[r1 - 1][c1] == grid[r1][c1] {
                        to_check.push_back((r1 - 1, c1));
                    }
                }

                // Check right
                if c1 < cols - 1 {
                    if grid[r1][c1 + 1] == grid[r1][c1] {
                        to_check.push_back((r1, c1 + 1));
                    }
                }

                // Check down
                if r1 < rows - 1 {
                    if grid[r1 + 1][c1] == grid[r1][c1] {
                        to_check.push_back((r1 + 1, c1));
                    }
                }

                // Check left
                if c1 > 0 {
                    if grid[r1][c1 - 1] == grid[r1][c1] {
                        to_check.push_back((r1, c1 - 1));
                    }
                }
            }

            let mut direction_change = 0;
            for y in 0..rows + 1 {
                for x in 0..cols + 1 {
                    if nodes[y][x] == 1 || nodes[y][x] == 3 {
                        direction_change += 1;
                    }
                    if nodes[y][x] == 7 {
                        direction_change += 2;
                    }
                }
            }
            // println!(
            //     "Process area {:?} - {area} with {direction_change} edges",
            //     grid[r][c]
            // );
            total_price += area * direction_change;
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

        assert_eq!(part2(dummy), 436);
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
