use itertools::Itertools;

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

fn execute(memory: Vec<i64>, input: Vec<i64>) -> i64 {
    let mut mem = memory.clone();
    let mut pointer = 0;
    let mut output = 0;
    let mut input = input.iter();
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
                mem[pos] = *input.next().unwrap();
                pointer + 2
            }
            4 => {
                output = read_mem(&mem, pointer, 1);
                // println!("Output: {}", output);
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
                mem[pos] = if read_mem(&mem, pointer, 1) < read_mem(&mem, pointer, 2) {
                    1
                } else {
                    0
                };
                pointer + 4
            }
            8 => {
                let pos = mem[pointer + 3] as usize;
                mem[pos] = if read_mem(&mem, pointer, 1) == read_mem(&mem, pointer, 2) {
                    1
                } else {
                    0
                };
                pointer + 4
            }
            99 => break output as i64,
            _ => unreachable!(),
        }
    }
}

fn applify(memory: &Vec<i64>, phases: &Vec<i64>) -> i64 {
    println!("{:?}", phases);
    phases.iter().fold(0, |input, phase| {
        execute(memory.clone(), vec![*phase, input])
    })
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let code: Vec<i64> = input
        .split(",")
        .map(|num| num.trim().parse().unwrap())
        .collect();
    if part_two {
        (5..10)
            .permutations(5)
            .map(|args| applify(&code, &args))
            .max()
            .unwrap_or(0)
    } else {
        (0..5)
            .permutations(5)
            .map(|args| applify(&code, &args))
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_1: &str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    static INPUT_2: &str = "3,23,3,24,1002,24,10,24,1002,23,-1,23,
101,5,23,23,1,24,23,23,4,23,99,0,0";
    static INPUT_3: &str = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    static INPUT_4: &str = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    static INPUT_5: &str = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT_1, false) == 43210);
        assert!(run(INPUT_2, false) == 54321);
        assert!(run(INPUT_3, false) == 65210);
        let results = run(include_str!("../input/day_7.txt"), false);
        println!("{}", results);
        assert!(results == 65464);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT_4, true) == 139629729);
        assert!(run(INPUT_5, true) == 18216);
        let results = run(include_str!("../input/day_7.txt"), true);
        println!("{}", results);
        assert!(results == 0);
    }
}
