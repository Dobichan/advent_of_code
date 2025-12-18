use crate::AoCSolution;

pub mod y2025d01;
pub mod y2025d02;
pub mod y2025d03;
pub mod y2025d04;
pub mod y2025d05;
pub mod y2025d06;
pub mod y2025d07;
pub mod y2025d08;
pub mod y2025d09;
pub mod y2025d10;
// pub mod y2025d11;
// pub mod y2025d12;

pub fn run(input: &str, day: u8, part: char, dryrun: bool) {
    let mut solution: Box<dyn AoCSolution>;
    match day {
        1 => solution = Box::new(y2025d01::Solution {}),
        2 => solution = Box::new(y2025d02::Solution {}),
        3 => solution = Box::new(y2025d03::Solution {}),
        4 => solution = Box::new(y2025d04::Solution {}),
        5 => solution = Box::new(y2025d05::Solution {}),
        6 => solution = Box::new(y2025d06::Solution {}),
        7 => solution = Box::new(y2025d07::Solution {}),
        8 => {
            solution = Box::new(y2025d08::Solution {
                num_operations_part1: 1000,
                ..Default::default()
            })
        }
        9 => {
            solution = Box::new(y2025d09::Solution {
                ..Default::default()
            })
        }
        10 => solution = Box::new(y2025d10::Solution {}),
        // 11 => solution =Box::new( y2025d11::Solution {}),
        // 12 => solution =Box::new( y2025d12::Solution {}),
        _ => panic!("Illegal day {} for 2025", day),
    }
    match part {
        '1' => solution.run_part1(input, dryrun),
        '2' => solution.run_part2(input, dryrun),
        'a' => solution.run(input, dryrun),
        _ => panic!("Illegal part option, use 1,2 or a"),
    }
}
