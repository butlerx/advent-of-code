use regex::Regex;
use std::collections::HashMap;

const MASK_36_BIT: u64 = (1u64 << 36) - 1;

fn main() {
    let input = include_str!("../../input/14.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn parse_input(input: &str) -> (usize, u64) {
    let cap = Regex::new(r"mem\[(\d+)\] = (\d+)")
        .unwrap()
        .captures(input)
        .unwrap();
    (
        cap[1].parse::<usize>().unwrap(),
        cap[2].trim().parse::<u64>().unwrap(),
    )
}

fn parse_mask(mask: &str) -> (u64, u64) {
    let mut ones = 0u64;
    let mut zeroes = 0u64;
    for bit in mask.bytes() {
        zeroes <<= 1;
        ones <<= 1;
        match bit {
            b'1' => ones += 1,
            b'0' => zeroes += 1,
            _ => (),
        };
    }
    (zeroes, ones)
}
fn parse_mask_floating(mask: &str) -> Vec<u64> {
    let mut parsed = Vec::new();
    for bit in mask.chars().rev() {
        match bit {
            '1' => parsed.push(1),
            '0' => parsed.push(0),
            'X' => parsed.push(2),
            _ => (),
        };
    }
    parsed
}

fn write(
    memory: &mut HashMap<usize, u64>,
    mask: &[u64],
    address: usize,
    value: u64,
    current_index: usize,
) {
    if current_index >= mask.len() {
        memory.insert(address, value);
        return;
    }
    match mask[current_index] {
        0 => write(memory, mask, address, value, current_index + 1),
        1 => write(
            memory,
            mask,
            (address & !(1 << current_index)) | (1 << current_index),
            value,
            current_index + 1,
        ),
        2 => {
            let address = address & !(1 << current_index);
            write(memory, mask, address, value, current_index + 1);
            let address = (address & !(1 << current_index)) | (1 << current_index);
            write(memory, mask, address, value, current_index + 1);
        }
        _ => unreachable!(),
    }
}

fn part_one(instructions: &str) -> i64 {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let re = Regex::new(r"mask = ([10X]+)").unwrap();
    let mut zeroes_mask = MASK_36_BIT;
    let mut ones_mask = 0u64;
    for instruction in instructions.lines() {
        if re.is_match(instruction) {
            let (zeroes, ones) = parse_mask(&re.captures(instruction).unwrap()[1]);
            zeroes_mask = MASK_36_BIT & !zeroes;
            ones_mask = ones;
        } else {
            let (pos, value) = parse_input(instruction);
            mem.insert(pos, (value | ones_mask) & zeroes_mask);
        }
    }
    mem.values().sum::<u64>() as i64
}

fn part_two(instructions: &str) -> i64 {
    let re = Regex::new(r"mask = ([10X]+)").unwrap();
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut mask: Vec<u64> = Vec::new();
    for instruction in instructions.lines() {
        if re.is_match(instruction) {
            mask = parse_mask_floating(&re.captures(instruction).unwrap()[1]);
        } else {
            let (pos, value) = parse_input(instruction);
            // assume re_mem matches
            write(&mut mem, &mask, pos, value, 0);
        }
    }
    mem.values().sum::<u64>() as i64
}

pub fn run(input: &str, version_two: bool) -> i64 {
    if version_two {
        part_two(input)
    } else {
        part_one(input)
    }
}

#[cfg(test)]
mod day_14_tests {
    use super::*;
    static INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    static INPUT_2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 165);
        let results = run(include_str!("../../input/14.txt"), false);
        println!("{}", results);
        assert!(results == 17481577045893);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT_2, true) == 208);
        let results = run(include_str!("../../input/14.txt"), true);
        println!("{}", results);
        assert!(results == 4160009892257);
    }
}
