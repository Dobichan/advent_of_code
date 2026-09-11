// My Library modules
pub mod grid;
pub mod iterators;
pub mod parsing;
pub mod runner;
pub mod solutions;

pub trait AoCSolution {
    fn part1(&mut self, input: &str) -> String;
    fn part2(&mut self, input: &str) -> String;
}
