static INPUT_TXT: &str = include_str!("../../input/02.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn part_1(input: &str) -> i64 {
    input.trim().lines().fold(0i64, |score, line| {
        let (move_a, move_b) = line.split_once(' ').unwrap();
        (match move_a {
            "A" => match move_b {
                "X" => 1 + 3,
                "Y" => 2 + 6,
                "Z" => 3,
                _ => unreachable!(),
            },
            "B" => match move_b {
                "X" => 1,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                _ => unreachable!(),
            },
            "C" => match move_b {
                "X" => 1 + 6,
                "Y" => 2,
                "Z" => 3 + 3,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }) + score
    })
}

fn part_2(input: &str) -> i64 {
    input.trim().lines().fold(0i64, |score, line| {
        let (move_a, move_b) = line.split_once(' ').unwrap();
        (match move_a {
            "A" => match move_b {
                "X" => 3,
                "Y" => 3 + 1,
                "Z" => 6 + 2,
                _ => unreachable!(),
            },
            "B" => match move_b {
                "X" => 1,
                "Y" => 3 + 2,
                "Z" => 6 + 3,
                _ => unreachable!(),
            },
            "C" => match move_b {
                "X" => 2,
                "Y" => 3 + 3,
                "Z" => 6 + 1,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }) + score
    })
}

#[cfg(test)]
mod day_2_tests {
    use super::*;
    static INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 15);
        assert_eq!(part_1(INPUT_TXT), 9651);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 12);
        assert_eq!(part_2(INPUT_TXT), 10560);
    }
}
