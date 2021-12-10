fn main() {
    let input = include_str!("../../input/10.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            let mut expected: Vec<char> = vec![];
            for c in line.chars() {
                match c {
                    '(' => expected.push(')'),
                    '[' => expected.push(']'),
                    '{' => expected.push('}'),
                    '<' => expected.push('>'),
                    _ => {
                        if Some(c) != expected.pop() {
                            return match c {
                                ')' => Some(3),
                                ']' => Some(57),
                                '}' => Some(1197),
                                '>' => Some(25137),
                                _ => None,
                            };
                        }
                    }
                };
            }
            None
        })
        .sum()
}

fn part_2(input: &str) -> i64 {
    let mut scores: Vec<i64> = input
        .lines()
        .filter_map(|line| {
            let mut expected = Vec::new();
            for c in line.chars() {
                match c {
                    '(' => expected.push(')'),
                    '[' => expected.push(']'),
                    '{' => expected.push('}'),
                    '<' => expected.push('>'),
                    _ => {
                        if Some(c) != expected.pop() {
                            return None;
                        }
                    }
                }
            }
            Some(
                expected
                    .into_iter()
                    .rev()
                    .filter_map(|c| match c {
                        ')' => Some(1),
                        ']' => Some(2),
                        '}' => Some(3),
                        '>' => Some(4),
                        _ => None,
                    })
                    .fold(0, |acc, x| (5 * acc) + x),
            )
        })
        .collect();
    scores.sort_unstable();
    *scores.get(scores.len() / 2).unwrap()
}

#[cfg(test)]
mod day_10_tests {
    use super::*;
    static INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 26397);
        assert_eq!(part_1(include_str!("../../input/10.txt")), 462693);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 288957);
        assert_eq!(part_2(include_str!("../../input/10.txt")), 3094671161);
    }
}
