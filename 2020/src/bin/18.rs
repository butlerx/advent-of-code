fn main() {
    let input = include_str!("../../input/18.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn find_matching_par(chars: &[char]) -> usize {
    let (mut nested, mut pointer) = (0, 0);
    loop {
        match chars[pointer] {
            '(' => nested += 1,
            ')' => nested -= 1,
            _ => {}
        }
        match nested {
            0 => break pointer,
            _ => pointer += 1,
        }
    }
}

mod part_1 {
    use super::find_matching_par;

    fn oper(op: char, x: i64, y: i64) -> i64 {
        match op {
            '+' => x + y,
            '*' => x * y,
            _ => unreachable!(),
        }
    }

    pub fn eval(chars: Vec<char>) -> i64 {
        let (mut res, mut i, mut op) = (0, 0, '+');
        loop {
            if i >= chars.len() {
                break res;
            }
            match chars[i] {
                '+' | '*' => op = chars[i],
                '0'..='9' => res = oper(op, res, chars[i].to_digit(10).unwrap() as i64),
                '(' => {
                    let end = i + find_matching_par(&chars[i..]);
                    res = oper(op, res, eval(chars[i + 1..end].to_vec()));
                    i = end;
                }
                _ => unreachable!(),
            }
            i += 1;
        }
    }
}

mod part_2 {
    use super::find_matching_par;

    fn gat_value(chars: &[char], i: usize) -> (i64, usize) {
        match chars[i] {
            '(' => {
                let j = find_matching_par(&chars[i..]);
                (eval(chars[i + 1..i + j].to_vec()), j)
            }
            _ => (chars[i].to_digit(10).unwrap() as i64, 0),
        }
    }

    pub fn eval(chars: Vec<char>) -> i64 {
        let (mut res, mut i) = (1, 0);
        loop {
            if i >= chars.len() {
                break res;
            }
            let (mut val, step) = gat_value(&chars, i);
            i += step;
            while let Some('+') = chars.get(i + 1) {
                let (tmp, step) = gat_value(&chars, i + 2);
                val += tmp;
                i += step + 2;
            }
            res *= val;
            i += 2;
        }
    }
}

fn run(input: &str, part_two: bool) -> i64 {
    input
        .lines()
        .map(|line| {
            let sums = line.chars().filter(|c| c != &' ').collect();
            if part_two {
                part_2::eval(sums)
            } else {
                part_1::eval(sums)
            }
        })
        .sum()
}

#[cfg(test)]
mod day_18_tests {
    use super::*;
    static INPUT: [&str; 5] = [
        "1 + (2 * 3) + (4 * (5 + 6))",
        "2 * 3 + (4 * 5)",
        "5 + (8 * 3 + 9 + 3 * 4 * 3)",
        "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
        "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
    ];

    #[test]
    fn test_eval() {
        assert_eq!(run(INPUT[0], false), 51);
        assert_eq!(run(INPUT[1], false), 26);
        assert_eq!(run(INPUT[2], false), 437);
        assert_eq!(run(INPUT[3], false), 12240);
        assert_eq!(run(INPUT[4], false), 13632);
    }

    #[test]
    fn test_eval_adv() {
        assert_eq!(run(INPUT[0], true), 51);
        assert_eq!(run(INPUT[1], true), 46);
        assert_eq!(run(INPUT[2], true), 1445);
        assert_eq!(run(INPUT[3], true), 669060);
        assert_eq!(run(INPUT[4], true), 23340);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            run(include_str!("../../input/18.txt"), false),
            25190263477788
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            run(include_str!("../../input/18.txt"), true),
            297139939002972
        );
    }
}
