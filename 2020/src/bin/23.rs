use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/23.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn move_cups(input: Vec<i64>, rounds: usize) -> Vec<i64> {
    let (n, mut pointers) = if rounds <= 100 {
        (input.len() as i64, [0i64; 10].to_vec())
    } else {
        let n = 1_000_000;
        let mut pointers: Vec<i64> = (1..(n + 2)).collect();
        pointers[n as usize] = input[0] as i64;
        (n, pointers)
    };
    for w in input.windows(2) {
        pointers[w[0] as usize] = w[1] as i64;
    }
    pointers[input[input.len() - 1] as usize] = if n == 9 {
        input[0]
    } else {
        (input.len() + 1) as i64
    };
    let mut cur = input[0];
    for _ in 0..rounds {
        let n = pointers.len() - 1;
        let t0 = pointers[cur as usize];
        let t1 = pointers[t0 as usize];
        let t2 = pointers[t1 as usize];
        let ts: HashSet<i64> = [t0, t1, t2].iter().cloned().collect();

        pointers[cur as usize] = pointers[t2 as usize];

        let mut dst = if cur > 1 { cur - 1 } else { n as i64 };
        while ts.contains(&dst) {
            dst = if dst > 1 { dst - 1 } else { n as i64 };
        }
        pointers[t2 as usize] = pointers[dst as usize];
        pointers[dst as usize] = t0;
        cur = pointers[cur as usize];
    }
    let mut res: Vec<i64> = Vec::new();
    let mut pointer = 1;
    loop {
        pointer = pointers[pointer as usize];
        if pointer == 1 || res.len() > 9 {
            break res;
        }
        res.push(pointer);
    }
}

fn run(input: &str, part_two: bool) -> i64 {
    let nums = input
        .trim()
        .chars()
        .map(|n| n.to_digit(10).unwrap() as i64)
        .collect();
    if part_two {
        let res = move_cups(nums, 10_000_000);
        (res[0] * res[1]) as i64
    } else {
        move_cups(nums, 100)
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i64>()
            .unwrap()
    }
}

#[cfg(test)]
mod day_23_tests {
    use super::*;
    static INPUT: &str = "389125467";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, false), 67384529);
        assert_eq!(run(include_str!("../../input/23.txt"), false), 25398647);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 149245887792);
        assert_eq!(run(include_str!("../../input/23.txt"), true), 363807398885);
    }
}
