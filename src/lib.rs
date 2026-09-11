// My Library modules
pub mod grid;
pub mod iterators;
pub mod parsing;
pub mod solutions;

pub trait AoCSolution {
    fn year(&self) -> u16;
    fn day(&self) -> u8;
    fn part1(&mut self, input: &str, dryrun: bool) -> String;
    fn part2(&mut self, input: &str, dryrun: bool) -> String;

    fn run(&mut self, input: &str, dryrun: bool) {
        let mut dry_run = "";
        if dryrun {
            dry_run = " dry-run"
        }
        println!(
            "-- Advent of Code {} day {}{} --",
            self.year(),
            self.day(),
            dry_run
        );

        let start = std::time::Instant::now();
        let answer = self.part1(input, dryrun);
        let end = std::time::Instant::now();
        println!(
            "Part 1:{} {answer} - in {:.3} ms",
            dry_run,
            (end - start).as_secs_f64() * 1000.0
        );

        let start = std::time::Instant::now();
        let answer = self.part2(input, dryrun);
        let end = std::time::Instant::now();
        println!(
            "Part 2:{} {answer} - in {:.3} ms",
            dry_run,
            (end - start).as_secs_f64() * 1000.0
        );
    }
    fn run_part1(&mut self, input: &str, dryrun: bool) {
        let mut dry_run = "";
        if dryrun {
            dry_run = " dry-run"
        }
        println!(
            "-- Advent of Code {} day {} Part1{} --",
            self.year(),
            self.day(),
            dry_run
        );

        let start = std::time::Instant::now();
        let answer = self.part1(input, dryrun);
        let end = std::time::Instant::now();
        println!(
            "Part 1: {answer} - in {:.3} ms",
            (end - start).as_secs_f64() * 1000.0
        );
    }
    fn run_part2(&mut self, input: &str, dryrun: bool) {
        let mut dry_run = "";
        if dryrun {
            dry_run = " dry-run"
        }
        println!(
            "-- Advent of Code {} day {} Part2{} --",
            self.year(),
            self.day(),
            dry_run
        );

        let start = std::time::Instant::now();
        let answer = self.part2(input, dryrun);
        let end = std::time::Instant::now();
        println!(
            "Part 2: {answer} - in {:.3} ms",
            (end - start).as_secs_f64() * 1000.0
        );
    }
}
