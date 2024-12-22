use std::collections::HashMap;

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
    println!("Part2!");

    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn get_num_keymap(num: u32, length: usize) -> HashMap<(i8, i8, i8, i8), u32> {
    let mut numbers: Vec<i8> = Vec::with_capacity(length);

    let mut temp_num = num;
    numbers.push((num % 10) as i8);
    for i in 0..length {
        temp_num = pseudorandom(temp_num);
        numbers.push((temp_num % 10) as i8);
    }

    // println!("{:?}", numbers);

    let mut ret: HashMap<(i8, i8, i8, i8), u32> = HashMap::with_capacity(length);

    for i in 4..length {
        let a = numbers[i - 3] - numbers[i - 4];
        let b = numbers[i - 2] - numbers[i - 3];
        let c = numbers[i - 1] - numbers[i - 2];
        let d = numbers[i] - numbers[i - 1];

        let key = (a, b, c, d);
        ret.entry(key).or_insert(numbers[i] as u32);
    }

    // println!("{:?}", ret);

    ret
}

fn part2(input: &str) -> u64 {
    let nums = parse(input);

    let mut results: HashMap<(i8, i8, i8, i8), u32> = HashMap::new();

    for n in nums {
        let temp_map = get_num_keymap(n, 2000);
        for (k, v) in temp_map {
            results.insert(k, results.get(&k).unwrap_or(&0) + v);
        }
        println!();
    }

    // println!("{:?}", results);
    let max_num = results.iter().max_by_key(|&(_, v)| v);
    println!("Max: {:?}", max_num);

    *max_num.unwrap().1 as u64
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_one_num() {
        let dummy = "123";
        let mut key_numbers = vec![];
        let mut result: Vec<i32> = vec![];

        let numbers = parse(dummy);
        let mut number = numbers[0];
        key_numbers.push(numbers[0] % 10);

        for _i in 0..10 {
            number = pseudorandom(number);
            key_numbers.push(number % 10);
            println!();
        }

        for i in 1..key_numbers.len() - 1 {
            result.push(key_numbers[i] as i32 - key_numbers[i - 1] as i32);
        }

        assert_eq!(result, [-3, 6, -1, -1, 0, 2, -2, 0, -2]);
    }

    #[test]
    fn test_4_nums() {
        let dummy = "1
2
3
2024";

        let nums = parse(&dummy);

        let mut results: HashMap<(i8, i8, i8, i8), u32> = HashMap::new();

        for n in nums {
            let temp_map = get_num_keymap(n, 2000);
            for (k, v) in temp_map {
                results.insert(k, results.get(&k).unwrap_or(&0) + v);
            }
            println!();
        }

        println!("{:?}", results);
        let max_num = results.iter().max_by_key(|&(_, v)| v);
        println!("Max: {:?}", max_num);

        assert_eq!(max_num, Some((&(-2, 1, -1, 3), &23)));
    }
}
