use std::{collections::HashMap, fmt::Display};

use crate::AoCSolution;

#[derive(Debug, Default)]
pub struct Solution {}

const END: usize = id_from_string("out".as_bytes());
const YOU: usize = id_from_string("you".as_bytes());
const SVR: usize = id_from_string("svr".as_bytes());
const FFT: usize = id_from_string("fft".as_bytes());
const DAC: usize = id_from_string("dac".as_bytes());

const fn id_from_string(id: &[u8]) -> usize {
    let mut ret = 0;
    let mut i = 0;
    while i < 3 {
        ret = ret * 26 + (id[i] - b'a') as usize;
        i += 1;
    }

    ret
}

fn count_paths(data: &Vec<Vec<usize>>, id: usize) -> usize {
    let mut ret = 0;

    for &node in data[id].iter() {
        if node == END {
            ret += 1
        } else {
            ret += count_paths(data, node);
        }
    }
    ret
}

fn count_dac_and_fft_paths(
    data: &Vec<Vec<usize>>,
    cache: &mut HashMap<(bool, bool, usize), usize>,
    fft_passed: bool,
    dac_passed: bool,
    id: usize,
) -> usize {
    let mut ret = 0;
    let key = (fft_passed, dac_passed, id);

    if let Some(cache_val) = cache.get(&key) {
        ret += cache_val;
    } else {
        for &node in data[id].iter() {
            if node == END {
                if dac_passed && fft_passed {
                    ret += 1;
                }
            } else {
                let val = count_dac_and_fft_paths(
                    data,
                    cache,
                    fft_passed || node == FFT,
                    dac_passed || node == DAC,
                    node,
                );
                ret += val;
            }
        }
    }
    cache.insert(key, ret);
    ret
}

impl AoCSolution for Solution {
    type Parsed = Vec<Vec<usize>>;

    fn parse(&self, input: &str) -> Self::Parsed {
        let mut ret = vec![vec![]; id_from_string("zzz".as_bytes()) + 1];

        for line in input.trim().as_bytes().split(|&b| b == b'\n') {
            let key = id_from_string(&line[..3]);

            let mut i = 5; // First node list
            while i + 2 < line.len() {
                ret[key].push(id_from_string(&line[i..=i + 2]));
                i += 4; // 3 chars + 1 space
            }
        }
        ret
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        count_paths(data, YOU)
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut map = HashMap::new();
        count_dac_and_fft_paths(data, &mut map, false, false, SVR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "5");
    }

    #[test]
    fn test_part2() {
        let example_2: &str = r"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
        assert_eq!(Solution::default().solve2(example_2), "2");
    }
}
