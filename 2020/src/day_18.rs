fn find_matching_par(chars: &[char]) -> usize {
    let mut d = 0;
    let mut pointer = 0;
    loop {
        match chars[pointer] {
            '(' => d += 1,
            ')' => d -= 1,
            _ => {}
        }
        if d == 0 {
            break pointer;
        }
        pointer += 1;
    }
}

fn oper(op: char, x: i64, y: i64) -> i64 {
    match op {
        '+' => x + y,
        '*' => x * y,
        _ => unreachable!(),
    }
}

fn eval(chars: Vec<char>) -> i64 {
    let (mut res, mut i, mut op) = (0, 0, '+');
    loop {
        if i >= chars.len() {
            break res;
        }
        match chars[i] {
            '+' => op = '+',
            '*' => op = '*',
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

fn new_math(input: &str) -> i64 {
    eval(input.chars().filter(|c| c != &' ').collect())
}

fn gat_value(chars: &Vec<char>, i: usize) -> (i64, usize) {
    if chars[i] == '(' {
        let j = find_matching_par(&chars[i..]);
        (eval_adv(chars[i + 1..i + j].to_vec()), j)
    } else {
        (chars[i].to_digit(10).unwrap() as i64, 0)
    }
}

fn eval_adv(chars: Vec<char>) -> i64 {
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

fn new_math_adv(input: &str) -> i64 {
    eval_adv(input.chars().filter(|c| c != &' ').collect())
}

pub fn run(input: &str, part_two: bool) -> i64 {
    input
        .lines()
        .map(if part_two { new_math_adv } else { new_math })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "1 + (2 * 3) + (4 * (5 + 6))";
    static INPUT_1: &str = "2 * 3 + (4 * 5)";
    static INPUT_2: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    static INPUT_3: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    static INPUT_4: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_new_math() {
        assert_eq!(new_math(INPUT), 51);
        assert_eq!(new_math(INPUT_1), 26);
        assert_eq!(new_math(INPUT_2), 437);
        assert_eq!(new_math(INPUT_3), 12240);
        assert_eq!(new_math(INPUT_4), 13632);
    }

    #[test]
    fn test_new_math_adv() {
        assert_eq!(new_math_adv(INPUT), 51);
        assert_eq!(new_math_adv(INPUT_1), 46);
        assert_eq!(new_math_adv(INPUT_2), 1445);
        assert_eq!(new_math_adv(INPUT_3), 669060);
        assert_eq!(new_math_adv(INPUT_4), 23340);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            run(include_str!("../input/day_18.txt"), false),
            25190263477788
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            run(include_str!("../input/day_18.txt"), true),
            297139939002972
        );
    }
}
