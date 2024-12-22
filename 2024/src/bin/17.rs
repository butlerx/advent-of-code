#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::time_execution;

static INPUT_TXT: &str = include_str!("../../input/17.txt");

fn main() {
    println!("🌟 --- Day 17 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

struct Register {
    a: u64,
    b: u64,
    c: u64,
}

impl From<&str> for Register {
    fn from(s: &str) -> Self {
        let registers_vec = s
            .lines()
            .map(|x| x.split(": ").nth(1).unwrap().parse().unwrap())
            .collect::<Vec<_>>();
        Register {
            a: registers_vec[0],
            b: registers_vec[1],
            c: registers_vec[2],
        }
    }
}

impl Register {
    fn new(a: u64) -> Self {
        Register { a, b: 0, c: 0 }
    }

    const fn combo(&self, x: u64) -> u64 {
        match x {
            0..=3 => x,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn run(&mut self, program: &[u64]) -> Vec<u64> {
        let mut output = Vec::with_capacity(program.len() / 2); // Preallocate vector
        let mut pointer = 0;

        while pointer < program.len() {
            let opcode = program[pointer];
            let operand = program[pointer + 1];
            pointer += 2;

            // Use direct matching instead of parse_instruction
            match opcode {
                0 => self.a >>= self.combo(operand),
                1 => self.b ^= operand,
                2 => self.b = self.combo(operand) % 8,
                3 if self.a != 0 => {
                    pointer = usize::try_from(operand).expect("could not convert u64 to usize");
                }
                4 => self.b ^= self.c,
                5 => output.push(self.combo(operand) % 8),
                6 => self.b = self.a >> self.combo(operand),
                7 => self.c = self.a >> self.combo(operand),
                _ => (),
            }
        }
        output
    }
}

fn parse_input(input: &str) -> (Register, Vec<u64>) {
    let mut blocks = input.split("\n\n");
    let registers = blocks
        .next()
        .map(Register::from)
        .expect("No registers found");
    let program = blocks
        .next()
        .and_then(|s| s.strip_prefix("Program: "))
        .map(|s| {
            s.split(',')
                .filter_map(|x| x.trim().parse().ok())
                .collect::<Vec<_>>()
        })
        .unwrap();
    (registers, program)
}

fn part_1(input: &str) -> String {
    let (mut registers, program) = parse_input(input);
    registers
        .run(&program)
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join(",")
}

fn part_2(input: &str) -> u64 {
    let (_, program) = parse_input(input);
    let mut factors = vec![0; program.len()];

    loop {
        let mut init_a = 0;
        for (i, f) in factors.iter().enumerate() {
            init_a += 8u64.pow(u32::try_from(i).expect("could not convert usize to u32")) * f;
        }

        let output = Register::new(init_a).run(&program);

        if output == program {
            break init_a;
        }

        for i in (0..program.len()).rev() {
            if output.len() < i {
                factors[i] += 1;
                break;
            }
            if output[i] != program[i] {
                factors[i] += 1;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    static INPUT_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(part_1(INPUT_TXT), "1,4,6,1,6,4,3,0,3");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_2), 117_440);
        assert_eq!(part_2(INPUT_TXT), 265_061_364_597_659);
    }
}
