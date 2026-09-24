use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display};

use nom::{IResult, Parser, bytes::complete::tag, character::complete::isize};

use crate::{AoCSolution, parsing::input_lines, point::Point3};

const PART1_CONNECTIONS: usize = 1000;

#[derive(Debug)]
pub struct Solution {
    pub(crate) num_operations_part1: usize,
}

impl Default for Solution {
    fn default() -> Self {
        Self {
            num_operations_part1: PART1_CONNECTIONS,
        }
    }
}

#[derive(Debug)]
pub struct Network {
    pub boxes: Vec<Point3>,
    pub pairs: Vec<(isize, (usize, usize))>,
}

fn get_coordinates(input: &str) -> IResult<&str, Point3> {
    (isize, tag(","), isize, tag(","), isize)
        .map(|(x, _, y, _, z)| Point3::new(x, y, z))
        .parse(input)
}

fn sorted_pairs(boxes: &[Point3]) -> Vec<(isize, (usize, usize))> {
    let mut pairs = Vec::with_capacity(boxes.len() * boxes.len().saturating_sub(1) / 2);
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            pairs.push((boxes[i].distance_squared(boxes[j]), (i, j)));
        }
    }
    pairs
}

fn join(circuits: &mut Vec<Vec<usize>>, a: usize, b: usize) {
    let find = |x: usize| circuits.iter().position(|c| c.contains(&x));

    match (find(a), find(b)) {
        (None, None) => circuits.push(vec![a, b]),
        (Some(i), None) => circuits[i].push(b),
        (None, Some(j)) => circuits[j].push(a),
        (Some(i), Some(j)) if i != j => {
            let mut moved = circuits.remove(i.max(j));
            circuits[i.min(j)].append(&mut moved);
        }
        (Some(_), Some(_)) => {} // already same circuit
    }
}

impl AoCSolution for Solution {
    type Parsed = Network;

    fn parse(&self, input: &str) -> Self::Parsed {
        let boxes: Vec<Point3> = input_lines(input)
            .map(|line| {
                get_coordinates(line)
                    .expect("Failed to parse coordinates")
                    .1
            })
            .collect();
        let pairs = sorted_pairs(&boxes);
        Network { boxes, pairs }
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut pairs = data.pairs.clone();
        pairs.select_nth_unstable(self.num_operations_part1);

        let mut circuits: Vec<Vec<usize>> = Vec::new();
        for &(_, (a, b)) in &pairs[..self.num_operations_part1] {
            join(&mut circuits, a, b);
        }

        circuits.sort_by_key(|c| Reverse(c.len()));
        circuits.iter().take(3).map(Vec::len).product::<usize>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut circuits: Vec<Vec<usize>> = Vec::new();

        let mut heap: BinaryHeap<_> = data.pairs.iter().copied().map(Reverse).collect();

        while let Some(Reverse((_, (a, b)))) = heap.pop() {
            join(&mut circuits, a, b);
            if circuits.len() == 1 && circuits[0].len() == data.boxes.len() {
                return data.boxes[a].x * data.boxes[b].x;
            }
        }

        panic!("Failed to form a single circuit")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_part1() {
        let sol = Solution {
            num_operations_part1: 10,
        };

        assert_eq!(sol.solve1(EXAMPLE_INPUT), "40");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "25272");
    }
}
