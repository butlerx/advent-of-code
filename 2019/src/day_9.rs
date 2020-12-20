#[derive(Clone)]
struct Computer {
    base: i64,
    memory: Vec<i64>,
    pointer: usize,
    output: i64,
}

impl Computer {
    fn new(input: &str) -> Self {
        let mut mem: Vec<i64> = input
            .split(",")
            .map(|num| num.trim().parse().unwrap())
            .collect();
        mem.resize(10 * 1024, 0);
        Computer {
            memory: mem,
            base: 0,
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
            3..=4 | 9 => 2,
            5..=6 => 3,
            _ => 0,
        };
    }

    fn read(&mut self, mem: usize) -> &mut i64 {
        let pos = self.pointer + mem;
        let address = match self.memory[self.pointer] / 10i64.pow(mem as u32 + 1) % 10 {
            0 => self.memory[pos] as usize,
            1 => pos,
            2 => (self.base + self.memory[pos]) as usize,
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
                9 => self.base = self.base + *self.read(1),
                99 => break None,
                _ => (),
            }
            self.move_pointer(op);
        }
    }
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let mut comp = Computer::new(input);
    if part_two {
        comp.execute(vec![2]).unwrap()
    } else {
        comp.execute(vec![1]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: [&str; 3] = [
        "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
        "1102,34915192,34915192,7,4,7,99,0",
        "104,1125899906842624,99",
    ];

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT[0], false), 109);
        assert_eq!(run(INPUT[1], false).to_string().len(), 16);
        assert_eq!(run(INPUT[2], false), 1125899906842624);
        assert_eq!(run(include_str!("../input/day_9.txt"), false), 3454977209);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(include_str!("../input/day_9.txt"), true), 50120);
    }
}
