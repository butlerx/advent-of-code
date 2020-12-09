use itertools::Itertools;

#[derive(Clone)]
struct Computer {
    memory: Vec<i64>,
    pointer: usize,
    output: i64,
}

impl Computer {
    fn new(input: &str) -> Self {
        Computer {
            memory: input
                .split(",")
                .map(|num| num.trim().parse().unwrap())
                .collect(),
            pointer: 0,
            output: 0,
        }
    }

    // parse opscode, returning the code and if the args are in position mode or imediate
    // if an arg is true its in  imediate mode
    fn parse_opcode(&self) -> usize {
        self.memory[self.pointer] as usize % 100
    }

    fn read(&self, mem: usize) -> i64 {
        if (self.memory[self.pointer] / 10i64.pow(mem as u32 + 1)) % 10 == 1 {
            self.memory[self.pointer + mem]
        } else {
            self.memory[self.memory[self.pointer + mem] as usize]
        }
    }

    fn read_pos(&self, pos: usize) -> usize {
        self.memory[self.pointer + pos] as usize
    }

    fn execute(&mut self, input: Vec<i64>) -> Option<i64> {
        let mut input = input.iter();
        loop {
            match self.parse_opcode() {
                1 => {
                    let pos = self.read_pos(3);
                    self.memory[pos] = self.read(1) + self.read(2);
                    self.pointer += 4;
                }
                2 => {
                    let pos = self.read_pos(3);
                    self.memory[pos] = self.read(1) * self.read(2);
                    self.pointer += 4;
                }
                3 => {
                    let pos = self.read_pos(1);
                    self.memory[pos] = *input.next().unwrap();
                    self.pointer += 2;
                }
                4 => {
                    self.output = self.read(1);
                    // println!("Output: {}", output);
                    self.pointer += 2;
                    break Some(self.output);
                }
                5 => {
                    if self.read(1) != 0 {
                        self.pointer = self.read(2) as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                6 => {
                    if self.read(1) == 0 {
                        self.pointer = self.read(2) as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                7 => {
                    let pos = self.read_pos(3);
                    self.memory[pos] = if self.read(1) < self.read(2) { 1 } else { 0 };
                    self.pointer += 4;
                }
                8 => {
                    let pos = self.read_pos(3);
                    self.memory[pos] = if self.read(1) == self.read(2) { 1 } else { 0 };
                    self.pointer += 4;
                }
                99 => break None,
                _ => unreachable!(),
            }
        }
    }
}

fn feedback(comp: &Computer, phases: &Vec<i64>) -> i64 {
    let mut amplifiers: Vec<_> = phases.iter().map(|_| comp.clone()).collect();

    let init = phases
        .iter()
        .zip(&mut amplifiers)
        .fold(0, |input, (p, a)| a.execute(vec![*p, input]).unwrap());

    (0..amplifiers.len())
        .cycle()
        .try_fold(init, |input, i| amplifiers[i].execute(vec![input]));

    amplifiers.last().unwrap().output
}

fn applify(comp: &Computer, phases: &Vec<i64>) -> i64 {
    phases.iter().fold(0, |input, phase| {
        comp.clone().execute(vec![*phase, input]).unwrap()
    })
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let comp = Computer::new(input);
    if part_two {
        (5..10)
            .permutations(5)
            .map(|args| feedback(&comp, &args))
            .max()
            .unwrap_or(0)
    } else {
        (0..5)
            .permutations(5)
            .map(|args| applify(&comp, &args))
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
        assert!(results == 1518124);
    }
}
