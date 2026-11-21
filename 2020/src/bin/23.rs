use std::collections::HashSet;
use std::convert::TryFrom;

fn main() {
    let input = include_str!("../../input/23.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn move_cups(input: &[i64], rounds: usize) -> Vec<i64> {
    let (n, mut pointers) = if rounds <= 100 {
        (
            i64::try_from(input.len()).expect("cast within range"),
            [0i64; 10].to_vec(),
        )
    } else {
        let n = 1_000_000;
        let mut pointers: Vec<i64> = (1..(n + 2)).collect();
        pointers[usize::try_from(n).expect("cast within range")] = input[0];
        (n, pointers)
    };
    for w in input.windows(2) {
        pointers[usize::try_from(w[0]).expect("cast within range")] = w[1];
    }
    pointers[usize::try_from(input[input.len() - 1]).expect("cast within range")] = if n == 9 {
        input[0]
    } else {
        i64::try_from(input.len() + 1).expect("cast within range")
    };
    let mut cur = input[0];
    for _ in 0..rounds {
        let n = pointers.len() - 1;
        let t0 = pointers[usize::try_from(cur).expect("cast within range")];
        let t1 = pointers[usize::try_from(t0).expect("cast within range")];
        let t2 = pointers[usize::try_from(t1).expect("cast within range")];
        let ts: HashSet<i64> = [t0, t1, t2].iter().copied().collect();

        pointers[usize::try_from(cur).expect("cast within range")] =
            pointers[usize::try_from(t2).expect("cast within range")];

        let mut dst = if cur > 1 {
            cur - 1
        } else {
            i64::try_from(n).expect("cast within range")
        };
        while ts.contains(&dst) {
            dst = if dst > 1 {
                dst - 1
            } else {
                i64::try_from(n).expect("cast within range")
            };
        }
        pointers[usize::try_from(t2).expect("cast within range")] =
            pointers[usize::try_from(dst).expect("cast within range")];
        pointers[usize::try_from(dst).expect("cast within range")] = t0;
        cur = pointers[usize::try_from(cur).expect("cast within range")];
    }
    let mut res: Vec<i64> = Vec::new();
    let mut pointer = 1;
    loop {
        pointer = pointers[usize::try_from(pointer).expect("cast within range")];
        if pointer == 1 || res.len() > 9 {
            break res;
        }
        res.push(pointer);
    }
}

fn run(input: &str, part_two: bool) -> i64 {
    let nums: Vec<i64> = input
        .trim()
        .chars()
        .map(|n| i64::from(n.to_digit(10).expect("valid puzzle input")))
        .collect();
    if part_two {
        let res = move_cups(&nums, 10_000_000);
        res[0] * res[1]
    } else {
        move_cups(&nums, 100)
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<String>()
            .parse::<i64>()
            .expect("valid puzzle input")
    }
}

#[cfg(test)]
mod day_23_tests {
    use super::*;
    static INPUT: &str = "389125467";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, false), 67_384_529);
        assert_eq!(run(include_str!("../../input/23.txt"), false), 25_398_647);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 149_245_887_792);
        assert_eq!(
            run(include_str!("../../input/23.txt"), true),
            363_807_398_885
        );
    }
}
