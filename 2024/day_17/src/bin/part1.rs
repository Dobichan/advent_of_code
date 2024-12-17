struct VirtualMachine {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
    pc: i32,
    program: Vec<u8>,
    output_buffer: String,
}

impl VirtualMachine {
    fn new(a: i32, b: i32, c: i32, prog: &Vec<u8>) -> VirtualMachine {
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
            "a: {:?} b: {:?} c: {:?} pc: {:?} out: {:?}",
            self.reg_a, self.reg_b, self.reg_c, self.pc, self.output_buffer
        );
    }
    fn combo_operand(&self, operand: u8) -> i32 {
        match operand {
            0..=3 => operand as i32,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => todo!("Reserved combo operand! Shall not happen"),
            _ => todo!("Illegal operand larger than 3 bits"),
        }
    }

    fn adv(&self, operand: u8) -> i32 {
        let denom = i32::pow(2, self.combo_operand(operand) as u32);
        self.reg_a / denom
    }

    fn bxl(&self, operand: u8) -> i32 {
        self.reg_b ^ operand as i32
    }

    fn bst(&self, operand: u8) -> i32 {
        self.combo_operand(operand) % 8
    }

    fn bxc(&self, _operand: u8) -> i32 {
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
            6 => { // bdv
                self.reg_b = self.adv(operand);
                self.pc += 2;
            }
            7 => { // cdv
                self.reg_c = self.adv(operand);
                self.pc += 2;
            }
            _ => {
                todo!("add opcodes {opcode}");
            }
        }
        if self.pc >= self.program.len() as i32 {
            return false;
        }
        true
    }
}

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let inp: Vec<&str> = input.lines().collect();
    println!("{:?}", inp);
    let a: i32 = inp[0]
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let b: i32 = inp[1]
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let c: i32 = inp[2]
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

    let mut machine = VirtualMachine::new(a, b, c, &program);
    machine.print_status();

    let mut l = 0;
    while machine.tick() == true && l < 100 {
        machine.print_status();
        l += 1;
    }

    machine.output_buffer
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        assert_eq!(part1(dummy), "4,6,3,5,6,3,5,2,1,0,");
    }
}
