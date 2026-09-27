use std::fmt::Display;

use crate::{AoCSolution, parsing::input_lines};

#[derive(Copy, Clone, Debug, Default)]
pub struct Present {
    _index: usize,
    space_used: usize,
}

#[derive(Debug)]
pub struct Region {
    w: usize,
    h: usize,
    usage: [usize; 6],
}

impl Region {
    fn space_utilization(&self, present: &[Present]) -> f64 {
        let total_space_used = self
            .usage
            .iter()
            .enumerate()
            .map(|(i, &u)| present[i].space_used * u)
            .sum::<usize>();

        let total_area = self.w * self.h;

        total_space_used as f64 / total_area as f64
    }
}

#[derive(Default)]
pub struct Solution {}

impl AoCSolution for Solution {
    type Parsed = ([Present; 6], Vec<Region>);

    fn parse(&self, input: &str) -> Self::Parsed {
        let mut presents = [Present::default(); 6];
        let mut input = input_lines(input);
        for p in &mut presents {
            let index = input.next().unwrap().as_bytes().first().unwrap() - b'0';
            let line1 = input.next().unwrap().chars().filter(|&c| c == '#').count();
            let line2 = input.next().unwrap().chars().filter(|&c| c == '#').count();
            let line3 = input.next().unwrap().chars().filter(|&c| c == '#').count();
            *p = Present {
                _index: index as usize,
                space_used: line1 + line2 + line3,
            };

            // Eat the empty line
            input.next();
        }

        let mut regions = Vec::new();
        for l in input {
            let (a, b) = l.split_once(':').unwrap();
            let (w, h) = a.split_once('x').unwrap();
            let w: usize = w.parse().unwrap();
            let h: usize = h.parse().unwrap();

            let usage: Vec<usize> = b
                .trim()
                .split(' ')
                .map(|e| e.parse::<usize>().unwrap())
                .collect();
            regions.push(Region {
                w,
                h,
                usage: usage.try_into().expect("Failed to parse usages"),
            });
        }

        (presents, regions)
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        // let test: Vec<_> = data
        //     .1
        //     .iter()
        //     .map(|elem| (elem.space_utilization(&data.0), elem))
        //     .sorted_by(|a, b| a.0.total_cmp(&b.0))
        //     .collect();

        // let mut ret = 0;
        // for t in test {
        //     if t.0 < 1.0 {
        //         ret += 1;
        //     }
        // }
        // ret
        data.1
            .iter()
            .map(|elem| elem.space_utilization(&data.0))
            .filter(|&usage| usage < 1.0)
            .count()
    }

    fn part2(&self, _data: &Self::Parsed) -> impl Display {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn test_part1() {
        // let temp = include_str!(
        //     "/home/haralda/workspace/rust_projects/advent_of_code/input/2025/day12.txt"
        // );

        // Solution::default().solve1(temp);
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "0");
    }
}
