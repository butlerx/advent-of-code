type ImageMap = Vec<Vec<bool>>;

fn main() {
    let input = include_str!("../../input/20.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    process_image(input, 2)
}

fn part_2(input: &str) -> usize {
    process_image(input, 50)
}

fn parse_input(input: &str, iterations: usize) -> (ImageMap, Vec<bool>) {
    let (r, img) = input.split_once("\n\n").unwrap();
    let first_line = r.replace("\n", "");
    let rules = first_line
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => unreachable!(),
        })
        .collect();

    let mut size = None;
    let map: ImageMap = img
        .lines()
        .enumerate()
        .map(|(li, line)| {
            let mut map_line = vec![false; line.len() + iterations * 2];
            for (i, c) in line.chars().enumerate() {
                match c {
                    '.' => continue,
                    '#' => *map_line.get_mut(i + iterations).unwrap() = true,
                    _ => unreachable!(),
                }
            }

            if li == 0 {
                size = Some(map_line.len());
            }
            map_line
        })
        .collect();

    let padding: ImageMap = (0..iterations)
        .map(|_| vec![false; size.unwrap()])
        .collect();
    let mut res_map = padding.clone();
    res_map.extend(map);
    res_map.extend(padding);

    (res_map, rules)
}

fn process_image(input: &str, iterations: usize) -> usize {
    let (image_map, rules) = parse_input(input, iterations);

    let (map, _) = (0..iterations).fold((image_map, false), |(map, default), _| {
        (
            map.iter()
                .enumerate()
                .map(|(r, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(c, _)| get_new_pixel((r, c), &map, &rules, default))
                        .collect()
                })
                .collect(),
            if default {
                rules[0b111111111]
            } else if rules[0] {
                true
            } else {
                default
            },
        )
    });

    map.iter().flatten().filter(|&v| *v).count()
}

fn get_new_pixel(rc: (usize, usize), map: &[Vec<bool>], rules: &[bool], default: bool) -> bool {
    let (r, c) = (rc.0 as isize, rc.1 as isize);
    rules[(r - 1..r + 2).fold(0, |num, row| {
        (c - 1..c + 2).fold(num, |n, col| {
            let number = n << 1;

            if if col < 0 || row < 0 || col as usize >= map.len() || r as usize >= map.len() {
                default
            } else {
                map[r as usize][c as usize]
            } {
                number + 1
            } else {
                number
            }
        })
    })]
}

#[cfg(test)]
mod day_20_tests {
    use super::*;
    static INPUT: &str =
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 35);
        assert_eq!(part_1(include_str!("../../input/20.txt")), 5361);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 3351);
        assert_eq!(part_2(include_str!("../../input/20.txt")), 16826);
    }
}
