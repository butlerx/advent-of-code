#![warn(clippy::pedantic, clippy::perf)]
static INPUT_TXT: &str = include_str!("../../input/12.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_line(line: &str) -> (String, Vec<usize>) {
    let (springs, groups_str) = line.split_once(' ').unwrap();
    let groups = groups_str
        .split(',')
        .map(|g| g.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    (springs.to_string(), groups)
}

fn possible_ways(springs: &[char], counts: &[usize]) -> usize {
    let s_len = springs.len();
    let c_len = counts.len();
    let mut results = vec![vec![vec![0; s_len + 1]; c_len + 1]; s_len + 1];

    results[s_len][c_len][0] = 1;
    results[s_len][c_len - 1][counts[c_len - 1]] = 1;

    (0..s_len).rev().fold(results, |mut results, pos| {
        for (group, max) in counts.iter().enumerate().take(c_len) {
            for count in 0..=*max {
                for c in ['.', '#'] {
                    if springs[pos] == c || springs[pos] == '?' {
                        let num = match c {
                            '.' if count == 0 => results[pos + 1][group][0],
                            '.' if group < c_len && counts[group] == count => {
                                results[pos + 1][group + 1][0]
                            }
                            '#' => results[pos + 1][group][count + 1],
                            _ => 0,
                        };
                        results[pos][group][count] += num;
                    }
                }
            }
        }
        if matches!(springs[pos], '.' | '?') {
            results[pos][c_len][0] += results[pos + 1][c_len][0];
        }
        results
    })[0][0][0]
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| {
            let (springs, groups) = parse_line(l);
            possible_ways(&springs.chars().collect::<Vec<_>>(), &groups)
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| {
            let (springs, groups) = parse_line(l);
            possible_ways(
                &[springs.as_str(); 5].join("?").chars().collect::<Vec<_>>(),
                &groups.repeat(5),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 21);
        assert_eq!(part_1(INPUT_TXT), 7407);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 525_152);
        assert_eq!(part_2(INPUT_TXT), 30_568_243_604_962);
    }
}
