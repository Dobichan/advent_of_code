use std::collections::HashMap;

fn main() {
    println!("Part2!");

    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, targets) = input.split_once("\n\n").unwrap();
    (
        towels.split(",").map(|t| t.trim()).collect(),
        targets.lines().collect(),
    )
}

fn num_create<'a>(
    pattern: &'a str,
    towels: &'a Vec<&str>,
    hash: &mut HashMap<(&'a str, &'a str), u64>,
) -> u64 {
    if pattern.len() == 0 {
        // println!("success");
        return 1;
    }

    let mut nums = 0;
    for towel in towels {
        // println!("Checking if {pattern} starts with {towel} - {nums}");
        if let Some(remaining_pattern) = pattern.strip_prefix(towel) {
            if let Some(possible) = hash.get(&(pattern, towel)) {
                // println!("using hash: {possible}");
                nums += *possible;
            } else {
                let temp = num_create(remaining_pattern, towels, hash);
                hash.insert((pattern, towel), temp);
                // println!("added: {:?}, {temp}", (pattern, towel));
                nums += temp;
            }
        }
    }
    // println!("Returning {nums}");
    nums
}

fn part2(input: &str) -> u64 {
    let (towels, targets) = parse(input);

    let mut ret = 0;
    let mut hash = HashMap::new();

    for target in targets {
        println!();
        // println!("Target: {target}");
        ret += num_create(target, &towels, &mut hash);
        // println!("{ret}")
    }
    ret
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let dummy = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        assert_eq!(part2(dummy), 16);
    }
}
