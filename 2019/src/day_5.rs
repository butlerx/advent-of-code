// parse opscode, returning the code and if the args are in position mode or imediate
// if an arg is true its in  imediate mode
fn parse_opcode(code: i64) -> usize {
    code as usize % 100
}

fn read_mem(memory: &Vec<i64>, pointer: usize, mem: usize) -> i64 {
    if (memory[pointer] / 10i64.pow(mem as u32 + 1)) % 10 == 1 {
        memory[pointer + mem]
    } else {
        memory[memory[pointer + mem] as usize]
    }
}

fn part_1(memory: Vec<i64>, input: i64) -> i64 {
    let mut mem = memory.clone();
    let mut pointer = 0;
    let mut output = 0;
    loop {
        match parse_opcode(mem[pointer]) {
            1 => {
                let pos = mem[pointer + 3] as usize;
                mem[pos] = read_mem(&mem, pointer, 1) + read_mem(&mem, pointer, 2);
                pointer += 4;
            }
            2 => {
                let pos = mem[pointer + 3] as usize;
                mem[pos] = read_mem(&mem, pointer, 1) * read_mem(&mem, pointer, 2);
                pointer += 4;
            }
            3 => {
                let pos = mem[pointer + 1] as usize;
                mem[pos] = input;
                pointer += 2;
            }
            4 => {
                output = read_mem(&mem, pointer, 1);
                println!("Output: {}", output);
                pointer += 2;
            }
            99 => return output as i64,
            _ => unreachable!(),
        }
    }
}

fn part_2(memory: Vec<i64>, input: i64) -> i64 {
    let mut mem = memory.clone();
    let mut pointer = 0;
    let mut output = 0;
    loop {
        match parse_opcode(mem[pointer]) {
            1 => {
                let pos = mem[pointer + 3] as usize;
                mem[pos] = read_mem(&mem, pointer, 1) + read_mem(&mem, pointer, 2);
                pointer += 4;
            }
            2 => {
                let pos = mem[pointer + 3] as usize;
                mem[pos] = read_mem(&mem, pointer, 1) * read_mem(&mem, pointer, 2);
                pointer += 4;
            }
            3 => {
                let pos = mem[pointer + 1] as usize;
                mem[pos] = input;
                pointer += 2;
            }
            4 => {
                output = read_mem(&mem, pointer, 1);
                println!("Output: {}", output);
                pointer += 2;
            }
            5 => {
                pointer = if read_mem(&mem, pointer, 1) != 0 {
                    read_mem(&mem, pointer, 2) as usize
                } else {
                    pointer + 3
                };
            }
            6 => {
                pointer = if read_mem(&mem, pointer, 1) == 0 {
                    read_mem(&mem, pointer, 2) as usize
                } else {
                    pointer + 3
                };
            }
            7 => {
                let pos = mem[pointer + 3] as usize;
                mem[pos] = if read_mem(&mem, pointer, 1) < read_mem(&mem, pointer, 2) {
                    1
                } else {
                    0
                };
                pointer += 4;
            }
            8 => {
                let pos = mem[pointer + 3] as usize;
                mem[pos] = if read_mem(&mem, pointer, 1) == read_mem(&mem, pointer, 2) {
                    1
                } else {
                    0
                };
                pointer += 4;
            }
            99 => return output as i64,
            _ => unreachable!(),
        }
    }
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let nums = input
        .split(",")
        .map(|num| num.trim().parse().unwrap())
        .collect();
    if part_two {
        part_2(nums, 5)
    } else {
        part_1(nums, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let results = run(include_str!("../input/day_5.txt"), false);
        println!("{}", results);
        assert!(results == 13294380);
    }

    #[test]
    fn test_part_2() {
        let results = run(include_str!("../input/day_5.txt"), true);
        println!("{}", results);
        assert!(results == 11460760);
    }
}
