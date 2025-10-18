#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::time_execution;
use std::collections::{BTreeSet, HashMap};

static INPUT_TXT: &str = include_str!("../../input/24.txt");

fn main() {
    println!("🌟 --- Day 24 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Operation<'a> {
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

impl Operation<'_> {
    fn is_input(&self, input: &str) -> bool {
        match self {
            Operation::And(a, b) | Operation::Or(a, b) | Operation::Xor(a, b) => {
                *a == input || *b == input
            }
        }
    }
}

impl<'a> From<&'a str> for Operation<'a> {
    fn from(s: &'a str) -> Self {
        let parts: Vec<&str> = s.split(' ').collect();
        let (left, right) = if parts[0] < parts[2] {
            (parts[0], parts[2])
        } else {
            (parts[2], parts[0])
        };
        match parts.get(1) {
            Some(&"AND") => Operation::And(left, right),
            Some(&"OR") => Operation::Or(left, right),
            Some(&"XOR") => Operation::Xor(left, right),
            _ => panic!("Invalid operation"),
        }
    }
}

fn parse_input(input: &str) -> (HashMap<&str, bool>, HashMap<&str, Operation<'_>>) {
    let (start_nums, instructions_str) = input.trim().split_once("\n\n").expect("Invalid input");
    let values = start_nums
        .lines()
        .map(|line| line.split_once(": ").expect("Invalid input no colon"))
        .map(|(start, value)| (start, value == "1"))
        .collect::<HashMap<_, _>>();

    let instructions = instructions_str
        .lines()
        .filter_map(|l| l.split_once(" -> "))
        .map(|(inputs, output)| {
            let operation = Operation::from(inputs);
            (output, operation)
        })
        .collect::<HashMap<_, _>>();
    (values, instructions)
}

fn exec<'a>(
    instructions: &HashMap<&'a str, Operation<'a>>,
    values: &mut HashMap<&'a str, bool>,
    n: &'a str,
) {
    if values.contains_key(n) {
        return;
    }

    let result = match &instructions[n] {
        Operation::And(left, right) => {
            exec(instructions, values, left);
            exec(instructions, values, right);
            values[left] && values[right]
        }
        Operation::Or(left, right) => {
            exec(instructions, values, left);
            exec(instructions, values, right);
            values[left] || values[right]
        }
        Operation::Xor(left, right) => {
            exec(instructions, values, left);
            exec(instructions, values, right);
            values[left] != values[right]
        }
    };
    values.insert(n, result);
}

fn binary_output(values: &HashMap<&str, bool>, p: char) -> u64 {
    values
        .iter()
        .filter(|(&key, _)| key.starts_with(p))
        .fold(0, |acc, (&key, &value)| {
            let i = key[1..].parse::<usize>().unwrap();
            if value {
                acc | (1 << i)
            } else {
                acc
            }
        })
}

fn part_1(input: &str) -> u64 {
    let (mut store, instructions) = parse_input(input);
    for output in instructions.keys().copied() {
        exec(&instructions, &mut store, output);
    }

    binary_output(&store, 'z')
}

fn part_2(input: &str) -> String {
    let (store, instructions) = parse_input(input);

    let mut swapped = BTreeSet::new();
    let z00 = instructions
        .iter()
        .find_map(|(output, o)| match o {
            Operation::Xor(left, right) if *left == "x00" && *right == "y00" => Some(output),
            _ => None,
        })
        .expect("No initial z00 found");

    if *z00 != "z00" {
        swapped.insert((*z00).to_string());
    }

    let mut carry: &str = instructions
        .iter()
        .find_map(|(output, o)| match o {
            Operation::And(left, right) if *left == "x00" && *right == "y00" => Some(output),
            _ => None,
        })
        .expect("No initial carry found");

    for bit in 1..store.len() / 2 {
        let x = format!("x{bit:02}");
        let y = format!("y{bit:02}");
        let basic_add = instructions
            .iter()
            .find(|(_, o)| match o {
                Operation::Xor(left, right) => left == &x && right == &y,
                _ => false,
            })
            .map(|(output, _)| output)
            .expect("No basic add found");

        let add = instructions
            .iter()
            .find(|(_, o)| match o {
                Operation::Xor(_, _) => o.is_input(carry) || o.is_input(basic_add),
                _ => false,
            })
            .expect("No add found");

        let z = format!("z{bit:02}");
        if add.0 != &z {
            swapped.insert(z);
            swapped.insert((*add.0).to_string());
        }

        if !add.1.is_input(basic_add) {
            swapped.insert((*basic_add).to_string());
        }

        let basic_carry = instructions
            .iter()
            .find(|(_, o)| match o {
                Operation::And(left, right) => *left == x && *right == y,
                _ => false,
            })
            .map(|(output, _)| output)
            .expect("No basic carry found");

        let cascade_carry = instructions
            .iter()
            .find(|(_, o)| match o {
                Operation::And(_, _) => o.is_input(basic_add) || o.is_input(carry),
                _ => false,
            })
            .map(|(output, _)| output)
            .expect("No cascade carry found");

        let (output, carry_gate) = instructions
            .iter()
            .find(|(_, o)| match o {
                Operation::Or(_, _) => o.is_input(cascade_carry) || o.is_input(basic_carry),
                _ => false,
            })
            .expect("No carry gate found");

        if !carry_gate.is_input(basic_carry) {
            swapped.insert((*basic_carry).to_string());
        }

        carry = output;
    }

    swapped.into_iter().collect::<Vec<_>>().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    static INPUT_2: &str = "x00: 1
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

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 4);
        assert_eq!(part_1(INPUT_2), 2024);
        assert_eq!(part_1(INPUT_TXT), 57_344_080_719_736);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_TXT), "cgq,fnr,kqk,nbc,svm,z15,z23,z39");
    }
}
