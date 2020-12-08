fn execute(instruction_set: &Vec<(&str, i64)>) -> Result<i64, i64> {
    let mut accumulator = 0;
    let mut visited = vec![false; instruction_set.len()];
    let mut pointer = 0;
    while pointer < instruction_set.len() {
        let (command, arg) = instruction_set[pointer];
        if visited[pointer] {
            return Err(accumulator);
        } else {
            visited[pointer] = true;
        }
        match command {
            "nop" => {
                pointer += 1;
            }
            "acc" => {
                accumulator += arg;
                pointer += 1;
            }
            "jmp" => {
                pointer = (pointer as i64 + arg) as usize;
            }
            _ => unreachable!(),
        }
    }
    Ok(accumulator)
}

fn swap_operations(instruction: &mut (&str, i64)) {
    match instruction {
        ("nop", _) => {
            instruction.0 = "jmp";
        }
        ("jmp", _) => {
            instruction.0 = "nop";
        }
        _ => (),
    }
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let mut instruction_set = input
        .lines()
        .map(|line| {
            let instruction = line.trim().split(" ").collect::<Vec<&str>>();
            (
                instruction[0].trim(),
                instruction[1].parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<(&str, i64)>>();

    if part_two {
        let mut solution = 0;
        for i in 0..instruction_set.len() {
            swap_operations(&mut instruction_set[i]);

            if let Ok(accumulator) = execute(&instruction_set) {
                solution = accumulator;
                break;
            }

            swap_operations(&mut instruction_set[i]);
        }
        solution
    } else {
        match execute(&instruction_set) {
            Ok(accumulator) => accumulator,
            Err(accumulator) => accumulator,
        }
    }
}

#[cfg(test)]
mod tests {
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
        let results = run(include_str!("../input/day_8.txt"), false);
        println!("{}", results);
        assert!(results == 1528);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 8);
        let results = run(include_str!("../input/day_8.txt"), true);
        println!("{}", results);
        assert!(results == 640);
    }
}
