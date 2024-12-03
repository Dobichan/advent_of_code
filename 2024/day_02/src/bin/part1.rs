#[derive(Debug)]
pub struct Report {
    pub levels: Vec<i32>,
}

impl Report {
    fn valid(&self) -> bool {
        let ascending: bool;

        if self.levels[0] < self.levels[1] {
            ascending = true;
        } else {
            ascending = false;
        }

        for i in 0..(self.levels.len() - 1) {
            if ascending {
                if self.levels[i] > self.levels[i + 1] {
                    return false;
                }
            } else {
                if self.levels[i] < self.levels[i + 1] {
                    return false;
                }
            }
            let diff = i32::abs(self.levels[i] - self.levels[i + 1]);
            if diff > 3 || diff == 0 {
                return false;
            }
        }
        true
    }
}
fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let reports = parse_input(input);
    let answ = part1(&reports);
    dbg!(answ);
}

fn parse_input(input: &str) -> Vec<Report> {
    let mut reports: Vec<Report> = vec![];
    for line in input.lines() {
        let mut rep: Report = Report { levels: vec![] };

        for level in line.split_whitespace() {
            rep.levels.push(level.parse::<i32>().unwrap());
        }

        reports.push(rep);
    }

    reports
}

fn part1(reports: &Vec<Report>) -> i32 {
    reports.iter().filter(|&report| report.valid()).count() as i32
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let reports = parse_input(dummy);
        let answ = part1(&reports);
        println!("{answ}");

        assert_eq!(answ, 2);
    }
}
