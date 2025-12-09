use std::collections::HashMap;

// My Library modules
pub mod grid;
pub mod parsing;
pub mod solutions;

pub trait AoCSolution {
    fn year(&self) -> u16;
    fn day(&self) -> u8;
    fn part1(&mut self, input: &str) -> String;
    fn part2(&mut self, input: &str) -> String;

    fn run(&mut self, input_file: &str) {
        let input = crate::parsing::read_input(input_file);
        println!("-- Advent of Code {} day {} --", self.year(), self.day());

        let start = std::time::Instant::now();
        let answer = self.part1(&input);
        let end = std::time::Instant::now();
        println!(
            "Part 1: {answer} - in {:.3} ms",
            (end - start).as_secs_f64() * 1000.0
        );

        let start = std::time::Instant::now();
        let answer = self.part2(&input);
        let end = std::time::Instant::now();
        println!(
            "Part 2: {answer} - in {:.3} ms",
            (end - start).as_secs_f64() * 1000.0
        );
    }
    fn run_part1(&mut self, input_file: &str) {
        let input = crate::parsing::read_input(input_file);
        println!(
            "-- Advent of Code {} day {} Part1 --",
            self.year(),
            self.day()
        );

        let start = std::time::Instant::now();
        let answer = self.part1(&input);
        let end = std::time::Instant::now();
        println!(
            "Part 1: {answer} - in {:.3} ms",
            (end - start).as_secs_f64() * 1000.0
        );
    }
    fn run_part2(&mut self, input_file: &str) {
        let input = crate::parsing::read_input(input_file);
        println!(
            "-- Advent of Code {} day {} Part2 --",
            self.year(),
            self.day()
        );

        let start = std::time::Instant::now();
        let answer = self.part2(&input);
        let end = std::time::Instant::now();
        println!(
            "Part 2: {answer} - in {:.3} ms",
            (end - start).as_secs_f64() * 1000.0
        );
    }
}

type SolutionConstructor = fn() -> Box<dyn AoCSolution>;

