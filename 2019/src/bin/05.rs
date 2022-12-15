fn main() {
    let input = include_str!("../../input/05.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

// parse opscode, returning the code and if the args are in position mode or imediate
// if an arg is true its in  imediate mode
fn parse_opcode(code: i64) -> usize {
    code as usize % 100
}

fn read_mem(memory: &[i64], pointer: usize, mem: usize) -> i64 {
    if (memory[pointer] / 10i64.pow(mem as u32 + 1)) % 10 == 1 {
        memory[pointer + mem]
    } else {
        memory[memory[pointer + mem] as usize]
    }
}

fn execute(memory: Vec<i64>, input: i64) -> i64 {
    let mut mem = memory;
    let mut pointer = 0;
    let mut output = 0;
    loop {
        pointer = match parse_opcode(mem[pointer]) {
            1 => {
                let pos = mem[pointer + 3] as usize;
                mem[pos] = read_mem(&mem, pointer, 1) + read_mem(&mem, pointer, 2);
                pointer + 4
            }
            2 => {
                let pos = mem[pointer + 3] as usize;
                mem[pos] = read_mem(&mem, pointer, 1) * read_mem(&mem, pointer, 2);
                pointer + 4
            }
            3 => {
                let pos = mem[pointer + 1] as usize;
                mem[pos] = input;
                pointer + 2
            }
            4 => {
                output = read_mem(&mem, pointer, 1);
                println!("Output: {}", output);
                pointer + 2
            }
            5 => {
                if read_mem(&mem, pointer, 1) != 0 {
                    read_mem(&mem, pointer, 2) as usize
                } else {
                    pointer + 3
                }
            }
            6 => {
                if read_mem(&mem, pointer, 1) == 0 {
                    read_mem(&mem, pointer, 2) as usize
                } else {
                    pointer + 3
                }
            }
            7 => {
                let pos = mem[pointer + 3] as usize;
                mem[pos] = i64::from(read_mem(&mem, pointer, 1) < read_mem(&mem, pointer, 2));
                pointer + 4
            }
            8 => {
                let pos = mem[pointer + 3] as usize;
                mem[pos] = i64::from(read_mem(&mem, pointer, 1) == read_mem(&mem, pointer, 2));
                pointer + 4
            }
            99 => break output as i64,
            _ => unreachable!(),
        }
    }
}

fn run(input: &str, part_two: bool) -> i64 {
    let nums = input
        .split(',')
        .map(|num| num.trim().parse().unwrap())
        .collect();
    if part_two {
        execute(nums, 5)
    } else {
        execute(nums, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(run(include_str!("../../input/05.txt"), false), 13294380);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(include_str!("../../input/05.txt"), true), 11460760);
    }
}
