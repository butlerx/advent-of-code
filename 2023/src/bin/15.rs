use std::collections::VecDeque;

static INPUT_TXT: &str = include_str!("../../input/15.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| {
        ((acc + u32::try_from(c).expect("failed to get ascii value")) * 17) % 256
    })
}

fn part_1(input: &str) -> u32 {
    input.trim().split(',').map(hash).sum()
}

enum Op {
    Minus,
    Equals(usize),
}

fn hashmap<'a>(
    mut boxes: Vec<VecDeque<(&'a str, usize)>>,
    lens: &'a str,
) -> Vec<VecDeque<(&'a str, usize)>> {
    let (label, focal_length) = if lens.contains('=') {
        let (a, b) = lens.split_once('=').unwrap();
        (a, Op::Equals(b.parse::<usize>().unwrap()))
    } else {
        (lens.trim_end_matches('-'), Op::Minus)
    };
    let box_index = usize::try_from(hash(label)).expect("failed to get box index");

    if let Some(pos) = boxes[box_index].iter().position(|&(l, _)| l == label) {
        match focal_length {
            Op::Equals(f) => {
                boxes[box_index][pos] = (label, f);
            }
            Op::Minus => {
                boxes[box_index].remove(pos);
            }
        }
    } else if let Op::Equals(f) = focal_length {
        boxes[box_index].push_back((label, f));
    }
    boxes
}

fn focusing_power(boxes: &VecDeque<(&str, usize)>) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(index, &(_, focus_length))| (index + 1) * focus_length)
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .fold(vec![VecDeque::default(); 256], hashmap)
        .iter()
        .enumerate()
        .filter(|(_, b)| !b.is_empty())
        .map(|(index, b)| (index + 1) * focusing_power(b))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 1320);
        assert_eq!(part_1(INPUT_TXT), 511_498);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 145);
        assert_eq!(part_2(INPUT_TXT), 284_674);
    }
}
