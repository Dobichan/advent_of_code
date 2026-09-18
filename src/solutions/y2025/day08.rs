use std::{cmp::Reverse, fmt::Display};

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

// fn create_distances() {
//     let mut distanses = Vec::with_capacity(self.num_operations_part1 * self.num_operations_part1);
//     let num_boxes = self.boxes.as_ref().unwrap().len();

//     for i in 0..num_boxes - 1 {
//         for j in i + 1..num_boxes {
//             let from = &self.boxes.as_ref().unwrap()[i];
//             let to = &self.boxes.as_ref().unwrap()[j];

//             let dist = from.distance(to);
//             distanses.push((dist, (i, j)));
//         }
//     }

//     distanses.sort();
//     self.distances = Some(distanses);
// }

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
    pairs.sort_unstable();
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
        let mut circuits: Vec<Vec<usize>> = Vec::new();

        for &(_, (a, b)) in data.pairs.iter().take(self.num_operations_part1) {
            join(&mut circuits, a, b);
        }

        circuits.sort_by_key(|c| Reverse(c.len()));
        circuits.iter().take(3).map(Vec::len).product::<usize>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut circuits: Vec<Vec<usize>> = Vec::new();

        for &(_, (a, b)) in &data.pairs {
            join(&mut circuits, a, b);
            if circuits.len() == 1 && circuits[0].len() == data.boxes.len() {
                return data.boxes[a].x * data.boxes[b].x;
            }
        }
        panic!("Failed to form a single circuit")

        // for i in 0..self.distances.as_ref().unwrap().len() {
        // println!("{i}: {:?}", &self.distances.as_ref().unwrap()[i]);
        // }

        // let mut circuits: Vec<Vec<_>> = Vec::new();
        // let mut unused_boxes = self.boxes.as_ref().unwrap().len();
        // let mut answer = 0;

        // for i in 0..self.distances.as_ref().unwrap().len() {
        //     let a = self.distances.as_ref().unwrap()[i].1.0;
        //     let b = self.distances.as_ref().unwrap()[i].1.1;

        //     // println!(
        //     //     "\nChecking {a} and {b} - {:?}-{:?}  ",
        //     //     self.boxes.as_ref().unwrap()[a],
        //     //     self.boxes.as_ref().unwrap()[b]
        //     // );
        //     let mut a_index = None;
        //     let mut b_index = None;

        //     for (circuit_index, circuit) in circuits.iter().enumerate() {
        //         // println!("{circuit_index}-{:?}  ", circuit);
        //         if circuit.contains(&a) {
        //             if a_index.is_some() {
        //                 panic!("{a} found before!");
        //             }
        //             a_index = Some(circuit_index);
        //         }
        //         if circuit.contains(&b) {
        //             if b_index.is_some() {
        //                 panic!("{b} found before!");
        //             }
        //             b_index = Some(circuit_index);
        //         }
        //     }

        //     if a_index.is_none() && b_index.is_none() {
        //         // println!("new circuit");
        //         circuits.push(vec![a, b]);
        //         unused_boxes -= 2;
        //     } else if a_index.is_some() && b_index.is_none() {
        //         // println!("{b} is added to {:?}", a_index);
        //         circuits[a_index.unwrap()].push(b);
        //         unused_boxes -= 1;
        //     } else if let (None, Some(b_index)) = (a_index, b_index) {
        //         // println!("{a} is added to {:?}", b_index);
        //         circuits[b_index].push(a);
        //         unused_boxes -= 1;
        //     } else if a_index.unwrap() != b_index.unwrap() {
        //         // Merge a and b
        //         let a_index = a_index.unwrap();
        //         let b_index = b_index.unwrap();

        //         let from = a_index.max(b_index);
        //         let to = a_index.min(b_index);
        //         // println!("merging {from} into {to} ({a_index}-{b_index})");
        //         let mut append_circuit = circuits.remove(from);
        //         circuits[to].append(&mut append_circuit);
        //     } else {
        //         // println!("{a} and {b} already in {:?}({:?})", a_index, b_index);
        //     }
        //     // println!("{i} - {:?}", circuits);

        //     if unused_boxes == 0 {
        //         // println!(
        //         //     "Solution: {a}-{b} {:?}-{:?}",
        //         //     self.boxes.as_ref().unwrap()[a],
        //         //     self.boxes.as_ref().unwrap()[b]
        //         // );
        //         // println!("{:?}", circuits);

        //         answer = self.boxes.as_ref().unwrap()[a].x * self.boxes.as_ref().unwrap()[b].x;
        //         // println!("Answer: {answer}");
        //         break;
        //     }
        // }

        // // println!("\n\n{:?}", circuits);

        // answer.to_string()
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
