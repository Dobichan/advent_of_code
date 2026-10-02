use std::fmt::Display;

use crate::{AoCSolution, parsing::input_lines, point::Point};

const WIDTH: isize = 101;
const HEIGHT: isize = 103;
const ITERATIONS: isize = 100;

#[derive(Debug)]
pub struct Solution {
    pub(crate) width: isize,
    pub(crate) height: isize,
}

impl Default for Solution {
    fn default() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
        }
    }
}

#[derive(Debug)]
pub struct Robot {
    pos: Point,
    moving: Point,
}

impl TryFrom<&str> for Robot {
    type Error = String;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (a, b) = line.split_once(' ').unwrap();
        let (a, b) = (&a[2..], &b[2..]);

        let (ax, ay) = a.split_once(',').unwrap();
        let (ax, ay) = (ax.parse::<isize>().unwrap(), ay.parse::<isize>().unwrap());
        let (bx, by) = b.split_once(',').unwrap();
        let (bx, by) = (bx.parse::<isize>().unwrap(), by.parse::<isize>().unwrap());

        Ok(Self {
            pos: Point::new(ax, ay),
            moving: Point::new(bx, by),
        })
    }
}

impl Solution {
    fn safety_factor(&self, robots: &[Robot], seconds: isize) -> usize {
        let (width, height) = (self.width, self.height);
        let mut quadrants: [usize; 4] = [0, 0, 0, 0];

        for robot in robots {
            let mut x = (robot.pos.x + robot.moving.x * seconds) % width;
            let mut y = (robot.pos.y + robot.moving.y * seconds) % height;
            if x < 0 {
                x += width;
            }
            if y < 0 {
                y += height;
            }

            // Quadrant1
            if x < width / 2 && y < height / 2 {
                quadrants[0] += 1;
            }

            // Quadrant2
            if x > width / 2 && y < height / 2 {
                quadrants[1] += 1;
            }
            // Quadrant3
            if x < width / 2 && y > height / 2 {
                quadrants[2] += 1;
            }
            // Quadrant4
            if x > width / 2 && y > height / 2 {
                quadrants[3] += 1;
            }
        }

        quadrants.iter().product()
    }
}

impl AoCSolution for Solution {
    type Parsed = Vec<Robot>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input_lines(input)
            .map(|l| Robot::try_from(l).unwrap())
            .collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        self.safety_factor(data, ITERATIONS)
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        (0..self.width * self.height)
            .min_by_key(|&seconds| self.safety_factor(data, seconds))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_part1() {
        let sol = Solution {
            width: 11,
            height: 7,
        };

        assert_eq!(sol.solve1(EXAMPLE_INPUT), "12");
    }
}
