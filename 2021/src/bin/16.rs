use std::collections::VecDeque;
use std::iter::FromIterator;

fn main() {
    let input = include_str!("../../input/16.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(s: &str) -> VecDeque<char> {
    VecDeque::from_iter(
        s.chars()
            .filter_map(|c| match c {
                '0' => Some(['0', '0', '0', '0']),
                '1' => Some(['0', '0', '0', '1']),
                '2' => Some(['0', '0', '1', '0']),
                '3' => Some(['0', '0', '1', '1']),
                '4' => Some(['0', '1', '0', '0']),
                '5' => Some(['0', '1', '0', '1']),
                '6' => Some(['0', '1', '1', '0']),
                '7' => Some(['0', '1', '1', '1']),
                '8' => Some(['1', '0', '0', '0']),
                '9' => Some(['1', '0', '0', '1']),
                'A' => Some(['1', '0', '1', '0']),
                'B' => Some(['1', '0', '1', '1']),
                'C' => Some(['1', '1', '0', '0']),
                'D' => Some(['1', '1', '0', '1']),
                'E' => Some(['1', '1', '1', '0']),
                'F' => Some(['1', '1', '1', '1']),
                _ => None,
            })
            .flatten(),
    )
}

fn part_1(input: &str) -> i64 {
    let mut queue = parse_input(input);
    let (res, _) = run(&mut queue).unwrap();
    res
}

fn part_2(input: &str) -> i64 {
    let mut queue = parse_input(input);
    let (_, res) = run(&mut queue).unwrap();
    res
}

fn pop_string(queue: &mut VecDeque<char>, length: usize) -> String {
    String::from_iter(
        (0..length)
            .map(|_| queue.pop_front().unwrap())
            .collect::<Vec<char>>(),
    )
}

fn run(queue: &mut VecDeque<char>) -> Option<(i64, i64)> {
    let version_sum = i64::from_str_radix(&pop_string(queue, 3), 2).unwrap();

    match pop_string(queue, 3).as_ref() {
        "100" => Some((version_sum, solve_literal(queue))),
        "000" => {
            let (version, numbers) = solve_operator(queue);
            Some((version_sum + version, numbers.iter().sum()))
        }
        "001" => {
            let (version, numbers) = solve_operator(queue);
            Some((version_sum + version, numbers.iter().product()))
        }
        "010" => {
            let (version, numbers) = solve_operator(queue);
            Some((version_sum + version, *numbers.iter().min().unwrap()))
        }
        "011" => {
            let (version, numbers) = solve_operator(queue);
            Some((version_sum + version, *numbers.iter().max().unwrap()))
        }
        "101" => {
            let (version, numbers) = solve_operator(queue);
            Some((version_sum + version, i64::from(numbers[0] > numbers[1])))
        }
        "110" => {
            let (version, numbers) = solve_operator(queue);
            Some((version_sum + version, i64::from(numbers[0] < numbers[1])))
        }
        "111" => {
            let (version, numbers) = solve_operator(queue);
            Some((version_sum + version, i64::from(numbers[0] == numbers[1])))
        }
        _ => None,
    }
}

fn solve_literal(queue: &mut VecDeque<char>) -> i64 {
    let mut number_bits = vec![];

    loop {
        let group_header = queue.pop_front();
        for _ in 0..4 {
            number_bits.push(queue.pop_front().unwrap());
        }
        if group_header == Some('0') {
            break;
        }
    }

    i64::from_str_radix(&String::from_iter(number_bits), 2).unwrap()
}

fn solve_operator(queue: &mut VecDeque<char>) -> (i64, Vec<i64>) {
    let mut version_sum = 0;
    let mut values = vec![];

    match queue.pop_front() {
        Some('0') => {
            let length = usize::from_str_radix(&pop_string(queue, 15), 2).unwrap();
            let start = queue.len();
            loop {
                let (version, value) = run(queue).unwrap();
                version_sum += version;
                values.push(value);

                if start - queue.len() >= length {
                    break;
                }
            }
        }
        Some('1') => {
            for _ in 0..usize::from_str_radix(&pop_string(queue, 11), 2).unwrap() {
                let (version, value) = run(queue).unwrap();
                version_sum += version;
                values.push(value);
            }
        }
        _ => panic!("Missing or corrupted length type bit"),
    }
    (version_sum, values)
}

#[cfg(test)]
mod day_16_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("8A004A801A8002F478"), 16);
        assert_eq!(part_1("620080001611562C8802118E34"), 12);
        assert_eq!(part_1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part_1("A0016C880162017C3686B18A3D4780"), 31);
        assert_eq!(part_1(include_str!("../../input/16.txt")), 913);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("C200B40A82"), 3);
        assert_eq!(part_2("04005AC33890"), 54);
        assert_eq!(part_2("880086C3E88112"), 7);
        assert_eq!(part_2("CE00C43D881120"), 9);
        assert_eq!(part_2("D8005AC2A8F0"), 1);
        assert_eq!(part_2("F600BC2D8F"), 0);
        assert_eq!(part_2("9C005AC2F8F0"), 0);
        assert_eq!(part_2("9C0141080250320F1802104A08"), 1);
        assert_eq!(
            part_2(include_str!("../../input/16.txt")),
            1_510_977_819_698
        );
    }
}
