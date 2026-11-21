use serde_json::Value;
use std::cmp::Ordering;
static INPUT_TXT: &str = include_str!("../../input/13.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn compare(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(lhs), Value::Number(rhs)) => lhs
            .as_i64()
            .expect("valid puzzle input")
            .cmp(&rhs.as_i64().expect("valid puzzle input")),
        (Value::Array(lhs), Value::Array(rhs)) => (0..std::cmp::max(lhs.len(), rhs.len()))
            .find_map(|idx| match (lhs.get(idx), rhs.get(idx)) {
                (None, _) => Some(Ordering::Less),
                (_, None) => Some(Ordering::Greater),
                (Some(lhs_item), Some(rhs_item)) => match compare(lhs_item, rhs_item) {
                    Ordering::Equal => None,
                    order => Some(order),
                },
            })
            .unwrap_or(Ordering::Equal),
        (Value::Array(_), Value::Number(_)) => compare(left, &Value::Array(vec![right.clone()])),
        (Value::Number(_), Value::Array(_)) => compare(&Value::Array(vec![left.clone()]), right),
        _ => unreachable!(),
    }
}
fn part_1(input: &str) -> i64 {
    input
        .trim()
        .split("\n\n")
        .enumerate()
        .filter(|(_, lines)| {
            let packets = lines
                .lines()
                .map(|l| serde_json::from_str::<Value>(l).expect("valid puzzle input"))
                .collect::<Vec<_>>();
            compare(&packets[0], &packets[1]) != Ordering::Greater
        })
        .map(|(i, _)| i64::try_from(i).expect("cast within range") + 1)
        .sum()
}

fn part_2(input: &str) -> i64 {
    let mut packets = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).expect("valid puzzle input"))
        .collect::<Vec<_>>();
    let divider_packets = [
        serde_json::from_str::<Value>("[[2]]").expect("valid puzzle input"),
        serde_json::from_str::<Value>("[[6]]").expect("valid puzzle input"),
    ];
    packets.extend(divider_packets.iter().cloned());
    packets.sort_by(compare);
    packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| divider_packets.contains(packet))
        .map(|(i, _)| i64::try_from(i).expect("cast within range") + 1)
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
