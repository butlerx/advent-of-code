use std::collections::HashSet;
use std::time::Instant;

static INPUT_TXT: &[u8] = include_bytes!("../../input/06.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
    let hash_now = Instant::now();
    marker_pos(INPUT_TXT, 14);
    println!("hashmap: {}", hash_now.elapsed().as_nanos());
    let or_now = Instant::now();
    marker_pos_or(INPUT_TXT, 14);
    println!("or: {}", or_now.elapsed().as_nanos());
    let xor_now = Instant::now();
    marker_pos_xor(INPUT_TXT, 14);
    println!("xor: {}", xor_now.elapsed().as_nanos());
}

fn marker_pos(input: &[u8], window_size: usize) -> usize {
    input
        .windows(window_size)
        .enumerate()
        .find(|(_, s)| s.iter().collect::<HashSet<_>>().len() == s.len())
        .unwrap()
        .0
        + window_size
}

fn marker_pos_or(input: &[u8], window_size: usize) -> usize {
    input
        .windows(window_size)
        .enumerate()
        .find(|(_, s)| {
            (0..window_size)
                .fold(0u32, |set, j| set | 1 << (u32::from(s[j]) - 'a' as u32))
                .count_ones() as usize
                == window_size
        })
        .unwrap()
        .0
        + window_size
}

fn marker_pos_xor(input: &[u8], window_size: usize) -> usize {
    let mut set = 0u32;
    for i in 0..input.len() {
        set ^= 1 << (u32::from(input[i]) - 'a' as u32);
        if i >= window_size {
            set ^= 1 << (u32::from(input[i - window_size]) - 'a' as u32);
        }
        if set.count_ones() as usize == window_size {
            return i + 1;
        }
    }
    unreachable!()
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

    #[test]
    fn test_bitwise() {
        assert_eq!(marker_pos_or(INPUT_1, 14), 19);
        assert_eq!(marker_pos_or(INPUT_2, 14), 23);
        assert_eq!(marker_pos_or(INPUT_3, 14), 23);
        assert_eq!(marker_pos_or(INPUT_4, 14), 29);
        assert_eq!(marker_pos_or(INPUT_5, 14), 26);
        assert_eq!(marker_pos_or(INPUT_TXT, 14), 2789);
        assert_eq!(marker_pos_xor(INPUT_1, 14), 19);
        assert_eq!(marker_pos_xor(INPUT_2, 14), 23);
        assert_eq!(marker_pos_xor(INPUT_3, 14), 23);
        assert_eq!(marker_pos_xor(INPUT_4, 14), 29);
        assert_eq!(marker_pos_xor(INPUT_5, 14), 26);
        assert_eq!(marker_pos_xor(INPUT_TXT, 14), 2789);
    }
}
