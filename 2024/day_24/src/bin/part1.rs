use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
enum Op {
    XOR,
    OR,
    AND,
}

#[derive(Debug, Clone, Copy)]
struct Operation<'a> {
    op1: &'a str,
    op2: &'a str,
    op: Op,
    out: &'a str,
}

fn parse(input: &str) -> (HashMap<&str, bool>, Vec<Operation>) {
    let (a, b) = input.split_once("\n\n").unwrap();

    let init_list: HashMap<&str, bool> = a
        .lines()
        .map(|entry| {
            let mut parts = entry.split(':');
            let id = parts.next().unwrap();
            // println!("{:?}", id);
            let value = parts.next().unwrap().trim().parse::<u8>().unwrap() != 0;
            // println!("{:?}", value);
            (id, value)
        })
        .collect();

    let ops: Vec<Operation> = b
        .lines()
        .map(|entry| {
            let mut part = entry.split(' ');

            let sym1 = part.next().unwrap();
            let op = part.next().unwrap();
            let sym2 = part.next().unwrap();
            let _ = part.next();
            let out = part.next().unwrap();
            let operation = match op {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => panic!("Illegal operation: {:?}", op),
            };
            // println!("s1: {:?} s2: {:?} op: {:?}, out: {:?}", sym1, sym2, op, out);

            Operation {
                op1: sym1,
                op2: sym2,
                op: operation,
                out,
            }
        })
        .collect();

    // println!("{:?}", init_list);
    // println!("{:?}", ops);
    (init_list, ops)
}

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_symbol_value<'a>(sym: &'a str, initial: &'a HashMap<&'a str, bool>) -> Option<&'a bool> {
    initial.get(sym)
}

fn calculate(
    operation: &Operation,
    values: &HashMap<&str, bool>,
    operations: &Vec<Operation>,
) -> bool {
    let mut val1 = false;
    if values.contains_key(operation.op1) {
        val1 = *values.get(operation.op1).unwrap();
    } else {
        let temp_op = operations
            .iter()
            .filter(|op| op.out == operation.op1)
            .next()
            .expect("Looking for {sym1} in operations, but it's not found.");
        val1 = calculate(temp_op, values, operations);
    }

    let mut val2 = false;
    if values.contains_key(operation.op2) {
        val2 = *values.get(operation.op2).unwrap();
    } else {
        let temp_op = operations
            .iter()
            .filter(|op| op.out == operation.op2)
            .next()
            .expect("Looking for {sym2} in operations, but it's not found.");
        val2 = calculate(temp_op, values, operations);
    }

    match operation.op {
        Op::AND => val1 & val2,
        Op::OR => val1 | val2,
        Op::XOR => val1 ^ val2,
    }
}

fn part1(input: &str) -> u64 {
    let (mut initial, operations) = parse(input);

    for op in operations.clone() {
        let s = calculate(&op, &initial, &operations);
        initial.insert(op.out, s);
    }

    println!("{:?}", initial);

    let mut found = true;
    let mut ret: u64 = 0;
    let mut index = 0;

    while found {
        let key = format!("z{index:02}");
        println!("Getting: {key}");
        if initial.contains_key(key.as_str()) {
            ret |= (*initial.get(key.as_str()).unwrap() as u64) << index;
        } else {
            found = false;
        }
        index += 1;
    }

    println!("Found the values: {:016b}", ret);
    ret
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

        let temp = parse(&dummy);

        assert_eq!(part1(dummy), 2024);
    }
}
