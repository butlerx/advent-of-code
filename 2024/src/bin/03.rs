use aoc_shared::time_execution;
static INPUT_TXT: &str = include_str!("../../input/03.txt");

fn main() {
    println!("🌟 --- Day 3 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

#[derive(Debug)]
enum Expression {
    Multiplication(u32, u32),
    Do,
    Dont,
}

impl std::str::FromStr for Expression {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some(expr) = input.strip_prefix("mul(").and_then(|rest| {
            let (nums, _) = rest.split_once(')')?;
            let (a, b) = nums.split_once(',')?;
            Some(Expression::Multiplication(a.parse().ok()?, b.parse().ok()?))
        }) {
            Ok(expr)
        } else {
            match input {
                s if s.starts_with("do()") => Ok(Expression::Do),
                s if s.starts_with("don't()") => Ok(Expression::Dont),
                _ => Err(()),
            }
        }
    }
}

impl Expression {
    fn evaluate(&self) -> u32 {
        match self {
            Expression::Multiplication(a, b) => a * b,
            _ => 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<Expression> {
    input
        .char_indices()
        .filter_map(|(i, c)| match c {
            'm' | 'd' => input[i..].parse().ok(),
            _ => None,
        })
        .collect()
}

fn part_1(input: &str) -> u32 {
    let mut sum = 0;

    for expr in parse_input(input) {
        if let Expression::Multiplication(_, _) = expr {
            sum += expr.evaluate();
        }
    }

    sum
}

fn part_2(input: &str) -> u32 {
    let mut sum = 0;
    let mut should_do = true;

    for expr in parse_input(input) {
        match expr {
            Expression::Multiplication(_, _) if should_do => {
                sum += expr.evaluate();
            }
            Expression::Do => should_do = true,
            Expression::Dont => should_do = false,
            Expression::Multiplication(_, _) => (),
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    static INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 161);
        assert_eq!(part_1(INPUT_TXT), 170_778_545);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_2), 48);
        assert_eq!(part_2(INPUT_TXT), 82_868_252);
    }
}
