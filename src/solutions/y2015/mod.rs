use crate::AoCSolution;

pub mod y2015d01;
pub mod y2015d02;
pub mod y2015d03;
pub mod y2015d04;
pub mod y2015d05;
// pub mod y2015d06;
// pub mod y2015d07;
// pub mod y2015d08;
// pub mod y2015d08;
// pub mod y2015d09;
// pub mod y2015d10;
// pub mod y2015d11;
// pub mod y2015d12;
// pub mod y2015d13;
// pub mod y2015d14;
// pub mod y2015d15;
// pub mod y2015d16;
// pub mod y2015d17;
// pub mod y2015d18;
// pub mod y2015d19;
// pub mod y2015d20;
// pub mod y2015d21;
// pub mod y2015d22;
// pub mod y2015d23;
// pub mod y2015d24;
// pub mod y2015d25;

pub fn run(input: &str, day: u8, part: char, dryrun: bool) {
    let mut solution: Box<dyn AoCSolution>;
    match day {
        1 => solution = Box::new(y2015d01::Solution {}),
        2 => solution = Box::new(y2015d02::Solution {}),
        3 => solution = Box::new(y2015d03::Solution {}),
        4 => solution = Box::new(y2015d04::Solution {}),
        5 => solution = Box::new(y2015d05::Solution::default()),
        // 6 =>solution = Box::new(y2015d06::Solution{}),
        // 7 =>solution = Box::new(y2015d07::Solution{}),
        // 8 =>solution = Box::new(y2015d08::Solution{}),
        // 9 =>solution = Box::new(y2015d09::Solution{}),
        // 10 =>solution =Box::new( y2015d10::Solution{}),
        // 11 =>solution =Box::new( y2015d11::Solution{}),
        // 12 =>solution =Box::new( y2015d12::Solution{}),
        // 13 =>solution =Box::new( y2015d13::Solution{}),
        // 14 =>solution =Box::new( y2015d14::Solution{}),
        // 15 =>solution =Box::new( y2015d15::Solution{}),
        // 16 =>solution =Box::new( y2015d16::Solution{}),
        // 17 =>solution =Box::new( y2015d17::Solution{}),
        // 18 =>solution =Box::new( y2015d18::Solution{}),
        // 19 =>solution =Box::new( y2015d19::Solution{}),
        // 20 =>solution =Box::new( y2015d20::Solution{}),
        // 21 =>solution =Box::new( y2015d21::Solution{}),
        // 22 =>solution =Box::new( y2015d22::Solution{}),
        // 23 =>solution =Box::new( y2015d23::Solution{}),
        // 24 =>solution =Box::new( y2015d24::Solution{}),
        // 25 =>solution =Box::new( y2015d25::Solution{}),
        _ => panic!("Illegal day {} for 2015", day),
    }
    match part {
        '1' => solution.run_part1(input, dryrun),
        '2' => solution.run_part2(input, dryrun),
        'a' => solution.run(input, dryrun),
        _ => panic!("Illegal part option, use 1,2 or a"),
    }
}
