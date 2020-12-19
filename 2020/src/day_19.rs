use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Seq(Vec<usize>),
    Alt(Vec<usize>, Vec<usize>),
}

fn parse_rules(rules: &str) -> HashMap<usize, Rule> {
    rules
        .lines()
        .map(|line| {
            let (key, value) = line.split(": ").collect_tuple().unwrap();
            let rule = if value.contains('"') {
                Rule::Char(value.chars().nth(1).unwrap())
            } else if value.contains('|') {
                let (seq_1, seq_2) = value
                    .split(" | ")
                    .map(|seg| seg.split(" ").map(|n| n.parse().unwrap()).collect())
                    .collect_tuple()
                    .unwrap();
                Rule::Alt(seq_1, seq_2)
            } else {
                Rule::Seq(value.split(" ").map(|n| n.parse().unwrap()).collect())
            };
            (key.parse::<usize>().unwrap(), rule)
        })
        .collect()
}

fn validate_msg(rules: &HashMap<usize, Rule>, msg: &str, mut queue: VecDeque<usize>) -> bool {
    if queue.is_empty() || msg.is_empty() {
        return queue.is_empty() && msg.is_empty();
    }

    let validate_seq = |seq: &Vec<usize>, mut q| {
        let mut buf = seq.iter().map(|&n| n).collect::<VecDeque<usize>>();
        buf.append(&mut q);
        validate_msg(rules, msg, buf)
    };

    match rules.get(&queue.pop_front().unwrap()) {
        Some(Rule::Char(c)) => match msg.chars().next() {
            Some(ch) if &ch == c => validate_msg(rules, &msg[1..], queue),
            _ => false,
        },
        Some(Rule::Seq(seq)) => validate_seq(&seq, queue),
        Some(Rule::Alt(seq_1, seq_2)) => {
            validate_seq(&seq_1, queue.clone()) || validate_seq(&seq_2, queue.clone())
        }
        _ => unreachable!(),
    }
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let (rules, messages): (&str, &str) = input.split("\n\n").collect_tuple().unwrap();
    let mut schema = parse_rules(rules);
    if part_two {
        schema.insert(8, Rule::Alt(vec![42], vec![42, 8]));
        schema.insert(11, Rule::Alt(vec![42, 31], vec![42, 11, 31]));
    }
    messages
        .lines()
        .filter(|line| validate_msg(&schema, line, vec![0].into_iter().collect()))
        .count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
    static INPUT_1: &str = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, false), 2);
        assert_eq!(run(INPUT_1, false), 3);
        assert_eq!(run(include_str!("../input/day_19.txt"), false), 195);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 2);
        assert_eq!(run(INPUT_1, true), 12);
        assert_eq!(run(include_str!("../input/day_19.txt"), true), 309);
    }
}
