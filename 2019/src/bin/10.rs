use std::collections::BTreeSet;
use std::convert::TryFrom;

static INPUT_TXT: &str = include_str!("../../input/10.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

/// Returns the most asteroids visible from any single asteroid.
///
/// # Panics
///
/// Panics if the input has no asteroids or a grid coordinate does not fit in an `i8`.
#[must_use]
pub fn part_1(input: &str) -> usize {
    let asteroids: Vec<_> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '#' {
                    Some((
                        i8::try_from(x).expect("cast within range"),
                        i8::try_from(y).expect("cast within range"),
                    ))
                } else {
                    None
                }
            })
        })
        .collect();

    asteroids
        .iter()
        .map(|&from| {
            asteroids
                .iter()
                .filter_map(|&to| {
                    if from == to {
                        None
                    } else {
                        let relative = (from.0 - to.0, from.1 - to.1);
                        // Truncate the angle to whole tenths of a degree and key the set
                        // on its bit pattern, avoiding a lossy float-to-int cast. Adding
                        // 0.0 folds -0.0 into 0.0 so equal angles share one key.
                        Some(
                            ((f64::from(relative.1)
                                .atan2(f64::from(relative.0))
                                .to_degrees()
                                * 10.0)
                                .trunc()
                                + 0.0)
                                .to_bits(),
                        )
                    }
                })
                .collect::<BTreeSet<u64>>()
                .len()
        })
        .max()
        .expect("valid puzzle input")
}

#[must_use]
pub fn part_2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_1: &str = ".#..#
    .....
    #####
    ....#
    ...##";

    static INPUT_2: &str = "......#.#.
    #..#.#....
    ..#######.
    .#.#.###..
    .#..#.....
    ..#....#.#
    #..#....#.
    .##.#..###
    ##...#..#.
    .#....####";

    static INPUT_3: &str = "#.#...#.#.
    .###....#.
    .#....#...
    ##.#.#.#.#
    ....#.#.#.
    .##..###.#
    ..#...##..
    ..##....##
    ......#...
    .####.###.";

    static INPUT_4: &str = ".#..#..###
    ####.###.#
    ....###.#.
    ..###.##.#
    ##.##.#.#.
    ....###..#
    ..#.#..#.#
    #..#.#.###
    .##...##.#
    .....#.#..";

    static INPUT_5: &str = ".#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT_1), 8);
        assert_eq!(part_1(INPUT_2), 33);
        assert_eq!(part_1(INPUT_3), 35);
        assert_eq!(part_1(INPUT_4), 41);
        assert_eq!(part_1(INPUT_5), 210);
        assert_eq!(part_1(INPUT_TXT), 230);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_1), 0);
        assert_eq!(part_2(INPUT_TXT), 0);
    }
}
