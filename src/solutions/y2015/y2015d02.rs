use crate::AoCSolution;
use crate::parsing::input_to_vectors;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::sequence::separated_pair;
use std::cmp::min;

const YEAR: u16 = 2015;
const DAY: u8 = 2;
pub struct Solution {}

fn get_dimmentions(input: &str) -> IResult<&str, (i64, i64, i64)> {
    let inner_parser = separated_pair(i64, tag("x"), i64);
    separated_pair(i64, tag("x"), inner_parser)
        .map(|(l, (w, h))| (l, w, h))
        .parse(input)
    // Ok((input, (0, 0, 0)))
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }

    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> String {
        let mut total = 0;
        for line in input_to_vectors(input) {
            let (_, (l, w, h)) = get_dimmentions(&line).unwrap();
            let area_1 = l * w;
            let area_2 = w * h;
            let area_3 = h * l;

            let extra = min(area_1, min(area_2, area_3));
            total += 2 * area_1 + 2 * area_2 + 2 * area_3 + extra;
        }
        total.to_string()
    }
    fn part2(&self, input: &str) -> String {
        let mut total = 0;
        for line in input_to_vectors(input) {
            let (_, (l, w, h)) = get_dimmentions(&line).unwrap();
            let mut dimmentions: Vec<_> = vec![l, w, h];
            dimmentions.sort();

            let extra = l * w * h;
            total += 2 * dimmentions[0] + 2 * dimmentions[1] + extra;
        }
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT1: &str = r"2x3x4";
        let sol = Solution {};
        assert_eq!(sol.part1(&INPUT1), "58");

        const INPUT2: &str = r"1x1x10";
        assert_eq!(sol.part1(&INPUT2), "43");
    }

    #[test]
    fn test_part2() {
        const INPUT1: &str = r"2x3x4";
        let sol = Solution {};
        assert_eq!(sol.part2(&INPUT1), "34");

        const INPUT2: &str = r"1x1x10";
        assert_eq!(sol.part2(&INPUT2), "14");
    }
}
