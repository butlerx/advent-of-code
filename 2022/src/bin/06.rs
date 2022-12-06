use std::collections::HashSet;

static INPUT_TXT: &[u8] = include_bytes!("../../input/06.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn marker_pos(input: &[u8], window_size: usize) -> usize {
    input
        .windows(window_size)
        .enumerate()
        .find(|(_, s)| s.into_iter().collect::<HashSet<_>>().len() == s.len())
        .unwrap()
        .0
        + window_size
}

fn part_1(input: &[u8]) -> usize {
    marker_pos(input, 4)
}

fn part_2(input: &[u8]) -> usize {
    marker_pos(input, 14)
}

#[cfg(test)]
mod day_6_tests {
    use super::*;
    static INPUT_1: &[u8] = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes();
    static INPUT_2: &[u8] = "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes();
    static INPUT_3: &[u8] = "nppdvjthqldpwncqszvftbrmjlhg".as_bytes();
    static INPUT_4: &[u8] = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes();
    static INPUT_5: &[u8] = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes();

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT_1), 7);
        assert_eq!(part_1(INPUT_2), 5);
        assert_eq!(part_1(INPUT_3), 6);
        assert_eq!(part_1(INPUT_4), 10);
        assert_eq!(part_1(INPUT_5), 11);
        assert_eq!(part_1(INPUT_TXT), 1155);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_1), 19);
        assert_eq!(part_2(INPUT_2), 23);
        assert_eq!(part_2(INPUT_3), 23);
        assert_eq!(part_2(INPUT_4), 29);
        assert_eq!(part_2(INPUT_5), 26);
        assert_eq!(part_2(INPUT_TXT), 2789);
    }
}
