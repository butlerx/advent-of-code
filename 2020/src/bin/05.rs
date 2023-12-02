fn main() {
    let input = include_str!("../../input/05.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn parse_space_partition(input: &str) -> i64 {
    i64::from(input.bytes().fold(0, |seat_id, byte| {
        u16::from(b"BR".contains(&byte)) + (seat_id << 1)
    }))
}

fn run(input: &str, part_two: bool) -> i64 {
    if part_two {
        part_2(input)
    } else {
        part_1(input)
    }
}

fn part_1(input: &str) -> i64 {
    input
        .lines()
        .map(parse_space_partition)
        .max()
        .expect("Missing seat")
}

fn part_2(input: &str) -> i64 {
    let mut occupied = [false; 1024];
    let mut max_occupied = 0;
    for line in input.lines() {
        let seat_id = parse_space_partition(line);
        occupied[seat_id as usize] = true;
        if seat_id > max_occupied {
            max_occupied = seat_id;
        }
    }
    (0..max_occupied)
        .rev()
        .find(|&seat_id| !occupied[seat_id as usize])
        .unwrap()
}

#[cfg(test)]
mod day_5_tests {
    use super::*;
    static INPUT: &str = "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn test_part_1() {
        assert!(part_1(INPUT) == 820);
        assert!(part_1(include_str!("../../input/05.txt")) == 901);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2(INPUT) == 819);
        assert!(part_2(include_str!("../../input/05.txt")) == 661);
    }
}
