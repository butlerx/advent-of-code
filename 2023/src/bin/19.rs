use std::collections::HashMap;

static INPUT_TXT: &str = include_str!("../../input/19.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    fn get(&self, p: char) -> u32 {
        match p {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!(),
        }
    }

    fn less_than(&self, p: char, n: u32) -> bool {
        self.get(p) < n
    }

    fn greater_than(&self, p: char, n: u32) -> bool {
        self.get(p) > n
    }
}

fn get_value(s: &str) -> u32 {
    s.split('=').nth(1).unwrap().parse().unwrap()
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let mut parts = s
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',');
        let x = get_value(parts.next().unwrap());
        let m = get_value(parts.next().unwrap());
        let a = get_value(parts.next().unwrap());
        let s = get_value(parts.next().unwrap());
        Self { x, m, a, s }
    }
}

type WorkFlows<'a> = HashMap<&'a str, (Vec<(char, bool, u32, &'a str)>, &'a str)>;

fn parse_workflows(input: &str) -> WorkFlows<'_> {
    input
        .lines()
        .map(|l| {
            let (name, rest) = l.split_once('{').unwrap();
            let (rest, label) = rest
                .strip_suffix('}')
                .unwrap()
                .split_at(rest.rfind(',').unwrap());
            let rules = rest
                .split(',')
                .map(|rule| {
                    let (rest, label) = rule.split_once(':').unwrap();
                    let lt = rest.contains('<');
                    let (name, n) = rest.split_once(if lt { '<' } else { '>' }).unwrap();
                    (
                        name.chars().next().unwrap(),
                        lt,
                        n.parse::<u32>().unwrap(),
                        label,
                    )
                })
                .collect::<Vec<_>>();
            (name, (rules, &label[1..]))
        })
        .collect::<HashMap<_, _>>()
}

fn part_1(input: &str) -> u32 {
    let (workflows_str, parts_str) = input.trim().split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows_str);
    parts_str
        .lines()
        .map(Part::from)
        .filter(|part| {
            let mut curr = "in";
            loop {
                if curr == "A" || curr == "R" {
                    break curr == "A";
                }
                let (rules, label) = &workflows[curr];
                curr = rules
                    .iter()
                    .find(|&&(p, lt, n, _)| {
                        if lt {
                            part.less_than(p, n)
                        } else {
                            part.greater_than(p, n)
                        }
                    })
                    .map_or(label, |&(_, _, _, l)| l);
            }
        })
        .map(|part| part.sum())
        .sum()
}

fn possible_accepted(workflows: &WorkFlows<'_>, curr: &str, mut ranges: [Vec<usize>; 4]) -> usize {
    if curr == "A" {
        return ranges.iter().map(std::vec::Vec::len).product();
    }
    if curr == "R" {
        return 0;
    }
    let (rules, rule_label) = &workflows[curr];
    rules.iter().fold(0, |ans, &(p, lt, n, label)| {
        let i = match p {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => unreachable!(),
        };
        let mut newranges = ranges.clone();
        (newranges[i], ranges[i]) = ranges[i].iter().partition(|&&val| {
            if lt {
                val < n as usize
            } else {
                val > n as usize
            }
        });
        ans + possible_accepted(workflows, label, newranges)
    }) + possible_accepted(workflows, rule_label, ranges)
}

fn part_2(input: &str) -> usize {
    let (workflows_str, _) = input.trim().split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows_str);
    possible_accepted(
        &workflows,
        "in",
        std::array::from_fn(|_| (1..=4000).collect::<Vec<_>>()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 19114);
        assert_eq!(part_1(INPUT_TXT), 386_787);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 167_409_079_868_000);
        assert_eq!(part_2(INPUT_TXT), 131_029_523_269_531);
    }
}
