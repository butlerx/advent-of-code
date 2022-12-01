use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/01.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .map(|num| {
            let mut chars = num.chars();
            match chars.next() {
                Some('-') => -(chars.as_str().parse::<i64>().unwrap()),
                Some('+') => chars.as_str().parse::<i64>().unwrap(),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part_1(input: &str) -> i64 {
    parse_input(input).into_iter().sum()
}

fn part_2(input: &str) -> i64 {
    let nums = parse_input(input);
    let mut counter = 0;
    let mut visted: HashSet<i64> = vec![0i64].into_iter().collect();
    'outer: loop {
        for num in &nums {
            counter += num;
            if visted.contains(&counter) {
                break 'outer counter;
            }
            visted.insert(counter);
        }
    }
}

#[cfg(test)]
mod day_1_tests {
    use super::*;
    static INPUT_1: &str = "+1
+1
+1";
    static INPUT_2: &str = "+1
+1
-2";
    static INPUT_3: &str = "-1
-2
-3";

    static INPUT_4: &str = "+1
-1";
    static INPUT_5: &str = "+3
+3
+4
-2
-4";
    static INPUT_6: &str = "-6
+3
+8
+5
-6";
    static INPUT_7: &str = "+7
+7
-2
-7
-4";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT_1), 3);
        assert_eq!(part_1(INPUT_2), 0);
        assert_eq!(part_1(INPUT_3), -6);
        assert_eq!(part_1(INPUT_4), 0);
        assert_eq!(part_1(INPUT_5), 4);
        assert_eq!(part_1(INPUT_6), 4);
        assert_eq!(part_1(INPUT_7), 1);
        assert_eq!(part_1(include_str!("../../input/01.txt")), 590);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_1(INPUT_1), 3);
        assert_eq!(part_1(INPUT_2), 0);
        assert_eq!(part_1(INPUT_3), -6);
        assert_eq!(part_2(INPUT_4), 0);
        assert_eq!(part_2(INPUT_5), 10);
        assert_eq!(part_2(INPUT_6), 5);
        assert_eq!(part_2(INPUT_7), 14);
        assert_eq!(part_2(include_str!("../../input/01.txt")), 83445);
    }
}
