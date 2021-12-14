use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/07.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

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
                .split(',')
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

    fn move_pointer(&mut self, op: usize) {
        self.pointer += match op {
            1..=2 | 7..=8 => 4,
            3..=4 => 2,
            5..=6 => 3,
            _ => 0,
        };
    }

    fn read(&mut self, mem: usize) -> &mut i64 {
        let pos = self.pointer + mem;
        let address = match self.memory[self.pointer] / 10i64.pow(mem as u32 + 1) % 10 {
            0 => self.memory[pos] as usize,
            1 => pos,
            _ => unreachable!(),
        };
        &mut self.memory[address]
    }

    fn execute(&mut self, input: Vec<i64>) -> Option<i64> {
        let mut input = input.iter();
        loop {
            let op = self.parse_opcode();
            match op {
                1 => *self.read(3) = *self.read(1) + *self.read(2),
                2 => *self.read(3) = *self.read(1) * *self.read(2),
                3 => *self.read(1) = *input.next().unwrap(),
                4 => break Some(*self.read(1)),
                5 if *self.read(1) != 0 => {
                    self.pointer = *self.read(2) as usize;
                    continue;
                }
                6 if *self.read(1) == 0 => {
                    self.pointer = *self.read(2) as usize;
                    continue;
                }
                7 => *self.read(3) = if *self.read(1) < *self.read(2) { 1 } else { 0 },
                8 => *self.read(3) = if *self.read(1) == *self.read(2) { 1 } else { 0 },
                99 => break None,
                _ => (),
            }
            self.move_pointer(op);
        }
    }
}

fn feedback(comp: &Computer, phases: &[i64]) -> i64 {
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

fn applify(comp: &Computer, phases: &[i64]) -> i64 {
    phases.iter().fold(0, |input, phase| {
        comp.clone().execute(vec![*phase, input]).unwrap()
    })
}

fn run(input: &str, part_two: bool) -> i64 {
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
        assert_eq!(run(INPUT_1, false), 43210);
        assert_eq!(run(INPUT_2, false), 54321);
        assert_eq!(run(INPUT_3, false), 65210);
        assert_eq!(run(include_str!("../../input/07.txt"), false), 65464);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT_4, true), 139629729);
        assert_eq!(run(INPUT_5, true), 18216);
        assert_eq!(run(include_str!("../../input/07.txt"), true), 1518124);
    }
}
