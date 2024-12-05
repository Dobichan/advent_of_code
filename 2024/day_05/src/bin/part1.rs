use nom::{
    bytes::complete::tag, character::complete, multi::separated_list0, sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct PrintJob {
    pages: Vec<i32>,
    // rules: &'a multimap::MultiMap<i32, i32>,
}

impl PrintJob {
    fn is_valid(&self, rules: &multimap::MultiMap<i32, i32>) -> bool {
        for i in 0..self.pages.len() {
            let checks = rules.get_vec(&self.pages[i]);
            if checks.is_some() {
                for check in checks.unwrap() {
                    for k in 0..i {
                        if self.pages[k] == *check {
                            // println!(
                            //     "{i} {k} {:?},{:?} - {:?}",
                            //     self.pages[i], self.pages[k], check
                            // );
                            return false;
                        }
                    }
                }
            }
            // println!("{i}-{:?} - {:?}", self.pages[i], checks);
        }
        // println!("{:?} is ok", self);
        true
    }

    fn get_middle_page(&self) -> i32 {
        self.pages[self.pages.len() / 2]
    }
}

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_job(input: &str) -> IResult<&str, Vec<i32>> {
    let a = separated_list0(tag(","), complete::i32)(input)?;
    Ok(("", a.1))
}
fn page_rule(input: &str) -> IResult<&str, (i32, i32)> {
    let a = separated_pair(complete::i32, tag("|"), complete::i32)(input)?;
    // println!("{:?}", a);
    Ok((input, (a.1 .0, a.1 .1)))
}

fn part1(input: &str) -> i32 {
    let mut rules = true;
    let mut rule_data: multimap::MultiMap<i32, i32> = multimap::MultiMap::new();
    let mut jobs: Vec<PrintJob> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            rules = false;
            continue;
        }

        if rules {
            let pair = page_rule(line).unwrap();
            rule_data.insert(pair.1 .0, pair.1 .1);
        } else {
            let pages = parse_job(line).unwrap();
            let job = PrintJob {
                pages: pages.1,
                // rules: &rule_data,
            };
            jobs.push(job);
        }
    }
    // println!("{:?}", rule_data);
    // println!("{:?}", jobs);
    // println!("");

    jobs.iter()
        .map(|j| {
            if j.is_valid(&rule_data) {
                return j.get_middle_page();
            }
            0
        })
        .sum::<i32>()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(part1(dummy), 143);
    }
}
