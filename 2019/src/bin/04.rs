fn main() {
    let input = include_str!("../../input/04.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn valid_password(password: &i64, max: bool) -> bool {
    let mut head = password / 10;
    let mut tail = password % 10;
    let mut pair = false;
    let mut combo = 1;

    loop {
        let prev = head % 10;

        if tail < prev {
            break false;
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
            break pair;
        }

        head /= 10;
        tail = prev;
    }
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let range: Vec<i64> = input
        .split('-')
        .map(|num| num.trim().parse::<i64>().unwrap())
        .collect();
    (range[0]..range[1])
        .filter(|pass| valid_password(pass, part_two))
        .count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(run(include_str!("../../input/04.txt"), false), 1748);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(include_str!("../../input/04.txt"), true), 1180);
    }
}