pub fn get_solutions() -> HashMap<(u16, u8), SolutionConstructor> {
    let mut map: HashMap<(u16, u8), SolutionConstructor> = HashMap::new();

    // 2015 ----------------------------------
    map.insert((2015, 1), || {
        Box::new(solutions::y2015::y2015d01::Solution {})
    });
    map.insert((2015, 2), || {
        Box::new(solutions::y2015::y2015d02::Solution {})
    });
    map.insert((2015, 3), || {
        Box::new(solutions::y2015::y2015d03::Solution {})
    });
    map.insert((2015, 4), || {
        Box::new(solutions::y2015::y2015d04::Solution {})
    });
    map.insert((2015, 5), || {
        Box::new(solutions::y2015::y2015d05::Solution::new())
    });
    // map.insert((2015, 6), || {
    //     Box::new(solutions::y2015::y2015d06::Solution {})
    // });
    // map.insert((2015, 7), || {
    //     Box::new(solutions::y2015::y2015d07::Solution {})
    // });
    // map.insert((2015, 8), || {
    //     Box::new(solutions::y2015::y2015d08::Solution {})
    // });
    // map.insert((2015, 9), || {
    //     Box::new(solutions::y2015::y2015d09::Solution {})
    // });
    // map.insert((2015, 10), || {
    //     Box::new(solutions::y2015::y2015d10::Solution {})
    // });
    // map.insert((2015, 11), || {
    //     Box::new(solutions::y2015::y2015d11::Solution {})
    // });
    // map.insert((2015, 12), || {
    //     Box::new(solutions::y2015::y2015d12::Solution {})
    // });
    // map.insert((2015, 13), || {
    //     Box::new(solutions::y2015::y2015d13::Solution {})
    // });
    // map.insert((2015, 14), || {
    //     Box::new(solutions::y2015::y2015d14::Solution {})
    // });
    // map.insert((2015, 15), || {
    //     Box::new(solutions::y2015::y2015d15::Solution {})
    // });
    // map.insert((2015, 16), || {
    //     Box::new(solutions::y2015::y2015d16::Solution {})
    // });
    // map.insert((2015, 17), || {
    //     Box::new(solutions::y2015::y2015d17::Solution {})
    // });
    // map.insert((2015, 18), || {
    //     Box::new(solutions::y2015::y2015d18::Solution {})
    // });
    // map.insert((2015, 19), || {
    //     Box::new(solutions::y2015::y2015d19::Solution {})
    // });
    // map.insert((2015, 20), || {
    //     Box::new(solutions::y2015::y2015d20::Solution {})
    // });
    // map.insert((2015, 21), || {
    //     Box::new(solutions::y2015::y2015d21::Solution {})
    // });
    // map.insert((2015, 22), || {
    //     Box::new(solutions::y2015::y2015d22::Solution {})
    // });
    // map.insert((2015, 23), || {
    //     Box::new(solutions::y2015::y2015d23::Solution {})
    // });
    // map.insert((2015, 24), || {
    //     Box::new(solutions::y2015::y2015d24::Solution {})
    // });
    // map.insert((2015, 25), || {
    //     Box::new(solutions::y2015::y2015d25::Solution {})
    // });

    // 2016 ----------------------------------
    // map.insert((2016, 1), || {
    //     Box::new(solutions::y2016::y2016d01::Solution {})
    // });
    // map.insert((2016, 2), || {
    //     Box::new(solutions::y2016::y2016d02::Solution {})
    // });
    // map.insert((2016, 3), || {
    //     Box::new(solutions::y2016::y2016d03::Solution {})
    // });
    // map.insert((2016, 4), || {
    //     Box::new(solutions::y2016::y2016d04::Solution {})
    // });
    // map.insert((2016, 5), || {
    //     Box::new(solutions::y2016::y2016d05::Solution {})
    // });
    // map.insert((2016, 6), || {
    //     Box::new(solutions::y2016::y2016d06::Solution {})
    // });
    // map.insert((2016, 7), || {
    //     Box::new(solutions::y2016::y2016d07::Solution {})
    // });
    // map.insert((2016, 8), || {
    //     Box::new(solutions::y2016::y2016d08::Solution {})
    // });
    // map.insert((2016, 9), || {
    //     Box::new(solutions::y2016::y2016d09::Solution {})
    // });
    // map.insert((2016, 10), || {
    //     Box::new(solutions::y2016::y2016d10::Solution {})
    // });
    // map.insert((2016, 11), || {
    //     Box::new(solutions::y2016::y2016d11::Solution {})
    // });
    // map.insert((2016, 12), || {
    //     Box::new(solutions::y2016::y2016d12::Solution {})
    // });
    // map.insert((2016, 13), || {
    //     Box::new(solutions::y2016::y2016d13::Solution {})
    // });
    // map.insert((2016, 14), || {
    //     Box::new(solutions::y2016::y2016d14::Solution {})
    // });
    // map.insert((2016, 15), || {
    //     Box::new(solutions::y2016::y2016d15::Solution {})
    // });
    // map.insert((2016, 16), || {
    //     Box::new(solutions::y2016::y2016d16::Solution {})
    // });
    // map.insert((2016, 17), || {
    //     Box::new(solutions::y2016::y2016d17::Solution {})
    // });
    // map.insert((2016, 18), || {
    //     Box::new(solutions::y2016::y2016d18::Solution {})
    // });
    // map.insert((2016, 19), || {
    //     Box::new(solutions::y2016::y2016d19::Solution {})
    // });
    // map.insert((2016, 20), || {
    //     Box::new(solutions::y2016::y2016d20::Solution {})
    // });
    // map.insert((2016, 21), || {
    //     Box::new(solutions::y2016::y2016d21::Solution {})
    // });
    // map.insert((2016, 22), || {
    //     Box::new(solutions::y2016::y2016d22::Solution {})
    // });
    // map.insert((2016, 23), || {
    //     Box::new(solutions::y2016::y2016d23::Solution {})
    // });
    // map.insert((2016, 24), || {
    //     Box::new(solutions::y2016::y2016d24::Solution {})
    // });
    // map.insert((2016, 25), || {
    //     Box::new(solutions::y2016::y2016d25::Solution {})
    // });

    // 2017 ----------------------------------
    // map.insert((2017, 1), || {
    //     Box::new(solutions::y2017::y2017d01::Solution {})
    // });
    // map.insert((2017, 2), || {
    //     Box::new(solutions::y2017::y2017d02::Solution {})
    // });
    // map.insert((2017, 3), || {
    //     Box::new(solutions::y2017::y2017d03::Solution {})
    // });
    // map.insert((2017, 4), || {
    //     Box::new(solutions::y2017::y2017d04::Solution {})
    // });
    // map.insert((2017, 5), || {
    //     Box::new(solutions::y2017::y2017d05::Solution {})
    // });
    // map.insert((2017, 6), || {
    //     Box::new(solutions::y2017::y2017d06::Solution {})
    // });
    // map.insert((2017, 7), || {
    //     Box::new(solutions::y2017::y2017d07::Solution {})
    // });
    // map.insert((2017, 8), || {
    //     Box::new(solutions::y2017::y2017d08::Solution {})
    // });
    // map.insert((2017, 9), || {
    //     Box::new(solutions::y2017::y2017d09::Solution {})
    // });
    // map.insert((2017, 10), || {
    //     Box::new(solutions::y2017::y2017d10::Solution {})
    // });
    // map.insert((2017, 11), || {
    //     Box::new(solutions::y2017::y2017d11::Solution {})
    // });
    // map.insert((2017, 12), || {
    //     Box::new(solutions::y2017::y2017d12::Solution {})
    // });
    // map.insert((2017, 13), || {
    //     Box::new(solutions::y2017::y2017d13::Solution {})
    // });
    // map.insert((2017, 14), || {
    //     Box::new(solutions::y2017::y2017d14::Solution {})
    // });
    // map.insert((2017, 15), || {
    //     Box::new(solutions::y2017::y2017d15::Solution {})
    // });
    // map.insert((2017, 16), || {
    //     Box::new(solutions::y2017::y2017d16::Solution {})
    // });
    // map.insert((2017, 17), || {
    //     Box::new(solutions::y2017::y2017d17::Solution {})
    // });
    // map.insert((2017, 18), || {
    //     Box::new(solutions::y2017::y2017d18::Solution {})
    // });
    // map.insert((2017, 19), || {
    //     Box::new(solutions::y2017::y2017d19::Solution {})
    // });
    // map.insert((2017, 20), || {
    //     Box::new(solutions::y2017::y2017d20::Solution {})
    // });
    // map.insert((2017, 21), || {
    //     Box::new(solutions::y2017::y2017d21::Solution {})
    // });
    // map.insert((2017, 22), || {
    //     Box::new(solutions::y2017::y2017d22::Solution {})
    // });
    // map.insert((2017, 23), || {
    //     Box::new(solutions::y2017::y2017d23::Solution {})
    // });
    // map.insert((2017, 24), || {
    //     Box::new(solutions::y2017::y2017d24::Solution {})
    // });
    // map.insert((2017, 25), || {
    //     Box::new(solutions::y2017::y2017d25::Solution {})
    // });

    // 2018 ----------------------------------
    // map.insert((2018, 1), || {
    //     Box::new(solutions::y2018::y2018d01::Solution {})
    // });
    // map.insert((2018, 2), || {
    //     Box::new(solutions::y2018::y2018d02::Solution {})
    // });
    // map.insert((2018, 3), || {
    //     Box::new(solutions::y2018::y2018d03::Solution {})
    // });
    // map.insert((2018, 4), || {
    //     Box::new(solutions::y2018::y2018d04::Solution {})
    // });
    // map.insert((2018, 5), || {
    //     Box::new(solutions::y2018::y2018d05::Solution {})
    // });
    // map.insert((2018, 6), || {
    //     Box::new(solutions::y2018::y2018d06::Solution {})
    // });
    // map.insert((2018, 7), || {
    //     Box::new(solutions::y2018::y2018d07::Solution {})
    // });
    // map.insert((2018, 8), || {
    //     Box::new(solutions::y2018::y2018d08::Solution {})
    // });
    // map.insert((2018, 9), || {
    //     Box::new(solutions::y2018::y2018d09::Solution {})
    // });
    // map.insert((2018, 10), || {
    //     Box::new(solutions::y2018::y2018d10::Solution {})
    // });
    // map.insert((2018, 11), || {
    //     Box::new(solutions::y2018::y2018d11::Solution {})
    // });
    // map.insert((2018, 12), || {
    //     Box::new(solutions::y2018::y2018d12::Solution {})
    // });
    // map.insert((2018, 13), || {
    //     Box::new(solutions::y2018::y2018d13::Solution {})
    // });
    // map.insert((2018, 14), || {
    //     Box::new(solutions::y2018::y2018d14::Solution {})
    // });
    // map.insert((2018, 15), || {
    //     Box::new(solutions::y2018::y2018d15::Solution {})
    // });
    // map.insert((2018, 16), || {
    //     Box::new(solutions::y2018::y2018d16::Solution {})
    // });
    // map.insert((2018, 17), || {
    //     Box::new(solutions::y2018::y2018d17::Solution {})
    // });
    // map.insert((2018, 18), || {
    //     Box::new(solutions::y2018::y2018d18::Solution {})
    // });
    // map.insert((2018, 19), || {
    //     Box::new(solutions::y2018::y2018d19::Solution {})
    // });
    // map.insert((2018, 20), || {
    //     Box::new(solutions::y2018::y2018d20::Solution {})
    // });
    // map.insert((2018, 21), || {
    //     Box::new(solutions::y2018::y2018d21::Solution {})
    // });
    // map.insert((2018, 22), || {
    //     Box::new(solutions::y2018::y2018d22::Solution {})
    // });
    // map.insert((2018, 23), || {
    //     Box::new(solutions::y2018::y2018d23::Solution {})
    // });
    // map.insert((2018, 24), || {
    //     Box::new(solutions::y2018::y2018d24::Solution {})
    // });
    // map.insert((2018, 25), || {
    //     Box::new(solutions::y2018::y2018d25::Solution {})
    // });

    // 2019 ----------------------------------
    // map.insert((2019, 1), || {
    //     Box::new(solutions::y2019::y2019d01::Solution {})
    // });
    // map.insert((2019, 2), || {
    //     Box::new(solutions::y2019::y2019d02::Solution {})
    // });
    // map.insert((2019, 3), || {
    //     Box::new(solutions::y2019::y2019d03::Solution {})
    // });
    // map.insert((2019, 4), || {
    //     Box::new(solutions::y2019::y2019d04::Solution {})
    // });
    // map.insert((2019, 5), || {
    //     Box::new(solutions::y2019::y2019d05::Solution {})
    // });
    // map.insert((2019, 6), || {
    //     Box::new(solutions::y2019::y2019d06::Solution {})
    // });
    // map.insert((2019, 7), || {
    //     Box::new(solutions::y2019::y2019d07::Solution {})
    // });
    // map.insert((2019, 8), || {
    //     Box::new(solutions::y2019::y2019d08::Solution {})
    // });
    // map.insert((2019, 9), || {
    //     Box::new(solutions::y2019::y2019d09::Solution {})
    // });
    // map.insert((2019, 10), || {
    //     Box::new(solutions::y2019::y2019d10::Solution {})
    // });
    // map.insert((2019, 11), || {
    //     Box::new(solutions::y2019::y2019d11::Solution {})
    // });
    // map.insert((2019, 12), || {
    //     Box::new(solutions::y2019::y2019d12::Solution {})
    // });
    // map.insert((2019, 13), || {
    //     Box::new(solutions::y2019::y2019d13::Solution {})
    // });
    // map.insert((2019, 14), || {
    //     Box::new(solutions::y2019::y2019d14::Solution {})
    // });
    // map.insert((2019, 15), || {
    //     Box::new(solutions::y2019::y2019d15::Solution {})
    // });
    // map.insert((2019, 16), || {
    //     Box::new(solutions::y2019::y2019d16::Solution {})
    // });
    // map.insert((2019, 17), || {
    //     Box::new(solutions::y2019::y2019d17::Solution {})
    // });
    // map.insert((2019, 18), || {
    //     Box::new(solutions::y2019::y2019d18::Solution {})
    // });
    // map.insert((2019, 19), || {
    //     Box::new(solutions::y2019::y2019d19::Solution {})
    // });
    // map.insert((2019, 20), || {
    //     Box::new(solutions::y2019::y2019d20::Solution {})
    // });
    // map.insert((2019, 21), || {
    //     Box::new(solutions::y2019::y2019d21::Solution {})
    // });
    // map.insert((2019, 22), || {
    //     Box::new(solutions::y2019::y2019d22::Solution {})
    // });
    // map.insert((2019, 23), || {
    //     Box::new(solutions::y2019::y2019d23::Solution {})
    // });
    // map.insert((2019, 24), || {
    //     Box::new(solutions::y2019::y2019d24::Solution {})
    // });
    // map.insert((2019, 25), || {
    //     Box::new(solutions::y2019::y2019d25::Solution {})
    // });

    // 2020 ----------------------------------
    // map.insert((2020, 1), || {
    //     Box::new(solutions::y2020::y2020d01::Solution {})
    // });
    // map.insert((2020, 2), || {
    //     Box::new(solutions::y2020::y2020d02::Solution {})
    // });
    // map.insert((2020, 3), || {
    //     Box::new(solutions::y2020::y2020d03::Solution {})
    // });
    // map.insert((2020, 4), || {
    //     Box::new(solutions::y2020::y2020d04::Solution {})
    // });
    // map.insert((2020, 5), || {
    //     Box::new(solutions::y2020::y2020d05::Solution {})
    // });
    // map.insert((2020, 6), || {
    //     Box::new(solutions::y2020::y2020d06::Solution {})
    // });
    // map.insert((2020, 7), || {
    //     Box::new(solutions::y2020::y2020d07::Solution {})
    // });
    // map.insert((2020, 8), || {
    //     Box::new(solutions::y2020::y2020d08::Solution {})
    // });
    // map.insert((2020, 9), || {
    //     Box::new(solutions::y2020::y2020d09::Solution {})
    // });
    // map.insert((2020, 10), || {
    //     Box::new(solutions::y2020::y2020d10::Solution {})
    // });
    // map.insert((2020, 11), || {
    //     Box::new(solutions::y2020::y2020d11::Solution {})
    // });
    // map.insert((2020, 12), || {
    //     Box::new(solutions::y2020::y2020d12::Solution {})
    // });
    // map.insert((2020, 13), || {
    //     Box::new(solutions::y2020::y2020d13::Solution {})
    // });
    // map.insert((2020, 14), || {
    //     Box::new(solutions::y2020::y2020d14::Solution {})
    // });
    // map.insert((2020, 15), || {
    //     Box::new(solutions::y2020::y2020d15::Solution {})
    // });
    // map.insert((2020, 16), || {
    //     Box::new(solutions::y2020::y2020d16::Solution {})
    // });
    // map.insert((2020, 17), || {
    //     Box::new(solutions::y2020::y2020d17::Solution {})
    // });
    // map.insert((2020, 18), || {
    //     Box::new(solutions::y2020::y2020d18::Solution {})
    // });
    // map.insert((2020, 19), || {
    //     Box::new(solutions::y2020::y2020d19::Solution {})
    // });
    // map.insert((2020, 20), || {
    //     Box::new(solutions::y2020::y2020d20::Solution {})
    // });
    // map.insert((2020, 21), || {
    //     Box::new(solutions::y2020::y2020d21::Solution {})
    // });
    // map.insert((2020, 22), || {
    //     Box::new(solutions::y2020::y2020d22::Solution {})
    // });
    // map.insert((2020, 23), || {
    //     Box::new(solutions::y2020::y2020d23::Solution {})
    // });
    // map.insert((2020, 24), || {
    //     Box::new(solutions::y2020::y2020d24::Solution {})
    // });
    // map.insert((2020, 25), || {
    //     Box::new(solutions::y2020::y2020d25::Solution {})
    // });

    // 2021 ----------------------------------
    // map.insert((2021, 1), || {
    //     Box::new(solutions::y2021::y2021d01::Solution {})
    // });
    // map.insert((2021, 2), || {
    //     Box::new(solutions::y2021::y2021d02::Solution {})
    // });
    // map.insert((2021, 3), || {
    //     Box::new(solutions::y2021::y2021d03::Solution {})
    // });
    // map.insert((2021, 4), || {
    //     Box::new(solutions::y2021::y2021d04::Solution {})
    // });
    // map.insert((2021, 5), || {
    //     Box::new(solutions::y2021::y2021d05::Solution {})
    // });
    // map.insert((2021, 6), || {
    //     Box::new(solutions::y2021::y2021d06::Solution {})
    // });
    // map.insert((2021, 7), || {
    //     Box::new(solutions::y2021::y2021d07::Solution {})
    // });
    // map.insert((2021, 8), || {
    //     Box::new(solutions::y2021::y2021d08::Solution {})
    // });
    // map.insert((2021, 9), || {
    //     Box::new(solutions::y2021::y2021d09::Solution {})
    // });
    // map.insert((2021, 10), || {
    //     Box::new(solutions::y2021::y2021d10::Solution {})
    // });
    // map.insert((2021, 11), || {
    //     Box::new(solutions::y2021::y2021d11::Solution {})
    // });
    // map.insert((2021, 12), || {
    //     Box::new(solutions::y2021::y2021d12::Solution {})
    // });
    // map.insert((2021, 13), || {
    //     Box::new(solutions::y2021::y2021d13::Solution {})
    // });
    // map.insert((2021, 14), || {
    //     Box::new(solutions::y2021::y2021d14::Solution {})
    // });
    // map.insert((2021, 15), || {
    //     Box::new(solutions::y2021::y2021d15::Solution {})
    // });
    // map.insert((2021, 16), || {
    //     Box::new(solutions::y2021::y2021d16::Solution {})
    // });
    // map.insert((2021, 17), || {
    //     Box::new(solutions::y2021::y2021d17::Solution {})
    // });
    // map.insert((2021, 18), || {
    //     Box::new(solutions::y2021::y2021d18::Solution {})
    // });
    // map.insert((2021, 19), || {
    //     Box::new(solutions::y2021::y2021d19::Solution {})
    // });
    // map.insert((2021, 20), || {
    //     Box::new(solutions::y2021::y2021d20::Solution {})
    // });
    // map.insert((2021, 21), || {
    //     Box::new(solutions::y2021::y2021d21::Solution {})
    // });
    // map.insert((2021, 22), || {
    //     Box::new(solutions::y2021::y2021d22::Solution {})
    // });
    // map.insert((2021, 23), || {
    //     Box::new(solutions::y2021::y2021d23::Solution {})
    // });
    // map.insert((2021, 24), || {
    //     Box::new(solutions::y2021::y2021d24::Solution {})
    // });
    // map.insert((2021, 25), || {
    //     Box::new(solutions::y2021::y2021d25::Solution {})
    // });

    // 2022 ----------------------------------
    // map.insert((2022, 1), || {
    //     Box::new(solutions::y2022::y2022d01::Solution {})
    // });
    // map.insert((2022, 2), || {
    //     Box::new(solutions::y2022::y2022d02::Solution {})
    // });
    // map.insert((2022, 3), || {
    //     Box::new(solutions::y2022::y2022d03::Solution {})
    // });
    // map.insert((2022, 4), || {
    //     Box::new(solutions::y2022::y2022d04::Solution {})
    // });
    // map.insert((2022, 5), || {
    //     Box::new(solutions::y2022::y2022d05::Solution {})
    // });
    // map.insert((2022, 6), || {
    //     Box::new(solutions::y2022::y2022d06::Solution {})
    // });
    // map.insert((2022, 7), || {
    //     Box::new(solutions::y2022::y2022d07::Solution {})
    // });
    // map.insert((2022, 8), || {
    //     Box::new(solutions::y2022::y2022d08::Solution {})
    // });
    // map.insert((2022, 9), || {
    //     Box::new(solutions::y2022::y2022d09::Solution {})
    // });
    // map.insert((2022, 10), || {
    //     Box::new(solutions::y2022::y2022d10::Solution {})
    // });
    // map.insert((2022, 11), || {
    //     Box::new(solutions::y2022::y2022d11::Solution {})
    // });
    // map.insert((2022, 12), || {
    //     Box::new(solutions::y2022::y2022d12::Solution {})
    // });
    // map.insert((2022, 13), || {
    //     Box::new(solutions::y2022::y2022d13::Solution {})
    // });
    // map.insert((2022, 14), || {
    //     Box::new(solutions::y2022::y2022d14::Solution {})
    // });
    // map.insert((2022, 15), || {
    //     Box::new(solutions::y2022::y2022d15::Solution {})
    // });
    // map.insert((2022, 16), || {
    //     Box::new(solutions::y2022::y2022d16::Solution {})
    // });
    // map.insert((2022, 17), || {
    //     Box::new(solutions::y2022::y2022d17::Solution {})
    // });
    // map.insert((2022, 18), || {
    //     Box::new(solutions::y2022::y2022d18::Solution {})
    // });
    // map.insert((2022, 19), || {
    //     Box::new(solutions::y2022::y2022d19::Solution {})
    // });
    // map.insert((2022, 20), || {
    //     Box::new(solutions::y2022::y2022d20::Solution {})
    // });
    // map.insert((2022, 21), || {
    //     Box::new(solutions::y2022::y2022d21::Solution {})
    // });
    // map.insert((2022, 22), || {
    //     Box::new(solutions::y2022::y2022d22::Solution {})
    // });
    // map.insert((2022, 23), || {
    //     Box::new(solutions::y2022::y2022d23::Solution {})
    // });
    // map.insert((2022, 24), || {
    //     Box::new(solutions::y2022::y2022d24::Solution {})
    // });
    // map.insert((2022, 25), || {
    //     Box::new(solutions::y2022::y2022d25::Solution {})
    // });

    // 2023 ----------------------------------
    // map.insert((2023, 1), || {
    //     Box::new(solutions::y2023::y2023d01::Solution {})
    // });
    // map.insert((2023, 2), || {
    //     Box::new(solutions::y2023::y2023d02::Solution {})
    // });
    // map.insert((2023, 3), || {
    //     Box::new(solutions::y2023::y2023d03::Solution {})
    // });
    // map.insert((2023, 4), || {
    //     Box::new(solutions::y2023::y2023d04::Solution {})
    // });
    // map.insert((2023, 5), || {
    //     Box::new(solutions::y2023::y2023d05::Solution {})
    // });
    // map.insert((2023, 6), || {
    //     Box::new(solutions::y2023::y2023d06::Solution {})
    // });
    // map.insert((2023, 7), || {
    //     Box::new(solutions::y2023::y2023d07::Solution {})
    // });
    // map.insert((2023, 8), || {
    //     Box::new(solutions::y2023::y2023d08::Solution {})
    // });
    // map.insert((2023, 9), || {
    //     Box::new(solutions::y2023::y2023d09::Solution {})
    // });
    // map.insert((2023, 10), || {
    //     Box::new(solutions::y2023::y2023d10::Solution {})
    // });
    // map.insert((2023, 11), || {
    //     Box::new(solutions::y2023::y2023d11::Solution {})
    // });
    // map.insert((2023, 12), || {
    //     Box::new(solutions::y2023::y2023d12::Solution {})
    // });
    // map.insert((2023, 13), || {
    //     Box::new(solutions::y2023::y2023d13::Solution {})
    // });
    // map.insert((2023, 14), || {
    //     Box::new(solutions::y2023::y2023d14::Solution {})
    // });
    // map.insert((2023, 15), || {
    //     Box::new(solutions::y2023::y2023d15::Solution {})
    // });
    // map.insert((2023, 16), || {
    //     Box::new(solutions::y2023::y2023d16::Solution {})
    // });
    // map.insert((2023, 17), || {
    //     Box::new(solutions::y2023::y2023d17::Solution {})
    // });
    // map.insert((2023, 18), || {
    //     Box::new(solutions::y2023::y2023d18::Solution {})
    // });
    // map.insert((2023, 19), || {
    //     Box::new(solutions::y2023::y2023d19::Solution {})
    // });
    // map.insert((2023, 20), || {
    //     Box::new(solutions::y2023::y2023d20::Solution {})
    // });
    // map.insert((2023, 21), || {
    //     Box::new(solutions::y2023::y2023d21::Solution {})
    // });
    // map.insert((2023, 22), || {
    //     Box::new(solutions::y2023::y2023d22::Solution {})
    // });
    // map.insert((2023, 23), || {
    //     Box::new(solutions::y2023::y2023d23::Solution {})
    // });
    // map.insert((2023, 24), || {
    //     Box::new(solutions::y2023::y2023d24::Solution {})
    // });
    // map.insert((2023, 25), || {
    //     Box::new(solutions::y2023::y2023d25::Solution {})
    // });

    // 2024 ----------------------------------
    map.insert((2024, 1), || {
        Box::new(solutions::y2024::y2024d01::Solution {})
    });
    map.insert((2024, 2), || {
        Box::new(solutions::y2024::y2024d02::Solution {})
    });
    map.insert((2024, 3), || {
        Box::new(solutions::y2024::y2024d03::Solution {})
    });
    map.insert((2024, 4), || {
        Box::new(solutions::y2024::y2024d04::Solution {})
    });
    map.insert((2024, 5), || {
        Box::new(solutions::y2024::y2024d05::Solution {})
    });
    map.insert((2024, 6), || {
        Box::new(solutions::y2024::y2024d06::Solution {})
    });
    map.insert((2024, 7), || {
        Box::new(solutions::y2024::y2024d07::Solution {})
    });
    map.insert((2024, 8), || {
        Box::new(solutions::y2024::y2024d08::Solution {})
    });
    map.insert((2024, 9), || {
        Box::new(solutions::y2024::y2024d09::Solution {})
    });
    map.insert((2024, 10), || {
        Box::new(solutions::y2024::y2024d10::Solution {})
    });
    // map.insert((2024, 11), || {
    //     Box::new(solutions::y2024::y2024d11::Solution {})
    // });
    // map.insert((2024, 12), || {
    //     Box::new(solutions::y2024::y2024d12::Solution {})
    // });
    // map.insert((2024, 13), || {
    //     Box::new(solutions::y2024::y2024d13::Solution {})
    // });
    // map.insert((2024, 14), || {
    //     Box::new(solutions::y2024::y2024d14::Solution {})
    // });
    // map.insert((2024, 15), || {
    //     Box::new(solutions::y2024::y2024d15::Solution {})
    // });
    // map.insert((2024, 16), || {
    //     Box::new(solutions::y2024::y2024d16::Solution {})
    // });
    // map.insert((2024, 17), || {
    //     Box::new(solutions::y2024::y2024d17::Solution {})
    // });
    // map.insert((2024, 18), || {
    //     Box::new(solutions::y2024::y2024d18::Solution {})
    // });
    // map.insert((2024, 19), || {
    //     Box::new(solutions::y2024::y2024d19::Solution {})
    // });
    // map.insert((2024, 20), || {
    //     Box::new(solutions::y2024::y2024d20::Solution {})
    // });
    // map.insert((2024, 21), || {
    //     Box::new(solutions::y2024::y2024d21::Solution {})
    // });
    // map.insert((2024, 22), || {
    //     Box::new(solutions::y2024::y2024d22::Solution {})
    // });
    // map.insert((2024, 23), || {
    //     Box::new(solutions::y2024::y2024d23::Solution {})
    // });
    // map.insert((2024, 24), || {
    //     Box::new(solutions::y2024::y2024d24::Solution {})
    // });
    // map.insert((2024, 25), || {
    //     Box::new(solutions::y2024::y2024d25::Solution {})
    // });

    // 2025 ----------------------------------
    map.insert((2025, 1), || {
        Box::new(solutions::y2025::y2025d01::Solution {})
    });
    map.insert((2025, 2), || {
        Box::new(solutions::y2025::y2025d02::Solution {})
    });
    map.insert((2025, 3), || {
        Box::new(solutions::y2025::y2025d03::Solution {})
    });
    map.insert((2025, 4), || {
        Box::new(solutions::y2025::y2025d04::Solution {})
    });
    map.insert((2025, 5), || {
        Box::new(solutions::y2025::y2025d05::Solution {})
    });
    map.insert((2025, 6), || {
        Box::new(solutions::y2025::y2025d06::Solution {})
    });
    map.insert((2025, 7), || {
        Box::new(solutions::y2025::y2025d07::Solution {})
    });
    map.insert((2025, 8), || {
        Box::new(solutions::y2025::y2025d08::Solution {
            num_operations_part1: 1000,
            ..Default::default()
        })
    });
    //     map.insert((2025, 9), || {
    //         Box::new(solutions::y2025::y2025d09::Solution {})
    //     });
    //     map.insert((2025, 10), || {
    //         Box::new(solutions::y2025::y2025d10::Solution {})
    //     });
    //     map.insert((2025, 11), || {
    //         Box::new(solutions::y2025::y2025d11::Solution {})
    //     });
    //     map.insert((2025, 12), || {
    //         Box::new(solutions::y2025::y2025d12::Solution {})
    //     });

    map
}
