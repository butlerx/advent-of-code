use itertools::Itertools;
static INPUT_TXT: &str = include_str!("../../input/15.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT, 2_000_000));
    println!("Part 2: {}", part_2(INPUT_TXT, 4_000_000));
}

fn parse_input(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter_map(|w| w.parse::<i64>().ok())
                .collect_tuple()
                .map(|(x, y, dx, dy)| (x, y, (x - dx).abs() + (y - dy).abs()))
                .unwrap()
        })
        .collect()
}
fn part_1(input: &str, line_num: i64) -> i64 {
    parse_input(input)
        .iter()
        .map(|&(x, y, d)| (x, d - (line_num - y).abs()))
        .filter(|&(_, left)| left >= 0)
        .flat_map(|(x, left)| [(x - left, true), (x + left + 1, false)])
        .sorted()
        .fold((-1, 0, 0), |(mut ans, prev, inside), (x, start)| {
            if inside > 0 {
                ans += x - prev
            }
            (ans, x, inside + if start { 1 } else { -1 })
        })
        .0
}

fn part_2(input: &str, search_space: i64) -> i64 {
    let beacons = parse_input(input);
    for py in 0..=search_space {
        let mut px = 0;
        'inner: while px <= search_space {
            for &(x, y, d) in &beacons {
                let p = (x.abs_diff(px) + y.abs_diff(py)) as i64;
                if d >= p {
                    let ydiff = y.abs_diff(py) as i64;
                    let xdiff = d - ydiff;
                    px = x + xdiff + 1;
                    continue 'inner;
                }
            }

            return px * 4_000_000 + py;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod day_15_tests {
    use super::*;
    static INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT, 10), 26);
        assert_eq!(part_1(INPUT_TXT, 2_000_000), 5688618);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT, 20), 56000011);
        assert_eq!(part_2(INPUT_TXT, 4_000_000), 12625383204261);
    }
}
