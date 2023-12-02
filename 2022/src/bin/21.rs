use std::collections::HashMap;
static INPUT_TXT: &str = include_str!("../../input/21.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

type Monkeys<'a> = HashMap<&'a str, Value<'a>>;
enum Op {
    Add,
    Minus,
    Multiply,
    Divide,
}

enum Value<'a> {
    Number(i64),
    Operation(&'a str, Op, &'a str),
}

impl<'a> std::convert::TryFrom<&'a str> for Value<'a> {
    type Error = ();

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(match s.parse::<i64>() {
            Ok(num) => Value::Number(num),
            Err(_) => {
                let value: Vec<_> = s.split_whitespace().collect();
                let op = match value[1] {
                    "+" => Op::Add,
                    "-" => Op::Minus,
                    "*" => Op::Multiply,
                    "/" => Op::Divide,
                    _ => unreachable!(),
                };
                Value::Operation(value[0], op, value[2])
            }
        })
    }
}

impl<'a> Value<'a> {
    fn get(&self, monkeys: &Monkeys) -> i64 {
        match self {
            Value::Number(value) => *value,
            Value::Operation(left, op, right) => {
                let left = monkeys[left].get(monkeys);
                let right = monkeys[right].get(monkeys);
                match op {
                    Op::Add => left + right,
                    Op::Minus => left - right,
                    Op::Multiply => left * right,
                    Op::Divide => left / right,
                }
            }
        }
    }

    fn get_not_human(&self, monkeys: &Monkeys) -> Option<i64> {
        match self {
            Value::Number(value) => Some(*value),
            Value::Operation(left, op, right) => {
                if *left == "humn" || *right == "humn" {
                    None
                } else {
                    match (
                        monkeys[left].get_not_human(monkeys),
                        monkeys[right].get_not_human(monkeys),
                    ) {
                        (Some(left), Some(right)) => Some(match op {
                            Op::Add => left + right,
                            Op::Minus => left - right,
                            Op::Multiply => left * right,
                            Op::Divide => left / right,
                        }),
                        _ => None,
                    }
                }
            }
        }
    }
}

fn part_1(input: &str) -> i64 {
    let monkeys: Monkeys = input
        .trim()
        .lines()
        .map(|l| {
            let (monkey, eq) = l.split_once(": ").unwrap();
            (monkey, Value::try_from(eq).unwrap())
        })
        .collect();
    monkeys["root"].get(&monkeys)
}

fn part_2(input: &str) -> i64 {
    let monkeys: Monkeys = input
        .trim()
        .lines()
        .map(|l| {
            let (monkey, eq) = l.split_once(": ").unwrap();
            (monkey, Value::try_from(eq).unwrap())
        })
        .collect();

    let Value::Operation(left, _, right) = monkeys["root"] else {
        unreachable!()
    };
    let (mut target_value, container) = if let Some(value) = monkeys[left].get_not_human(&monkeys) {
        (value, right)
    } else {
        (monkeys[right].get(&monkeys), left)
    };
    let mut container = container;
    loop {
        let value = &monkeys[container];
        if match value {
            Value::Number(_) => false,
            Value::Operation(left, _, right) => *left == "humn" || *right == "humn",
        } {
            if let Value::Operation(left, op, right) = value {
                break if *left == "humn" {
                    let value = monkeys[right].get(&monkeys);
                    match op {
                        Op::Add => target_value - value,
                        Op::Minus => target_value + value,
                        Op::Multiply => target_value / value,
                        Op::Divide => value * target_value,
                    }
                } else {
                    let value = monkeys[left].get(&monkeys);
                    match op {
                        Op::Add => target_value - value,
                        Op::Minus => value - target_value,
                        Op::Multiply => target_value / value,
                        Op::Divide => value / target_value,
                    }
                };
            }
        }
        match value {
            Value::Number(_) => unreachable!(),
            Value::Operation(left, op, right) => {
                if let Some(value) = monkeys[left].get_not_human(&monkeys) {
                    target_value = match op {
                        Op::Add => target_value - value,
                        Op::Minus => value - target_value,
                        Op::Multiply => target_value / value,
                        Op::Divide => value / target_value,
                    };
                    container = *right;
                } else {
                    let value = monkeys[right].get(&monkeys);
                    target_value = match op {
                        Op::Add => target_value - value,
                        Op::Minus => target_value + value,
                        Op::Multiply => target_value / value,
                        Op::Divide => value * target_value,
                    };
                    container = *left;
                }
            }
        }
    }
}

#[cfg(test)]
mod day_21_tests {
    use super::*;
    static INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 152);
        assert_eq!(part_1(INPUT_TXT), 22_382_838_633_806);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 301);
        assert_eq!(part_2(INPUT_TXT), 3_099_532_691_300);
    }
}
