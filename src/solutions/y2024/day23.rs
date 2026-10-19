use std::fmt::Display;

use crate::{AoCSolution, parsing::input_lines};

#[derive(Default)]
pub struct Solution {}

fn node_to_id(n: &[u8]) -> usize {
    let b1 = n[0] - b'a';
    let b2 = n[1] - b'a';

    b1 as usize * 26 + b2 as usize
}

fn id_to_node(id: usize) -> String {
    let a = (id / 26) as u8 + b'a';
    let b = (id % 26) as u8 + b'a';
    format!("{}{}", a as char, b as char)
}

fn check_for_network(
    group: &mut Vec<usize>,
    candidates: &[usize],
    data: &[Vec<bool>],
    best: &mut Vec<usize>,
) {
    if group.len() > best.len() {
        *best = group.clone();
    }

    if group.len() + candidates.len() <= best.len() {
        return;
    }

    for (i, &node) in candidates.iter().enumerate() {
        let next: Vec<usize> = candidates[i + 1..]
            .iter()
            .copied()
            .filter(|&j| data[node][j])
            .collect();
        group.push(node);
        check_for_network(group, &next, data, best);
        group.pop();
    }
}
impl AoCSolution for Solution {
    type Parsed = Vec<Vec<bool>>;

    fn parse(&self, input: &str) -> Self::Parsed {
        let mut ret = vec![vec![false; 26 * 26]; 26 * 26];

        for line in input_lines(input) {
            let a = node_to_id(&line.as_bytes()[..2]);
            let b = node_to_id(&line.as_bytes()[3..]);
            ret[a][b] = true;
            ret[b][a] = true;
        }
        ret
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut ret = 0;
        for i in 0..26 * 26 {
            for j in (i + 1)..26 * 26 {
                if data[i][j] {
                    for k in (j + 1)..26 * 26 {
                        if data[i][k]
                            && data[j][k]
                            && (i / 26 == 19 || j / 26 == 19 || k / 26 == 19)
                        {
                            ret += 1;
                        }
                    }
                }
            }
        }

        ret
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut best = Vec::new();

        for n in 0..26 * 26 {
            // Get all remaining nodes this node connects to
            let candidates: Vec<usize> = ((n + 1)..26 * 26).filter(|&m| data[n][m]).collect();
            check_for_network(&mut vec![n], &candidates, data, &mut best);
        }
        best.iter()
            .map(|&id| id_to_node(id))
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "7");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "co,de,ka,ta");
    }
}
