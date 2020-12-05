use std::io::Error;

fn parse_space_partition(input: &str) -> i64 {
    input.bytes().fold(0, |seat_id, byte| {
        b"BR".contains(&byte) as u16 + (seat_id << 1)
    }) as i64
}

pub fn part_1(input: String) -> Result<i64, Error> {
    Ok(input
        .lines()
        .map(parse_space_partition)
        .max()
        .expect("Missing seat"))
}

pub fn part_2(input: String) -> Result<i64, Error> {
    let mut occupied = [false; 1024];
    let mut max_occupied = 0;
    for line in input.lines() {
        let seat_id = parse_space_partition(line);
        occupied[seat_id as usize] = true;
        if seat_id > max_occupied {
            max_occupied = seat_id;
        }
    }
    Ok((0..max_occupied)
        .rev()
        .into_iter()
        .find(|&seat_id| !occupied[seat_id as usize])
        .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn test_part_1() {
        assert!(part_1(INPUT.to_string()).unwrap() == 820);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2(INPUT.to_string()).unwrap() == 819);
    }
}
