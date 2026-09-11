// My Library modules
pub mod grid;
pub mod iterators;
pub mod parsing;
pub mod runner;
pub mod solutions;

pub trait AoCSolution {
    fn year(&self) -> u16;
    fn day(&self) -> u8;
    fn part1(&mut self, input: &str, dryrun: bool) -> String;
    fn part2(&mut self, input: &str, dryrun: bool) -> String;
}
