use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/10.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let nums = input
        .lines()
        .map(|num| num.trim().parse::<i64>().unwrap())
        .sorted();
    if part_two {
        let mut points = HashMap::new();
        let end = nums.clone().last().unwrap();
        points.insert(0, 1);
        for num in nums {
            let value = points.get(&(num - 1)).unwrap_or(&0)
                + points.get(&(num - 2)).unwrap_or(&0)
                + points.get(&(num - 3)).unwrap_or(&0);
            points.insert(num, value);
        }
        points[&end]
    } else {
        let diff = nums
            .tuple_windows()
            .fold((1, 0, 1), |diffs, (last, next)| match next - last {
                1 => (diffs.0 + 1, diffs.1, diffs.2),
                2 => (diffs.0, diffs.1 + 1, diffs.2),
                3 => (diffs.0, diffs.1, diffs.2 + 1),
                _ => diffs,
            });
        diff.0 * diff.2
    }
}

#[cfg(test)]
mod day_10_tests {
    use super::*;
    static INPUT: &str = "16
10
15
5
1
11
7
19
6
12
4";
    static INPUT_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, false), 35);
        assert_eq!(run(INPUT_2, false), 220);
        assert_eq!(run(include_str!("../../input/10.txt"), false), 2432);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 8);
        assert_eq!(run(INPUT_2, true), 19208);
        assert_eq!(
            run(include_str!("../../input/10.txt"), true),
            453551299002368
        );
    }
}
