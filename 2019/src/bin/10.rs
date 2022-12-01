use std::collections::BTreeSet;

static INPUT_TXT: &str = include_str!("../../input/10.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

pub fn part_1(input: &str) -> usize {
    let asteroids: Vec<_> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '#' {
                    Some((x as i8, y as i8))
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
                    if from != to {
                        let relative = (from.0 - to.0, from.1 - to.1);
                        Some(
                            ((relative.1 as f64).atan2(relative.0 as f64).to_degrees() * 10.0)
                                as i16,
                        )
                    } else {
                        None
                    }
                })
                .collect::<BTreeSet<i16>>()
                .len()
        })
        .max()
        .unwrap()
}

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
