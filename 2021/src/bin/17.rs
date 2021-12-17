type Targets = (i64, i64);

fn main() {
    let input = include_str!("../../input/17.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn contains(target_x: Targets, target_y: Targets, (x, y): Targets) -> bool {
    target_x.0 <= x && x <= target_x.1 && target_y.0 <= y && y <= target_y.1
}

fn find_max_height(target_x: Targets, target_y: Targets, mut point: Targets) -> Option<i64> {
    let (mut x, mut y, mut max_height): (i64, i64, i64) = (0, 0, 0);
    loop {
        if contains(target_x, target_y, (x, y)) {
            break Some(max_height);
        }
        x += point.0;
        y += point.1;
        if y < target_y.0 {
            break None;
        }
        max_height = max_height.max(y);
        point = (0.max(point.0 - 1), point.1 - 1);
    }
}

fn parse_range(input: &str) -> Targets {
    let (_, range) = input.trim().split_once('=').unwrap();
    let (low, high) = range.trim().split_once("..").unwrap();
    (
        low.trim().parse::<i64>().unwrap(),
        high.trim().parse::<i64>().unwrap(),
    )
}

fn parse_input(input: &str) -> (Targets, Targets) {
    let args = input.replace("target area:", "");
    let (x, y) = args.trim().split_once(',').unwrap();
    (parse_range(x), parse_range(y))
}

fn heights(input: &str) -> impl Iterator<Item = i64> {
    let (x, y) = parse_input(input);
    let min_x = ((8.0 * x.0 as f64 + 1.0).sqrt() / 2.0 - 0.5).ceil() as i64;
    (min_x..=x.1)
        .flat_map(move |vx| (y.0..=-y.0).filter_map(move |vy| find_max_height(x, y, (vx, vy))))
}

fn part_1(input: &str) -> i64 {
    heights(input).max().unwrap()
}

fn part_2(input: &str) -> usize {
    heights(input).count()
}

#[cfg(test)]
mod day_17_tests {
    use super::*;
    static INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 45);
        assert_eq!(part_1(include_str!("../../input/17.txt")), 5995);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 112);
        assert_eq!(part_2(include_str!("../../input/17.txt")), 3202);
    }
}
