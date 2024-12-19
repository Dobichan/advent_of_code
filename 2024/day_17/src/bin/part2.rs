struct VirtualMachine {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    pc: i32,
    program: Vec<u8>,
    output_buffer: String,
}

impl VirtualMachine {
    fn new(a: u64, b: u64, c: u64, prog: &Vec<u8>) -> VirtualMachine {
        VirtualMachine {
            reg_a: a,
            reg_b: b,
            reg_c: c,
            pc: 0,
            program: prog.clone(),
            output_buffer: String::new(),
        }
    }

    fn print_status(&self) {
        println!(
            "a: {:?} {:018o} b: {:?} c: {:?} pc: {:?} out: {:?}",
            self.reg_a, self.reg_a, self.reg_b, self.reg_c, self.pc, self.output_buffer
        );
    }
    fn combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => todo!("Reserved combo operand! Shall not happen"),
            _ => todo!("Illegal operand larger than 3 bits"),
        }
    }

    fn adv(&self, operand: u8) -> u64 {
        self.reg_a >> self.combo_operand(operand)
    }

    fn bxl(&self, operand: u8) -> u64 {
        self.reg_b ^ operand as u64
    }

    fn bst(&self, operand: u8) -> u64 {
        self.combo_operand(operand) & 0x07
    }

    fn bxc(&self, _operand: u8) -> u64 {
        self.reg_b ^ self.reg_c
    }

    fn tick(&mut self) -> bool {
        let opcode = self.program[self.pc as usize];
        let operand = self.program[(self.pc + 1) as usize];
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
                    self.pc = operand as i32;
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
                    .push_str(&(self.combo_operand(operand) % 8).to_string());
                self.output_buffer.push(',');
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
        if self.pc >= self.program.len() as i32 {
            if self.output_buffer.len() > 0 {
                self.output_buffer.pop();
            }
            return false;
        }
        true
    }
}

fn main() {
    println!("Part2!");

    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u64 {
    let inp: Vec<&str> = input.lines().collect();
    println!("{:?}", inp);
    let a: u64 = inp[0]
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let b: u64 = inp[1]
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let c: u64 = inp[2]
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    println!("{a} {b} {c}");

    let program: Vec<u8> = inp[4]
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|i| i.parse::<u8>().unwrap())
        .collect();

    println!("{:?}", program);

    for i in 1..=7 {
        let num = test_number(i, 0, &program, b, c);
        if num.is_some() {
            return num.unwrap();
        }
    }

    0
}

fn test_number(num: u64, level: u32, digits: &Vec<u8>, b: u64, c: u64) -> Option<u64> {
    let to_test = digits
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{level} - {num} to test: {:?}", to_test);
    for i in 0..=7 {
        let test_num = (num << 3) | i;
        let mut machine = VirtualMachine::new(test_num, b, c, &digits);
        while machine.tick() {}

        println!("{test_num:018o} => {:?}", machine.output_buffer);

        if machine.output_buffer == to_test {
            println!("Found the number at level {level}");
            return Some(test_num);
        }

        if machine.output_buffer.len() > to_test.len() {
            println!(
                "Number probably too large - machine: {:?}",
                machine.output_buffer
            );
            return None;
        }

        if to_test.ends_with(&machine.output_buffer) {
            let next_level = test_number(test_num, level + 1, digits, b, c);
            if next_level.is_some() {
                return next_level;
            }
        }
    }
    None
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let dummy = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

        assert_eq!(part2(dummy), 117440);
    }
}
