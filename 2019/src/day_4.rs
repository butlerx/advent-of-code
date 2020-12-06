fn valid_password(password: &i64, max: bool) -> bool {
    let mut head = password / 10;
    let mut tail = password % 10;
    let mut pair = false;
    let mut combo = 1;

    loop {
        let prev = head % 10;

        if tail < prev {
            return false;
        }

        if tail == prev {
            if max {
                combo += 1;
            } else {
                pair = true;
            }
        } else if max {
            if combo == 2 {
                pair = true;
            }
            combo = 1;
        }

        if head == 0 {
            return pair;
        }

        head /= 10;
        tail = prev;
    }
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let range: Vec<i64> = input
        .split("-")
        .map(|num| num.trim().parse::<i64>().unwrap())
        .collect();
    (range[0]..range[1])
        .filter(|pass| valid_password(pass, part_two))
        .count() as i64
}

pub fn part_1(input: &str) -> i64 {
    run(input, false)
}

pub fn part_2(input: &str) -> i64 {
    run(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let results = part_1(include_str!("../input/day_4.txt"));
        println!("{}", results);
        assert!(results == 1748);
    }

    #[test]
    fn test_part_2() {
        let results = part_2(include_str!("../input/day_4.txt"));
        println!("{}", results);
        assert!(results == 981);
    }
}
