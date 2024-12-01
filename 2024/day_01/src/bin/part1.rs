use sorted_vec::SortedVec;

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let (a, b) = parse_data(input);

    let answer = part1(&a, &b);
    dbg!(a);
    dbg!(b);
    dbg!(answer);
}

fn parse_data(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut split = line.split_whitespace();
        a.push(
            split
                .next()
                .expect("Could not parse as number")
                .parse()
                .expect("not a number"),
        );
        b.push(
            split
                .next()
                .expect("Could not parse as number")
                .parse()
                .expect("not a number"),
        );
    }

    (a, b)
}

fn part1(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut diff: Vec<i32> = Vec::new();

    let x: SortedVec<i32> = SortedVec::from_unsorted(a.to_vec());
    let y: SortedVec<i32> = SortedVec::from_unsorted(b.to_vec());

    for i in 0..a.len() {
        diff.push(i32::abs(x[i] - y[i]));
    }

    diff.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {
        let dummy = "3   4
4   3
2   5
1   3
3   9
3   3";

        let (a, b) = parse_data(dummy);
        assert_eq!(a, [3, 4, 2, 1, 3, 3]);
        assert_eq!(b, [4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_compare_sorted_numbers() {
        let dummy = "3   4
4   3
2   5
1   3
3   9
3   3";

        let (a, b) = parse_data(dummy);

        let total_dist = part1(&a, &b);
        assert_eq!(total_dist, 11);
    }
}
