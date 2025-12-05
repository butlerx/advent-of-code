use aoc_shared::time_execution_us;
static INPUT_TXT: &str = include_str!("../../input/05.txt");

fn main() {
    println!("🌟 --- Day 5 Results --- 🌟");
    let (res_1, duration_1) = time_execution_us(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} us");

    let (res_2, duration_2) = time_execution_us(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} us");
}

#[inline]
fn parse_range(section: &str) -> (usize, usize) {
    let (start_str, end_str) = section.split_once('-').expect("Invalid range");
    let start = start_str.parse::<usize>().expect("Invalid start number");
    let end = end_str.parse::<usize>().expect("Invalid end number");
    (start, end)
}

fn part_1(input: &str) -> usize {
    let (ranges, ingredient) = input.trim().split_once("\n\n").expect("Invalid input");

    let fresh_ranges: Vec<(usize, usize)> = ranges.trim().lines().map(parse_range).collect();

    ingredient
        .trim()
        .lines()
        .map(|line| line.parse::<usize>().expect("Invalid number"))
        .filter(|&n| {
            fresh_ranges
                .iter()
                .any(|(start, end)| n >= *start && n <= *end)
        })
        .count()
}

fn part_2(input: &str) -> usize {
    let (ranges, _) = input.trim().split_once("\n\n").expect("Invalid input");

    let mut ranges: Vec<(usize, usize)> = ranges.trim().lines().map(parse_range).collect();
    ranges.sort_unstable_by_key(|r| r.0);

    ranges
        .into_iter()
        .fold::<Vec<(usize, usize)>, _>(vec![], |mut merged, (start, end)| {
            match merged.last_mut() {
                Some(last) if start <= last.1 + 1 => {
                    last.1 = last.1.max(end);
                }
                _ => {
                    merged.push((start, end));
                }
            }
            merged
        })
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(TEST_INPUT), 3);
        assert_eq!(part_1(INPUT_TXT), 520);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT), 14);
        assert_eq!(part_2(INPUT_TXT), 347_338_785_050_515);
    }
}
