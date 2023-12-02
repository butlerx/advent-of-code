static INPUT_TXT: &str = include_str!("../../input/11.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(i64),
    Multiply(i64),
    MultiplySelf,
}

impl std::str::FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ops: Vec<_> = s.split(' ').collect();
        Ok(if ops[4] == "*" && ops[5] == "old" {
            Self::MultiplySelf
        } else if ops[4] == "+" {
            Self::Add(ops[5].parse().unwrap())
        } else {
            Self::Multiply(ops[5].parse().unwrap())
        })
    }
}

impl Operation {
    fn execute(&self, x: i64) -> i64 {
        match self {
            Operation::Add(y) => x + y,
            Operation::Multiply(y) => x * y,
            Operation::MultiplySelf => x * x,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    on_true: usize,
    on_false: usize,
    throws: i64,
}

impl std::str::FromStr for Monkey {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = input.lines().map(str::trim).collect();
        let items: Vec<i64> = lines[1]
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        let operation = lines[2].parse()?;

        let test = lines[3].split(' ').nth(3).unwrap().parse::<i64>().unwrap();
        let on_true = lines[4]
            .split(' ')
            .nth(5)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let on_false = lines[5]
            .split(' ')
            .nth(5)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Ok(Self {
            items,
            operation,
            test,
            on_true,
            on_false,
            throws: 0,
        })
    }
}

enum Divisor {
    Three,
    Custom(i64),
}

fn calculate(mut monkeys: Vec<Monkey>, num_rounds: usize, inspect: Divisor) -> i64 {
    for _ in 0..num_rounds {
        for monkey in 0..monkeys.len() {
            let items = monkeys[monkey].items.clone();
            monkeys[monkey].throws += items.len() as i64;
            monkeys[monkey].items.clear();
            for i in items {
                let item = match inspect {
                    Divisor::Three => monkeys[monkey].operation.execute(i) / 3,
                    Divisor::Custom(d) => monkeys[monkey].operation.execute(i) % d,
                };
                let next_monkey = if item % monkeys[monkey].test == 0 {
                    monkeys[monkey].on_true
                } else {
                    monkeys[monkey].on_false
                };
                monkeys[next_monkey].items.push(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.throws.cmp(&a.throws));
    monkeys[0].throws * monkeys[1].throws
}

fn part_1(input: &str) -> i64 {
    let monkeys: Vec<Monkey> = input
        .trim()
        .split("\n\n")
        .map(|monkey| monkey.parse().unwrap())
        .collect();
    calculate(monkeys, 20, Divisor::Three)
}

fn part_2(input: &str) -> i64 {
    let monkeys: Vec<Monkey> = input
        .trim()
        .split("\n\n")
        .map(|monkey| monkey.parse().unwrap())
        .collect();
    let mod_product = monkeys.iter().map(|x| x.test).product::<i64>();
    calculate(monkeys, 10_000, Divisor::Custom(mod_product))
}

#[cfg(test)]
mod day_11_tests {
    use super::*;
    static INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 10605);
        assert_eq!(part_1(INPUT_TXT), 58056);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 2_713_310_158);
        assert_eq!(part_2(INPUT_TXT), 15_048_718_170);
    }
}
