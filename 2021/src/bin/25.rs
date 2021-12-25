use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/25.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> i64 {
    let mut map = input.lines().map(|l| l.chars().collect()).collect();
    let mut round = 0;
    loop {
        let move_1 = step(&mut map, true);
        let move_2 = step(&mut map, false);
        round += 1;
        if !move_1 && !move_2 {
            break round;
        }
    }
}

fn step(map: &mut Vec<Vec<char>>, go_east: bool) -> bool {
    let (direction_south, direction_east) = if go_east { (0, 1) } else { (1, 0) };
    let (height, width) = (map.len(), map[0].len());
    let (moved, new_map) = (0..height).cartesian_product(0..width).fold(
        (false, vec![vec!['.'; width]; height]),
        |(moved, mut new_map), (south, east)| match map[south][east] {
            '>' if map[south][(east + direction_east) % width] == '.' => {
                new_map[south][(east + direction_east) % width] = '>';
                (true, new_map)
            }
            'v' if map[(south + direction_south) % height][east] == '.' => {
                new_map[(south + direction_south) % height][east] = 'v';
                (true, new_map)
            }
            '>' => {
                new_map[south][east] = '>';
                (moved, new_map)
            }
            'v' => {
                new_map[south][east] = 'v';
                (moved, new_map)
            }
            _ => (moved, new_map),
        },
    );
    *map = new_map;
    moved
}

#[cfg(test)]
mod day_25_tests {
    use super::*;
    static INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 58);
        assert_eq!(part_1(include_str!("../../input/25.txt")), 321);
    }
}
