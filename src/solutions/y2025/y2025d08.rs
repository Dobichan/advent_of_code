use crate::AoCSolution;

const YEAR: u16 = 2025;
const DAY: u8 = 8;

#[derive(Default)]
pub struct Solution {
    pub(crate) num_operations_part1: usize,
    pub(crate) boxes: Option<Vec<JunctionBox>>,
    pub(crate) distances: Option<Vec<(i64, (usize, usize))>>,
}

#[derive(Debug)]
pub struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn new(input: &str) -> Self {
        let mut elems = input.split(',');
        JunctionBox {
            x: elems.next().unwrap().parse().unwrap(),
            y: elems.next().unwrap().parse().unwrap(),
            z: elems.next().unwrap().parse().unwrap(),
        }
    }

    fn distance(&self, other: &JunctionBox) -> i64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;

        dx * dx + dy * dy + dz * dz
    }
}

impl Solution {
    fn create_boxes(&mut self, input: &str) {
        let mut boxes = Vec::with_capacity(1000);
        for line in input.trim().lines() {
            boxes.push(JunctionBox::new(line));
        }
        self.boxes = Some(boxes);
    }

    fn create_distances(&mut self) {
        let mut distanses =
            Vec::with_capacity(self.num_operations_part1 * self.num_operations_part1);
        let num_boxes = self.boxes.as_ref().unwrap().len();

        for i in 0..num_boxes - 1 {
            for j in i + 1..num_boxes {
                let from = &self.boxes.as_ref().unwrap()[i];
                let to = &self.boxes.as_ref().unwrap()[j];

                let dist = from.distance(to);
                distanses.push((dist, (i, j)));
            }
        }

        distanses.sort();
        self.distances = Some(distanses);
    }
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str) -> String {
        if self.boxes.is_none() {
            self.create_boxes(input);
        }

        if self.distances.is_none() {
            self.create_distances();
        }

        // println!("{:?}", &self.distances.as_ref().unwrap()[0..10]);

        let mut circuits: Vec<Vec<_>> = Vec::new();

        for i in 0..self
            .num_operations_part1
            .min(self.distances.as_ref().unwrap().len())
        {
            let a = self.distances.as_ref().unwrap()[i].1.0;
            let b = self.distances.as_ref().unwrap()[i].1.1;

            // println!(
            //     "\nChecking {a} and {b} - {:?}-{:?}  ",
            //     self.boxes.as_ref().unwrap()[a],
            //     self.boxes.as_ref().unwrap()[b]
            // );
            let mut a_index = None;
            let mut b_index = None;

            for circuit_index in 0..circuits.len() {
                let circuit = &circuits[circuit_index];
                // println!("{circuit_index}-{:?}  ", circuit);
                if circuit.contains(&a) {
                    if a_index.is_some() {
                        panic!("{a} found before!");
                    }
                    a_index = Some(circuit_index);
                }
                if circuit.contains(&b) {
                    if b_index.is_some() {
                        panic!("{b} found before!");
                    }
                    b_index = Some(circuit_index);
                }
            }

            if a_index.is_none() && b_index.is_none() {
                // println!("new circuit");
                circuits.push(vec![a, b]);
            } else if a_index.is_some() && b_index.is_none() {
                // println!("{b} is added to {:?}", a_index);
                circuits[a_index.unwrap()].push(b);
            } else if a_index.is_none() && b_index.is_some() {
                // println!("{a} is added to {:?}", b_index);
                circuits[b_index.unwrap()].push(a);
            } else if a_index.unwrap() != b_index.unwrap() {
                // Merge a and b
                let a_index = a_index.unwrap();
                let b_index = b_index.unwrap();

                let from = a_index.max(b_index);
                let to = a_index.min(b_index);
                // println!("merging {from} into {to} ({a_index}-{b_index})");
                let mut append_circuit = circuits.remove(from);
                circuits[to].append(&mut append_circuit);
            } else {
                // println!("{a} and {b} already in {:?}({:?})", a_index, b_index);
            }

            // println!("{i} - {:?}", circuits);
        }

        // println!("\n\n{:?}", circuits);
        circuits.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());

        // println!("{:?}", circuits);

        (circuits[0].len() * circuits[1].len() * circuits[2].len()).to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        if self.boxes.is_none() {
            self.create_boxes(input);
        }

        if self.distances.is_none() {
            self.create_distances();
        }

        // for i in 0..self.distances.as_ref().unwrap().len() {
        // println!("{i}: {:?}", &self.distances.as_ref().unwrap()[i]);
        // }

        let mut circuits: Vec<Vec<_>> = Vec::new();
        let mut unused_boxes = self.boxes.as_ref().unwrap().len();
        let mut answer = 0;

        for i in 0..self.distances.as_ref().unwrap().len() {
            let a = self.distances.as_ref().unwrap()[i].1.0;
            let b = self.distances.as_ref().unwrap()[i].1.1;

            // println!(
            //     "\nChecking {a} and {b} - {:?}-{:?}  ",
            //     self.boxes.as_ref().unwrap()[a],
            //     self.boxes.as_ref().unwrap()[b]
            // );
            let mut a_index = None;
            let mut b_index = None;

            for circuit_index in 0..circuits.len() {
                let circuit = &circuits[circuit_index];
                // println!("{circuit_index}-{:?}  ", circuit);
                if circuit.contains(&a) {
                    if a_index.is_some() {
                        panic!("{a} found before!");
                    }
                    a_index = Some(circuit_index);
                }
                if circuit.contains(&b) {
                    if b_index.is_some() {
                        panic!("{b} found before!");
                    }
                    b_index = Some(circuit_index);
                }
            }

            if a_index.is_none() && b_index.is_none() {
                // println!("new circuit");
                circuits.push(vec![a, b]);
                unused_boxes -= 2;
            } else if a_index.is_some() && b_index.is_none() {
                // println!("{b} is added to {:?}", a_index);
                circuits[a_index.unwrap()].push(b);
                unused_boxes -= 1;
            } else if a_index.is_none() && b_index.is_some() {
                // println!("{a} is added to {:?}", b_index);
                circuits[b_index.unwrap()].push(a);
                unused_boxes -= 1;
            } else if a_index.unwrap() != b_index.unwrap() {
                // Merge a and b
                let a_index = a_index.unwrap();
                let b_index = b_index.unwrap();

                let from = a_index.max(b_index);
                let to = a_index.min(b_index);
                // println!("merging {from} into {to} ({a_index}-{b_index})");
                let mut append_circuit = circuits.remove(from);
                circuits[to].append(&mut append_circuit);
            } else {
                // println!("{a} and {b} already in {:?}({:?})", a_index, b_index);
            }
            // println!("{i} - {:?}", circuits);

            if unused_boxes == 0 {
                // println!(
                //     "Solution: {a}-{b} {:?}-{:?}",
                //     self.boxes.as_ref().unwrap()[a],
                //     self.boxes.as_ref().unwrap()[b]
                // );
                // println!("{:?}", circuits);

                answer = self.boxes.as_ref().unwrap()[a].x * self.boxes.as_ref().unwrap()[b].x;
                // println!("Answer: {answer}");
                break;
            }
        }

        // println!("\n\n{:?}", circuits);

        answer.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = "162,817,812\n\
                57,618,57\n\
                906,360,560\n\
                592,479,940\n\
                352,342,300\n\
                466,668,158\n\
                542,29,236\n\
                431,825,988\n\
                739,650,466\n\
                52,470,668\n\
                216,146,977\n\
                819,987,18\n\
                117,168,530\n\
                805,96,715\n\
                346,949,466\n\
                970,615,88\n\
                941,993,340\n\
                862,61,35\n\
                984,92,344\n\
                425,690,689";

        let mut sol = Solution {
            num_operations_part1: 10,
            ..Default::default()
        };
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "40");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = "162,817,812\n\
                57,618,57\n\
                906,360,560\n\
                592,479,940\n\
                352,342,300\n\
                466,668,158\n\
                542,29,236\n\
                431,825,988\n\
                739,650,466\n\
                52,470,668\n\
                216,146,977\n\
                819,987,18\n\
                117,168,530\n\
                805,96,715\n\
                346,949,466\n\
                970,615,88\n\
                941,993,340\n\
                862,61,35\n\
                984,92,344\n\
                425,690,689";

        let mut sol = Solution {
            num_operations_part1: 10,
            ..Default::default()
        };
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "25272");
    }
}
