use std::collections::HashMap;

type Cache = HashMap<(i64, usize), Option<i64>>;

fn main() {
    let input = include_str!("../../input/24.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i64 {
    run(&parse_input(input), true)
}

fn part_2(input: &str) -> i64 {
    run(&parse_input(input), false)
}

#[derive(Clone, Copy)]
enum Source {
    Reg(usize),
    Val(i64),
}

fn register(s: &str) -> Option<usize> {
    match s {
        "w" => Some(0),
        "x" => Some(1),
        "y" => Some(2),
        "z" => Some(3),
        _ => None,
    }
}

impl Source {
    fn from_str(s: &str) -> Self {
        register(s).map_or_else(|| Self::Val(s.parse().unwrap()), Self::Reg)
    }

    fn val(&self, regs: &[i64; 4]) -> i64 {
        match *self {
            Self::Reg(i) => regs[i],
            Self::Val(v) => v,
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Inp,
    Add(usize, Source),
    Mul(usize, Source),
    Div(usize, Source),
    Mod(usize, Source),
    Eql(usize, Source),
}

impl Instruction {
    fn exec(&self, regs: &mut [i64; 4]) {
        match self {
            Self::Add(a, b) => regs[*a] += b.val(regs),
            Self::Mul(a, b) => regs[*a] *= b.val(regs),
            Self::Div(a, b) => regs[*a] /= b.val(regs),
            Self::Mod(a, b) => regs[*a] %= b.val(regs),
            Self::Eql(a, b) => regs[*a] = i64::from(regs[*a] == b.val(regs)),
            Self::Inp => unreachable!(),
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        let mut parts = s.split_whitespace();
        let op = parts.next()?;
        if op == "inp" {
            return Some(Self::Inp);
        }
        let dst = register(parts.next()?)?;
        let src = Source::from_str(parts.next()?);
        match op {
            "add" => Some(Self::Add(dst, src)),
            "mul" => Some(Self::Mul(dst, src)),
            "div" => Some(Self::Div(dst, src)),
            "mod" => Some(Self::Mod(dst, src)),
            "eql" => Some(Self::Eql(dst, src)),
            _ => None,
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().filter_map(Instruction::from_str).collect()
}

fn find_model_num(
    cache: &mut Cache,
    instructions: &[Instruction],
    pc: usize,
    z: i64,
    range: &[i64],
) -> Option<i64> {
    if let Some(&answer) = cache.get(&(z, pc)) {
        return answer;
    }
    let mut result = None;
    for &digit in range {
        let mut regs = [digit, 0, 0, z];
        for instruction in &instructions[pc..pc + 17] {
            instruction.exec(&mut regs);
        }
        let next_z = regs[3];
        if pc + 17 == instructions.len() {
            if next_z == 0 {
                result = Some(digit);
                break;
            }
        } else if let Some(best) = find_model_num(cache, instructions, pc + 18, next_z, range) {
            result = Some(best * 10 + digit);
            break;
        }
    }
    cache.insert((z, pc), result);
    result
}

fn run(instrunctions: &[Instruction], biggest: bool) -> i64 {
    let range: Vec<i64> = if biggest {
        (1..=9).rev().collect()
    } else {
        (1..=9).collect()
    };
    find_model_num(&mut Cache::new(), instrunctions, 1, 0, &range)
        .unwrap()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}

#[cfg(test)]
mod day_24_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../input/24.txt")),
            45_989_929_946_199
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../input/24.txt")),
            11_912_814_611_156
        );
    }
}
