fn main() {
    let input = include_str!("../../input/12.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn rotate_heading(heading: (f32, f32), angle_change: f32) -> (f32, f32) {
    let rads = angle_change.to_radians();
    let last_heading = heading;
    (
        (rads.sin() * last_heading.1 + rads.cos() * last_heading.0).round(),
        (rads.cos() * last_heading.1 - rads.sin() * last_heading.0).round(),
    )
}

fn follow_directions(
    directions: Vec<(char, f32)>,
    mut heading: (f32, f32),
    part_two: bool,
) -> (i64, i64) {
    let mut pos = (0.0, 0.0);
    for (d, num) in directions.iter() {
        match d {
            'N' => {
                if part_two {
                    heading.1 += num
                } else {
                    pos.0 += num
                }
            }
            'S' => {
                if part_two {
                    heading.1 -= num
                } else {
                    pos.0 -= num
                }
            }
            'E' => {
                if part_two {
                    heading.0 += num
                } else {
                    pos.1 += num
                }
            }
            'W' => {
                if part_two {
                    heading.0 -= num
                } else {
                    pos.1 -= num
                }
            }
            'L' => heading = rotate_heading(heading, -num as f32),
            'R' => heading = rotate_heading(heading, *num),
            'F' => {
                pos.1 += num * heading.0;
                pos.0 += num * heading.1;
            }
            _ => unreachable!(),
        };
    }
    (pos.0.abs() as i64, pos.1.abs() as i64)
}

fn run(input: &str, part_two: bool) -> i64 {
    let directions: Vec<(char, f32)> = input
        .lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
        .collect();
    let (north, east) = follow_directions(
        directions,
        if part_two { (10.0, 1.0) } else { (1.0, 0.0) },
        part_two,
    );
    north + east
}

#[cfg(test)]
mod day_12_tests {
    use super::*;
    static INPUT: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 25);
        let results = run(include_str!("../../input/12.txt"), false);
        println!("{}", results);
        assert!(results == 1007);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 286);
        let results = run(include_str!("../../input/12.txt"), true);
        println!("{}", results);
        assert!(results == 41212);
    }
}
