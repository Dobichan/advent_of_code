use std::fmt::Display;

use itertools::Itertools;

use crate::{AoCSolution, parsing::input_lines};

#[derive(Default)]
pub struct Solution {}

#[derive(Clone, Debug)]
pub struct VirtualMachine {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    pc: usize,
    program: Vec<u8>,
    output_buffer: Vec<u8>,
}

impl VirtualMachine {
    fn new(a: usize, b: usize, c: usize, prog: &[u8]) -> Self {
        Self {
            reg_a: a,
            reg_b: b,
            reg_c: c,
            pc: 0,
            program: prog.to_vec(),
            output_buffer: vec![],
        }
    }

    fn reset(&mut self, new_a: usize) {
        self.reg_a = new_a;
        self.reg_b = 0;
        self.reg_c = 0;
        self.pc = 0;
        self.output_buffer.clear();
    }

    fn combo_operand(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("Reserved combo operand! Shall not happen"),
            _ => panic!("Illegal operand larger than 3 bits"),
        }
    }

    fn adv(&self, operand: u8) -> usize {
        // let denom = usize::pow(2, self.combo_operand(operand) as u32);
        // self.reg_a / denom
        self.reg_a >> self.combo_operand(operand)
    }

    fn bxl(&self, operand: u8) -> usize {
        self.reg_b ^ operand as usize
    }

    fn bst(&self, operand: u8) -> usize {
        self.combo_operand(operand) % 8
    }

    fn bxc(&self, _operand: u8) -> usize {
        self.reg_b ^ self.reg_c
    }

    fn tick(&mut self) -> bool {
        let opcode = self.program[self.pc];
        let operand = self.program[self.pc + 1];

        match opcode {
            0 => {
                // adv
                self.reg_a = self.adv(operand);
                self.pc += 2;
            }
            1 => {
                // bxl
                self.reg_b = self.bxl(operand);
                self.pc += 2;
            }
            2 => {
                // bst
                self.reg_b = self.bst(operand);
                self.pc += 2;
            }
            3 => {
                // jnz
                if self.reg_a != 0 {
                    self.pc = operand as usize;
                } else {
                    self.pc += 2;
                }
            }
            4 => {
                // bxc
                self.reg_b = self.bxc(operand);
                self.pc += 2;
            }
            5 => {
                // out
                self.output_buffer
                    .push(self.combo_operand(operand) as u8 % 8);
                self.pc += 2;
            }
            6 => {
                // bdv
                self.reg_b = self.adv(operand);
                self.pc += 2;
            }
            7 => {
                // cdv
                self.reg_c = self.adv(operand);
                self.pc += 2;
            }
            _ => {
                todo!("add opcodes {opcode}");
            }
        }

        if self.pc >= self.program.len() {
            return false;
        }

        true
    }
}

fn parse_register(line: &str) -> usize {
    if line.len() > 12 {
        return line[12..].parse::<usize>().unwrap();
    }
    panic!("Register malformed: {:?}", line);
}

fn parse_program(line: &str) -> Vec<u8> {
    if line.len() > 8 {
        line[9..]
            .split(',')
            .map(|op| op.parse::<u8>().unwrap())
            .collect()
    } else {
        panic!("Program malformed: {:?}", line);
    }
}

fn test_number(num: usize, machine: &mut VirtualMachine) -> Option<usize> {
    for i in 0..8 {
        let test_num = (num << 3) | i;
        machine.reset(test_num);
        while machine.tick() {}

        if machine.output_buffer == machine.program {
            return Some(test_num);
        }

        if machine.output_buffer.len() > machine.program.len() {
            return None;
        }

        if machine.program.ends_with(&machine.output_buffer) {
            let next_digits = test_number(test_num, machine);
            if next_digits.is_some() {
                return next_digits;
            }
        }
    }

    None
}

impl AoCSolution for Solution {
    type Parsed = VirtualMachine;

    fn parse(&self, input: &str) -> Self::Parsed {
        let elems: Vec<_> = input_lines(input).filter(|l| !l.is_empty()).collect();
        let a = parse_register(elems[0]);
        let b = parse_register(elems[1]);
        let c = parse_register(elems[2]);

        let prog = parse_program(elems[3]);
        VirtualMachine::new(a, b, c, &prog)
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut machine = data.clone();

        while machine.tick() {}

        machine
            .output_buffer
            .iter()
            .map(|n| n.to_string())
            .join(",")
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut machine = data.clone();

        for i in 1..8 {
            if let Some(num) = test_number(i, &mut machine) {
                return num;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const EXAMPLE_INPUT_2: &str = r"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn test_part1() {
        assert_eq!(
            Solution::default().solve1(EXAMPLE_INPUT_1),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT_2), "117440");
    }
}
