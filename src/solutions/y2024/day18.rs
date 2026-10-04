use std::fmt::Display;

use crate::{AoCSolution, grid::Grid, parsing::input_lines, point::Point};

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const ITERATIONS: usize = 1024;

pub struct Solution {
    width: usize,
    height: usize,
    iterations: usize,
}

impl Default for Solution {
    fn default() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            iterations: ITERATIONS,
        }
    }
}

impl Solution {
    fn blocked_grid(&self, points: &[Point], num_points: usize) -> Grid<bool> {
        let mut grid = Grid::new(self.width, self.height, false);
        for &p in &points[..num_points] {
            grid[p] = true;
        }
        grid
    }

    fn has_path(&self, start: Point, end: Point, points: &[Point], num_points: usize) -> bool {
        self.blocked_grid(points, num_points)
            .shortest_path(start, end, |b| !b)
            .is_some()
    }
}

impl AoCSolution for Solution {
    type Parsed = Vec<Point>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input_lines(input)
            .map(|l| {
                let (x, y) = l.split_once(',').unwrap();
                Point::new(x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
            })
            .collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let start = Point::new(0, 0);
        let end = Point::new((self.width - 1) as isize, (self.height - 1) as isize);

        self.blocked_grid(data, self.iterations)
            .shortest_path(start, end, |b| !b)
            .expect("No path found!")
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let start = Point::new(0, 0);
        let end = Point::new((self.width - 1) as isize, (self.height - 1) as isize);

        // Do a binary search of data to quickly find which one blocks
        let mut low = self.iterations; // From part1, we know this is not blocked
        let mut high = data.len();

        while high - low > 1 {
            let mid = (low + high) / 2;
            if self.has_path(start, end, data, mid) {
                low = mid;
            } else {
                high = mid;
            }
        }
        let p = data[low];
        format!("{},{}", p.x, p.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test_part1() {
        let sol = Solution {
            width: 7,
            height: 7,
            iterations: 12,
        };

        assert_eq!(sol.solve1(EXAMPLE_INPUT), "24");
    }

    #[test]
    fn test_part2() {
        let sol = Solution {
            width: 7,
            height: 7,
            iterations: 12,
        };

        assert_eq!(sol.solve2(EXAMPLE_INPUT), "6,1");
    }
}
