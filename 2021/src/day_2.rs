pub fn run(input: &str, part_two: bool) -> i64 {
    let directions = input.lines().map(|line| {
        let i: Vec<&str> = line.split_whitespace().collect();
        (i[0], i[1].trim().parse::<i64>().unwrap())
    });
    if part_two {
        let position = directions.fold((0, 0, 0), |pos, (dir, num)| match dir {
            "forward" => (pos.0 + num, pos.1 + (num * pos.2), pos.2),
            "down" => (pos.0, pos.1, pos.2 + num),
            "up" => (pos.0, pos.1, pos.2 - num),
            _ => pos,
        });

        position.0 * position.1
    } else {
        let position = directions.fold((0, 0), |pos, (dir, num)| match dir {
            "forward" => (pos.0 + num, pos.1),
            "down" => (pos.0, pos.1 + num),
            "up" => (pos.0, pos.1 - num),
            _ => pos,
        });

        position.0 * position.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 150);
        let results = run(include_str!("../input/day_2.txt"), false);
        println!("{}", results);
        assert!(results == 2019945);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 900);
        let results = run(include_str!("../input/day_2.txt"), true);
        println!("{}", results);
        assert!(results == 1599311480);
    }
}
