use aoc_shared::time_execution;

static INPUT_TXT: &str = include_str!("../../input/25.txt");

fn main() {
    println!("🌟 --- Day 25 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");
}

fn count_hashes(line: &str) -> Vec<u8> {
    (0..5)
        .map(|i| line.chars().nth(i).map_or(0, |c| u8::from(c == '#')))
        .collect()
}

fn process_block(block: &str) -> Vec<u8> {
    block
        .lines()
        .map(count_hashes)
        .reduce(|acc, row| acc.iter().zip(row).map(|(a, b)| a + b).collect())
        .unwrap_or_else(|| vec![0; 5])
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let (locks, keys): (Vec<_>, Vec<_>) = input.split("\n\n").partition(|block| {
        block
            .lines()
            .next()
            .is_some_and(|l| l.chars().all(|c| c == '#'))
    });

    (
        locks.into_iter().map(process_block).collect(),
        keys.into_iter().map(process_block).collect(),
    )
}
fn part_1(input: &str) -> usize {
    let (locks, keys) = parse_input(input);

    locks
        .iter()
        .map(|l| {
            keys.iter()
                .filter(|k| k.iter().zip(l).all(|(a, b)| a + b <= 7))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 3);
        assert_eq!(part_1(INPUT_TXT), 2835);
    }
}
