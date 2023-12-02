use std::collections::{HashMap, HashSet};

static INPUT_TXT: &str = include_str!("../../input/03.txt");
fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn part_1(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .fold(HashMap::new(), |mut map, line| {
            let (_, quor) = line.trim().split_once(" @ ").unwrap();
            let (start_pos, size) = quor.trim().split_once(": ").unwrap();
            let start = start_pos
                .split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let size = size
                .split('x')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            for pos_x in start[0]..(start[0] + size[0]) {
                for pos_y in start[1]..(start[1] + size[1]) {
                    *map.entry((pos_x, pos_y)).or_insert(0) += 1;
                }
            }
            map
        })
        .iter()
        .filter_map(|(_, &value)| if value > 1 { Some(1) } else { None })
        .sum()
}

fn part_2(input: &str) -> i64 {
    let mut ids: HashSet<i64> = HashSet::new();
    let mut set: HashMap<(i64, i64), i64> = HashMap::new();
    for line in input.trim().lines() {
        let (id, quor) = line.trim().split_once(" @ ").unwrap();
        let mut chars = id.trim().chars();
        chars.next();
        let id_num = chars.as_str().parse::<i64>().unwrap();
        ids.insert(id_num);

        let (start_pos, size) = quor.trim().split_once(": ").unwrap();
        let start = start_pos
            .split(',')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let size = size
            .split('x')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        for pos_x in start[0]..(start[0] + size[0]) {
            for pos_y in start[1]..(start[1] + size[1]) {
                match set.get(&(pos_x, pos_y)) {
                    Some(x) => {
                        ids.remove(&id_num);
                        ids.remove(x);
                    }
                    None => {
                        set.insert((pos_x, pos_y), id_num);
                    }
                }
            }
        }
    }
    *ids.iter().collect::<Vec<_>>()[0]
}

#[cfg(test)]
mod day_3_tests {
    use super::*;
    static INPUT: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 4);
        assert_eq!(part_1(INPUT_TXT), 118_840);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 3);
        assert_eq!(part_2(INPUT_TXT), 919);
    }
}
