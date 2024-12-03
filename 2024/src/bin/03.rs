static INPUT_TXT: &str = include_str!("../../input/03.txt");

fn main() {
    println!("🌟 --- Day 2 Results --- 🌟");
    println!("📌 Part 1: {}", part_1(INPUT_TXT));
    println!("📌 Part 2: {}", part_2(INPUT_TXT));
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
        match expr {
            Expression::Multiplication(_, _) => {
                sum += expr.evaluate();
            }
            _ => continue,
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
            _ => continue,
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
        assert_eq!(part_1(INPUT_TXT), 170778545);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_2), 48);
        assert_eq!(part_2(INPUT_TXT), 82868252);
    }
}
