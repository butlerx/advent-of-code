fn main() {
    let mut comp = Computer::new(include_str!("../../input/09.txt"));
    println!("Part 1: {}", comp.execute(vec![1]).unwrap());
    println!("Part 2: {}", comp.execute(vec![2]).unwrap());
}

#[derive(Clone)]
struct Computer {
    base: i64,
    memory: Vec<i64>,
    pointer: usize,
}

impl Computer {
    fn new(input: &str) -> Self {
        let mut mem: Vec<i64> = input
            .split(',')
            .map(|num| num.trim().parse().unwrap())
            .collect();
        mem.resize(10 * 1024, 0);
        Computer {
            memory: mem,
            base: 0,
            pointer: 0,
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
                9 => self.base += *self.read(1),
                99 => break None,
                _ => (),
            }
            self.move_pointer(op);
        }
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
    fn test_small_input() {
        let mut c0 = Computer::new(INPUT[0]);
        assert_eq!(c0.execute(vec![1]).unwrap(), 109);
        let mut c1 = Computer::new(INPUT[1]);
        assert_eq!(c1.execute(vec![1]).unwrap().to_string().len(), 16);
        let mut c2 = Computer::new(INPUT[2]);
        assert_eq!(c2.execute(vec![1]).unwrap(), 1125899906842624);
    }

    #[test]
    fn test_large_input() {
        let mut comp = Computer::new(include_str!("../../input/09.txt"));
        assert_eq!(comp.execute(vec![1]).unwrap(), 3454977209);
        assert_eq!(comp.execute(vec![2]).unwrap(), 50120);
    }
}
