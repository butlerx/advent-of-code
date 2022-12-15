static INPUT_TXT: &[u8] = include_bytes!("../../input/05.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &[u8]) -> Vec<u8> {
    input.iter().fold(vec![], |mut result, a| {
        if result.last().filter(|&b| a ^ b == 0x20).is_some() {
            result.pop();
        } else {
            result.push(*a);
        }
        result
    })
}
fn part_1(input: &[u8]) -> usize {
    parse_input(input).len()
}

fn part_2(input: &[u8]) -> usize {
    (b'a'..=b'x')
        .map(|unit| {
            input
                .iter()
                .filter(|&&b| b | 0x60 != unit)
                .copied()
                .collect::<Vec<_>>()
        })
        .map(|polymer| parse_input(&polymer).len())
        .min()
        .unwrap()
}

#[cfg(test)]
mod day_5_tests {
    use super::*;
    static INPUT: &[u8] = b"dabAcCaCBAcCcaDA";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 10);
        assert_eq!(part_1(INPUT_TXT), 10368);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 4);
        assert_eq!(part_2(INPUT_TXT), 4122);
    }
}
