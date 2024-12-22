fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|n| n.parse::<u32>().unwrap()).collect()
}

fn pseudorandom(seed: u32) -> u32 {
    // println!("Start seed: {seed} {seed:08x}");
    let step1 = seed << 6;
    // println!("step1: {step1} {step1:08x}");

    let mut result = (seed ^ step1) & 0xffffff;
    // println!("result1: {result} {result:08x}");

    let step2 = result >> 5;
    // println!("step2: {step2} {step2:08x}");
    result = (result ^ step2) & 0xffffff;
    // println!("result2: {result} {result:08x}");

    let step3 = result << 11;
    // println!("step3: {step3} {step3:16x}");
    result = (result ^ step3) & 0xffffff;
    // println!("result3: {result} {result:08x}");

    result
}

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u64 {
    let data = parse(input);

    data.iter()
        .map(|n| {
            let mut num = *n;
            for _i in 0..2000 {
                num = pseudorandom(num)
            }
            num as u64
        })
        .sum::<u64>()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_one_num() {
        let dummy = "123";
        let mut result = vec![];

        let numbers = parse(dummy);
        result.push(numbers[0]);

        for i in 0..10 {
            result.push(pseudorandom(result[i]));
            println!();
        }

        assert_eq!(
            result,
            [
                123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
                7753432, 5908254
            ]
        );
    }

    #[test]
    fn test_num_1() {
        let mut result = 1;

        for _i in 0..2000 {
            result = pseudorandom(result);
        }

        assert_eq!(result, 8685429);
    }

    #[test]
    fn test_num_10() {
        let mut result = 10;

        for _i in 0..2000 {
            result = pseudorandom(result);
        }

        assert_eq!(result, 4700978);
    }

    #[test]
    fn test_4_nums() {
        let dummy = "1
10
100
2024";

        assert_eq!(part1(dummy), 37327623);
    }
}
