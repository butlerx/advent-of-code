static INPUT_TXT: &str = include_str!("../../input/18.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }
}

impl From<u32> for Direction {
    fn from(n: u32) -> Self {
        match n {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => unreachable!(),
        }
    }
}

fn shoe_string(input: &str, parse_string: fn(&str) -> (Direction, i64)) -> i64 {
    let mut position = (0i64, 0i64);
    let (perimiter, filled) =
        input
            .trim()
            .lines()
            .map(parse_string)
            .fold((0, 0), |(perimiter, filled), (dir, steps)| {
                let (x, y) = match dir {
                    Direction::Up => (position.0, position.1 - steps),
                    Direction::Down => (position.0, position.1 + steps),
                    Direction::Left => (position.0 - steps, position.1),
                    Direction::Right => (position.0 + steps, position.1),
                };
                let sum = filled + (position.0 - x) * (position.1 + y);
                position = (x, y);
                (perimiter + steps, sum)
            });
    (perimiter + filled) / 2 + 1
}

fn part_1(input: &str) -> i64 {
    shoe_string(input, |l| {
        let (dir_str, rest) = l.split_once(' ').expect("failed to split line no space");
        let (steps_str, _) = rest.split_once(' ').expect("failed to split line no space");
        let steps = steps_str.parse::<i64>().expect("failed to parse steps");
        (Direction::from(dir_str), steps)
    })
}

fn part_2(input: &str) -> i64 {
    shoe_string(input, |l| {
        let (_, hex_str) = l.split_once('#').expect("failed to split line no #");
        let hex = u32::from_str_radix(&hex_str[..6], 16).expect("failed to parse hex");
        let steps = i64::try_from(hex >> 4).expect("failed to convert hex to int");
        (Direction::from(hex & 3), steps)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 62);
        assert_eq!(part_1(INPUT_TXT), 67891);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 952_408_144_115);
        assert_eq!(part_2(INPUT_TXT), 94_116_351_948_493);
    }
}
