use advent_of_code_2024::aoc;
use itertools::Itertools;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum Register {
    A, B, C,
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Adv, // 0
    Bxl, // 1
    Bst, // 2
    Jnz, // 3
    Bxc, // 4
    Out, // 5
    Bdv, // 6
    Cdv, // 7
}

impl Instruction {
    fn from_opcode(opcode: i64) -> Self {
        use Instruction::*;

        match opcode {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Computer {
    instruction_pointer: usize,
    registers: [i64; 3],
    program: Vec<i64>,
}

impl Computer {
    fn from_input(input: &str) -> Self {
        let re = Regex::new(r"Register [A-C]: (\d+)").unwrap();

        let mut captures = re.captures_iter(input);
        let a = captures
            .next()
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();
        let b = captures
            .next()
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();
        let c = captures
            .next()
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();

        let re = Regex::new(r"Program: (\d(?:,\d)*)").unwrap();
        let program_str = re.captures(input).unwrap().get(1).unwrap().as_str();
        let program = program_str
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();

        Self {
            instruction_pointer: 0,
            registers: [a, b, c],
            program,
        }
    }

    fn read_instruction(&self) -> Option<Instruction> {
        if self.instruction_pointer >= self.program.len() {
            None
        } else {
            Some(Instruction::from_opcode(
                self.program[self.instruction_pointer],
            ))
        }
    }

    fn read_operand(&self) -> Option<i64> {
        if self.instruction_pointer + 1 >= self.program.len() {
            None
        } else {
            Some(self.program[self.instruction_pointer + 1])
        }
    }

    fn increment(&mut self) {
        self.instruction_pointer += 2;
    }

    fn read_register(&self, register: Register) -> i64 {
        match register {
            Register::A => self.registers[0],
            Register::B => self.registers[1],
            Register::C => self.registers[2],
        }
    }

    fn write_register(&mut self, register: Register, val: i64) {
        match register {
            Register::A => self.registers[0] = val,
            Register::B => self.registers[1] = val,
            Register::C => self.registers[2] = val,
        }
    }
    
    fn combo(&self, operand: i64) -> i64 {
        match operand {
            x @ 0..=3 => x,
            4 => self.read_register(Register::A),
            5 => self.read_register(Register::B),
            6 => self.read_register(Register::C),
            7 => unreachable!(),
            _ => unreachable!(),
        }
    }

    fn execute(&mut self, instruction: Instruction, operand: i64) -> Option<i64> {
        let mut should_increment = true;
        let mut output_val = None;
        match instruction {
            Instruction::Adv => {
                let numerator = self.read_register(Register::A);
                let denominator = 2i64.pow(self.combo(operand) as u32);
                let result = numerator / denominator;
                self.write_register(Register::A, result);
            },
            Instruction::Bxl => {
                let result = self.read_register(Register::B) ^ operand;
                self.write_register(Register::B, result);
            },
            Instruction::Bst => {
                self.write_register(Register::B, self.combo(operand) % 8);
            },
            Instruction::Jnz => {
                if self.read_register(Register::A) != 0 {
                    self.instruction_pointer = operand as usize;
                    should_increment = false;
                }
            },
            Instruction::Bxc => {
                let result = self.read_register(Register::B) ^ self.read_register(Register::C);
                self.write_register(Register::B, result);
            },
            Instruction::Out => {
                output_val = Some(self.combo(operand) % 8);
            },
            Instruction::Bdv => {
                let numerator = self.read_register(Register::A);
                let denominator = 2i64.pow(self.combo(operand) as u32);
                let result = numerator / denominator;
                self.write_register(Register::B, result);
            },
            Instruction::Cdv => {
                let numerator = self.read_register(Register::A);
                let denominator = 2i64.pow(self.combo(operand) as u32);
                let result = numerator / denominator;
                self.write_register(Register::C, result);
            },
        };

        if should_increment {
            self.increment();
        }

        output_val
    }

    fn run(&mut self) -> Vec<i64> {
        let mut output: Vec<i64> = vec![];

        loop {
            let Some(instruction) = self.read_instruction() else {
                break;
            };
            let Some(operand) = self.read_operand() else {
                break;
            };

            if let Some(output_val) = self.execute(instruction, operand) {
                output.push(output_val);
            }
            dbg!(instruction, operand);
            dbg!(self.registers);
        }

        output
    }
}

fn main() {
    let input = include_str!("input.txt");
    aoc::run_parts(input, part_1, part_2);
}

fn part_1(input: &str) -> String {
    let mut computer = Computer::from_input(input);
    dbg!(&computer);
    let output = computer.run();
    dbg!(&output);
    output
        .iter()
        .map(|x| format!("{x}"))
        .collect_vec()
        .join(",")
}

fn part_2(input: &str) -> String {
    "hi".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
        ";

        let computer = Computer::from_input(input);
        let expected = Computer {
            instruction_pointer: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            registers: [729, 0, 0],
        };
        assert_eq!(computer, expected);
    }
}
