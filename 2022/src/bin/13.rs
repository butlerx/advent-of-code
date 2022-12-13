use serde_json::Value;
use std::cmp::Ordering;
static INPUT_TXT: &str = include_str!("../../input/13.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_i64().unwrap().cmp(&y.as_i64().unwrap()),
        (Value::Array(a), Value::Array(b)) => (0..std::cmp::max(a.len(), b.len()))
            .find_map(|i| match (a.get(i), b.get(i)) {
                (None, _) => Some(Ordering::Less),
                (_, None) => Some(Ordering::Greater),
                (Some(x), Some(y)) => match compare(x, y) {
                    Ordering::Equal => None,
                    c => Some(c),
                },
            })
            .unwrap_or(Ordering::Equal),
        (Value::Array(_), Value::Number(_)) => compare(a, &Value::Array(vec![b.clone()])),
        (Value::Number(_), Value::Array(_)) => compare(&Value::Array(vec![a.clone()]), b),
        _ => unreachable!(),
    }
}
fn part_1(input: &str) -> i64 {
    input
        .trim()
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, lines)| {
            let packets = lines
                .lines()
                .map(|l| serde_json::from_str::<Value>(l).unwrap())
                .collect::<Vec<_>>();
            if compare(&packets[0], &packets[1]) != Ordering::Greater {
                Some(i as i64 + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: &str) -> i64 {
    let mut packets = input
        .trim()
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(serde_json::from_str::<Value>(line).unwrap())
            }
        })
        .collect::<Vec<_>>();
    let divider_packets = [
        serde_json::from_str::<Value>("[[2]]").unwrap(),
        serde_json::from_str::<Value>("[[6]]").unwrap(),
    ];
    packets.extend(divider_packets.iter().cloned());
    packets.sort_by(compare);
    packets
        .iter()
        .enumerate()
        .filter_map(|(i, packet)| {
            if divider_packets.contains(packet) {
                Some(i as i64 + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod day_13_tests {
    use super::*;
    static INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 13);
        assert_eq!(part_1(INPUT_TXT), 6369);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 140);
        assert_eq!(part_2(INPUT_TXT), 25800);
    }
}
