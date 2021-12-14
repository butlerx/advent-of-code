fn main() {
    let (p1, p2) = run(include_str!("../../input/02.txt"));
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn run(input: &str) -> (i64, i64) {
    let position = input
        .lines()
        .map(|line| {
            let i: Vec<&str> = line.split_whitespace().collect();
            (i[0], i[1].trim().parse::<i64>().unwrap())
        })
        .fold((0, 0, 0), |pos, (dir, dis)| match dir {
            "forward" => (pos.0 + pos.2 * dis, pos.1 + dis, pos.2),
            "down" => (pos.0, pos.1, pos.2 + dis),
            "up" => (pos.0, pos.1, pos.2 - dis),
            _ => pos,
        });

    (position.1 * position.2, position.0 * position.1)
}

#[cfg(test)]
mod day_2_tests {
    use super::*;

    static INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_tun() {
        assert_eq!(run(INPUT), (150, 900));
        let (p1, p2) = run(include_str!("../../input/02.txt"));
        assert_eq!(p1, 2019945);
        assert_eq!(p2, 1599311480);
    }
}
