use std::{collections::VecDeque, fmt::Display};

use crate::{AoCSolution, grid::Grid, point::Point};

#[derive(Default)]
pub struct Solution {}

impl AoCSolution for Solution {
    type Parsed = Grid;

    fn parse(&self, input: &str) -> Self::Parsed {
        input.parse().unwrap()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut total_price = 0;
        let mut to_check: VecDeque<Point> = VecDeque::new();

        let width = data.width();
        let height = data.height();
        let mut processed = vec![false; width * height];

        for r in 0..height {
            for c in 0..width {
                let index = r * width + c;
                if processed[index] {
                    continue;
                }

                let mut area = 0;
                let mut perimeter = 0;

                processed[index] = true;
                to_check.push_back(Point::new(c as isize, r as isize));

                while let Some(p) = to_check.pop_front() {
                    area += 1;
                    let plant = data[p];

                    for delta in Point::CARDINAL {
                        let n = p + delta;
                        if data.get(n) == Some(plant) {
                            let i = n.y as usize * width + n.x as usize;
                            if !processed[i] {
                                processed[i] = true;
                                to_check.push_back(n);
                            }
                        } else {
                            perimeter += 1;
                        }
                    }
                }
                total_price += area * perimeter;
            }
        }

        total_price
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut total_price = 0;
        let mut to_check: VecDeque<Point> = VecDeque::new();

        let width = data.width();
        let height = data.height();
        let mut processed = vec![false; width * height];

        let node_width = width + 1;
        let node_height = height + 1;
        let mut nodes = vec![0u8; node_width * node_height];
        let mut touched: Vec<usize> = Vec::new();

        for r in 0..height {
            for c in 0..width {
                if processed[r * width + c] {
                    continue;
                }

                let mut area = 0;
                processed[r * width + c] = true;
                to_check.push_back(Point::new(c as isize, r as isize));
                while let Some(p) = to_check.pop_front() {
                    area += 1;

                    let plant = data[p];

                    let same = |dx, dy| data.get(p + Point::new(dx, dy)) == Some(plant);

                    // x1 | x2 | x3
                    // x4 | p  | x5
                    // x6 | x7 | x8
                    let (x1, x2, x3) = (same(-1, -1), same(0, -1), same(1, -1));
                    let (x4, x5) = (same(-1, 0), same(1, 0));
                    let (x6, x7, x8) = (same(-1, 1), same(0, 1), same(1, 1));

                    let top_left = p.y as usize * node_width + p.x as usize;
                    let top_right = top_left + 1;
                    let bottom_left = top_left + node_width;
                    let bottom_right = bottom_left + 1;

                    // diagonal: only the diagonal cell is in the region, the two sides are not
                    let mut visit = |node: usize, diagonal: bool| {
                        if nodes[node] == 0 {
                            touched.push(node);
                        }
                        if diagonal && nodes[node] == 1 {
                            // Set special code if we have visited this node before
                            nodes[node] = 7;
                        } else {
                            nodes[node] += 1;
                        }
                    };

                    visit(top_left, x1 && !x2 && !x4);
                    visit(top_right, x3 && !x2 && !x5);
                    visit(bottom_right, x8 && !x5 && !x7);
                    visit(bottom_left, x6 && !x4 && !x7);

                    for delta in Point::CARDINAL {
                        let n = p + delta;
                        if data.get(n) == Some(plant) {
                            let i = n.y as usize * width + n.x as usize;
                            if !processed[i] {
                                processed[i] = true;
                                to_check.push_back(n);
                            }
                        }
                    }
                }

                let mut direction_change = 0;
                for &node in &touched {
                    direction_change += match nodes[node] {
                        1 | 3 => 1,
                        7 => 2,
                        _ => 0,
                    };
                    nodes[node] = 0;
                }
                touched.clear();

                total_price += area * direction_change;
            }
        }

        total_price
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
AAAA
BBCD
BBCC
EEEC
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "140");
    }

    #[test]
    fn test_part2_1() {
        let dummy = "AAAA
BBCD
BBCC
EEEC";

        assert_eq!(Solution::default().solve2(dummy), "80");
    }

    #[test]
    fn test_part2_2() {
        let dummy = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

        assert_eq!(Solution::default().solve2(dummy), "236");
    }

    #[test]
    fn test_part2_3() {
        let dummy = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

        assert_eq!(Solution::default().solve2(dummy), "368");
    }

    #[test]
    fn test_part2_4() {
        let dummy = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

        assert_eq!(Solution::default().solve2(dummy), "436");
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

        assert_eq!(Solution::default().solve2(dummy), "1206");
    }
}
