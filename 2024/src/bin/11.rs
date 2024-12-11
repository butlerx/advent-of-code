use std::{collections::HashMap, time::Instant};

static INPUT_TXT: &str = include_str!("../../input/11.txt");

fn main() {
    println!("🌟 --- Day 11 Results --- 🌟");
    let start_1 = Instant::now();
    let res_1 = part_1(INPUT_TXT);
    let duration_1 = start_1.elapsed().as_millis();
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let start_2 = Instant::now();
    let res_2 = part_2(INPUT_TXT);
    let duration_2 = start_2.elapsed().as_millis();
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn parse_input(input: &str) -> HashMap<usize, usize> {
    input
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .fold(HashMap::default(), |mut acc, val| {
            *acc.entry(val).or_default() += 1;
            acc
        })
}

fn blink(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![1];
    }
    let num_len = n.ilog10() + 1;
    if num_len % 2 == 0 {
        let divisor = 10usize.pow(num_len / 2);
        vec![n / divisor, n % divisor]
    } else {
        vec![n * 2024]
    }
}

fn blink_all(numbers: HashMap<usize, usize>, times: usize) -> usize {
    (0..times)
        .fold(numbers, |acc, _| {
            acc.iter()
                .flat_map(|(n, freq)| blink(*n).into_iter().map(move |new_num| (new_num, freq)))
                .fold(HashMap::default(), |mut res, (n, freq)| {
                    *res.entry(n).or_default() += freq;
                    res
                })
        })
        .values()
        .sum()
}

fn part_1(input: &str) -> usize {
    blink_all(parse_input(input), 25)
}

fn part_2(input: &str) -> usize {
    blink_all(parse_input(input), 75)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "125 17";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 55312);
        assert_eq!(part_1(INPUT_TXT), 199986);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 65601038650482);
        assert_eq!(part_2(INPUT_TXT), 236804088748754);
    }
}
