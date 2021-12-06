fn main() {
    let input = include_str!("../../input/01.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|num| num.trim().parse::<i64>().unwrap())
        .collect()
}

pub fn part_1(input: &str) -> i64 {
    parse_input(input)
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count() as i64
}

pub fn part_2(input: &str) -> i64 {
    parse_input(input)
        .windows(4)
        .filter(|w| w[0] < w[3])
        .count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 7);
        assert_eq!(1715, part_1(include_str!("../../input/01.txt")));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 5);
        assert_eq!(part_2(include_str!("../../input/01.txt")), 1739);
    }
}
