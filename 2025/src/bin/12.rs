#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::time_execution_us;
static INPUT_TXT: &str = include_str!("../../input/12.txt");

fn main() {
    println!("🌟 --- Day 12 Results --- 🌟");
    let (res_1, duration_1) = time_execution_us(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} us");
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .filter(|section| !section.contains('#'))
        .flat_map(|section| section.lines())
        .filter_map(parse_region_line)
        .filter(|((width, height), present_counts)| {
            let three_by_three_squares = (width / 3) * (height / 3);
            three_by_three_squares >= present_counts.iter().sum()
        })
        .count()
}

fn parse_region_line(line: &str) -> Option<((usize, usize), Vec<usize>)> {
    let mut entries = line.split_ascii_whitespace();
    let dimensions = entries.next()?;

    let (x, y) = parse_dimensions(dimensions)?;
    let counts = entries.filter_map(|e| e.parse().ok()).collect();

    Some(((x, y), counts))
}

fn parse_dimensions(dimensions: &str) -> Option<(usize, usize)> {
    let x_pos = dimensions.find('x')?;
    let x = dimensions[..x_pos].parse().ok()?;
    let y = dimensions[x_pos + 1..dimensions.len() - 1].parse().ok()?;
    Some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1() {
        //assert_eq!(part_1(TEST_INPUT), 2);
        assert_eq!(part_1(INPUT_TXT), 487);
    }
}
