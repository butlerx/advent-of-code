static INPUT_TXT: &str = include_str!("../../input/13.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn are_mirrors(ns: &[u32], i: usize, smudge: u32) -> bool {
    (0..i)
        .rev()
        .zip(i..ns.len())
        .map(|(a, b)| (ns[a] ^ ns[b]).count_ones())
        .sum::<u32>()
        == smudge
}

fn parse_block(block: &str) -> (Vec<u32>, Vec<u32>) {
    let mut cols = Vec::new();
    let rows = block
        .lines()
        .map(|line| {
            cols.resize(line.len(), 0);
            line.chars().enumerate().fold(0, |row, (c, v)| {
                let is_rock = u32::from(v == '#');
                cols[c] = (cols[c] << 1) | is_rock;
                (row << 1) | is_rock
            })
        })
        .collect::<Vec<_>>();
    (cols, rows)
}

fn find_mirror(block: &str, smudge: u32) -> usize {
    let (cols, rows) = parse_block(block);
    if let Some(c) = (1..cols.len()).find(|c| are_mirrors(&cols, *c, smudge)) {
        return c;
    }
    if let Some(r) = (1..rows.len()).find(|r| are_mirrors(&rows, *r, smudge)) {
        return 100 * r;
    }
    unreachable!()
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|block| find_mirror(block, 0))
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|block| find_mirror(block, 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 405);
        assert_eq!(part_1(INPUT_TXT), 26957);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 400);
        assert_eq!(part_2(INPUT_TXT), 42695);
    }
}
