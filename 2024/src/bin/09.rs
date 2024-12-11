#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use std::time::Instant;

static INPUT_TXT: &str = include_str!("../../input/09.txt");

fn main() {
    println!("🌟 --- Day 9 Results --- 🌟");
    let start_1 = Instant::now();
    let res_1 = part_1(INPUT_TXT);
    let duration_1 = start_1.elapsed().as_millis();
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let start_2 = Instant::now();
    let res_2 = part_2(INPUT_TXT);
    let duration_2 = start_2.elapsed().as_millis();
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn compact_disk(disk: Vec<(usize, i32)>) -> usize {
    (0..disk.len())
        .rev()
        .fold(disk, |mut acc, i| {
            let (size, id) = acc[i];
            if id == -1 {
                return acc;
            }
            if let Some(j) = acc[0..i].iter().position(|&(s, id)| id == -1 && size <= s) {
                let (empty_size, _) = acc[j];
                acc[j] = (size, id);
                acc[i] = (size, -1);
                if size < empty_size {
                    acc.insert(j + 1, (empty_size - size, -1));
                }
            }
            acc
        })
        .iter()
        .flat_map(|&(s, id)| std::iter::repeat(id).take(s))
        .enumerate()
        .filter(|&(_, id)| id != -1)
        .map(|(i, id)| i * id as usize)
        .sum()
}

fn part_1(input: &str) -> usize {
    let mut id = 0;
    compact_disk(
        input
            .trim()
            .bytes()
            .enumerate()
            .flat_map(|(i, b)| {
                let value = if i % 2 == 0 {
                    id += 1;
                    id - 1
                } else {
                    -1
                };
                (0..b - b'0').map(move |_| (1, value))
            })
            .collect(),
    )
}

fn part_2(input: &str) -> usize {
    let mut id = 0;
    compact_disk(
        input
            .trim()
            .bytes()
            .enumerate()
            .map(|(i, b)| {
                let value = if i % 2 == 0 {
                    id += 1;
                    id - 1
                } else {
                    -1
                };
                ((b - b'0') as usize, value)
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 1928);
        assert_eq!(part_1(INPUT_TXT), 6_323_641_412_437);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 2858);
        assert_eq!(part_2(INPUT_TXT), 6_351_801_932_670);
    }
}
