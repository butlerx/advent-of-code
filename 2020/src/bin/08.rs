use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/08.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn execute(instruction_set: Vec<(&str, i64)>) -> Result<i64, i64> {
    let (mut pointer, mut accumulator) = (0, 0);
    let mut visited = vec![false; instruction_set.len()];
    loop {
        if pointer >= instruction_set.len() {
            break Ok(accumulator);
        } else if visited[pointer] {
            break Err(accumulator);
        } else {
            visited[pointer] = true;
        }
        pointer = match instruction_set[pointer] {
            ("nop", _) => pointer + 1,
            ("acc", arg) => {
                accumulator += arg;
                pointer + 1
            }
            ("jmp", arg) => (pointer as i64 + arg) as usize,
            _ => unreachable!(),
        }
    }
}

fn fix_instruction_set(instruction_set: Vec<(&str, i64)>, pointer: usize) -> Vec<(&str, i64)> {
    let mut new_instruction_set = instruction_set.clone();
    new_instruction_set[pointer].0 = match new_instruction_set[pointer] {
        ("nop", _) => "jmp",
        ("jmp", _) => "nop",
        (code, _) => code,
    };
    new_instruction_set
}

fn run(input: &str, part_two: bool) -> i64 {
    let instruction_set = input
        .lines()
        .map(|line| {
            let (code, arg) = line.trim().split(' ').collect_tuple().unwrap();
            (code.trim(), arg.parse::<i64>().unwrap())
        })
        .collect::<Vec<(&str, i64)>>();

    if part_two {
        for i in 0..instruction_set.len() {
            if let Ok(accumulator) = execute(fix_instruction_set(instruction_set.clone(), i)) {
                return accumulator;
            }
        }
        0
    } else {
        // Dont care if it fails or succedes just want the value of the accumulator at
        // end of the first run
        match execute(instruction_set) {
            Ok(accumulator) => accumulator,
            Err(accumulator) => accumulator,
        }
    }
}

#[cfg(test)]
mod day_8_tests {
    use super::*;
    static INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 5);
        let results = run(include_str!("../../input/08.txt"), false);
        println!("{}", results);
        assert!(results == 1528);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 8);
        let results = run(include_str!("../../input/08.txt"), true);
        println!("{}", results);
        assert!(results == 640);
    }
}
