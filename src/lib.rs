use std::fmt::Display;

// My Library modules
pub mod direction;
pub mod grid;
pub mod iterators;
pub mod parsing;
pub mod point;
pub mod runner;
pub mod solutions;

pub trait AoCSolution {
    /// The data for each solution after the input text has been parsed (grid, lines, struct of lookups...)
    type Parsed;

    fn parse(&self, input: &str) -> Self::Parsed;
    fn part1(&self, data: &Self::Parsed) -> impl Display;
    fn part2(&self, data: &Self::Parsed) -> impl Display;

    fn solve1(&self, input: &str) -> String {
        self.part1(&self.parse(input)).to_string()
    }

    fn solve2(&self, input: &str) -> String {
        self.part2(&self.parse(input)).to_string()
    }
}
