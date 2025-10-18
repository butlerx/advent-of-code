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

impl Source {
    fn from_str(s: &str) -> Self {
        match s {
            "w" => Self::Reg(0),
            "x" => Self::Reg(1),
            "y" => Self::Reg(2),
            "z" => Self::Reg(3),
            _ => Self::Val(s.parse().unwrap()),
        }
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
        let src = s[4..5].parse::<usize>().unwrap();
        match &s[..3] {
            "inp" => Some(Self::Inp),
            "add" => Some(Self::Add(src, Source::from_str(&s[6..]))),
            "mul" => Some(Self::Mul(src, Source::from_str(&s[6..]))),
            "div" => Some(Self::Div(src, Source::from_str(&s[6..]))),
            "mod" => Some(Self::Mod(src, Source::from_str(&s[6..]))),
            "eql" => Some(Self::Eql(src, Source::from_str(&s[6..]))),
            _ => None,
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().filter_map(Instruction::from_str).collect()
}

fn find_model_num(
    cache: &mut Cache,
    instrunctions: &[Instruction],
    pc: usize,
    z: i64,
    range: &[i64],
) -> Option<i64> {
    if let Some(&answer) = cache.get(&(z, pc)) {
        answer
    } else {
        for &digit in range {
            let mut regs = [digit, 0, 0, z];
            for _ in pc..pc + 17 {
                instrunctions[pc].exec(&mut regs);
            }
            let z = regs[3];
            if pc + 17 == instrunctions.len() {
                if z == 0 {
                    cache.insert((z, pc), Some(digit));
                    return Some(digit);
                }
            } else if let Some(best) = find_model_num(cache, instrunctions, pc + 18, z, range) {
                cache.insert((z, pc), Some(best * 10 + digit));
                return Some(best * 10 + digit);
            }
        }

        cache.insert((z, pc), None);
        None
    }
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
