use crate::AoCSolution;

pub mod y2015;
pub mod y2024;
pub mod y2025;

pub fn lookup(year: u16, day: u8) -> Option<Box<dyn AoCSolution>> {
    use y2015::*;
    use y2024::*;
    use y2025::*;

    Some(match (year, day) {
        (2015, 1) => Box::new(y2015d01::Solution {}),
        (2015, 2) => Box::new(y2015d02::Solution {}),
        (2015, 3) => Box::new(y2015d03::Solution {}),
        (2015, 4) => Box::new(y2015d04::Solution {}),
        (2015, 5) => Box::new(y2015d05::Solution::default()),

        (2024, 1) => Box::new(y2024d01::Solution {}),
        (2024, 2) => Box::new(y2024d02::Solution {}),
        (2024, 3) => Box::new(y2024d03::Solution {}),
        (2024, 4) => Box::new(y2024d04::Solution {}),
        (2024, 5) => Box::new(y2024d05::Solution {}),
        (2024, 6) => Box::new(y2024d06::Solution {}),
        (2024, 7) => Box::new(y2024d07::Solution {}),
        (2024, 8) => Box::new(y2024d08::Solution {}),
        (2024, 9) => Box::new(y2024d09::Solution {}),

        (2025, 1) => Box::new(y2025d01::Solution {}),
        (2025, 2) => Box::new(y2025d02::Solution {}),
        (2025, 3) => Box::new(y2025d03::Solution {}),
        (2025, 4) => Box::new(y2025d04::Solution {}),
        (2025, 5) => Box::new(y2025d05::Solution {}),
        (2025, 6) => Box::new(y2025d06::Solution {}),
        (2025, 7) => Box::new(y2025d07::Solution {}),
        (2025, 8) => Box::new(y2025d08::Solution {
            num_operations_part1: 1000,
            ..Default::default()
        }),
        (2025, 9) => Box::new(y2025d09::Solution::default()),
        (2025, 10) => Box::new(y2025d10::Solution {}),

        _ => return None,
    })
}
