static INPUT_TXT: &str = include_str!("../../input/25.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
}

fn dec_to_snafu(mut num: i64) -> String {
    let mut res = Vec::new();
    while num > 0 {
        let c = match (num % 5) as u32 {
            d if d < 3 => std::char::from_digit(d, 10).unwrap(),
            d if d == 3 => {
                num += 2;
                '='
            }
            _ => {
                num += 1;
                '-'
            }
        };
        res.push(c);
        num /= 5;
    }
    res.reverse();
    res.into_iter().collect()
}

fn snafu_to_dec_sum(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|l| {
            l.chars().fold(0, |i, n| {
                let num = match n {
                    '-' => -1,
                    '=' => -2,
                    _ => i64::from(n.to_digit(10).unwrap()),
                };
                num + (i * 5)
            })
        })
        .sum()
}

fn part_1(input: &str) -> String {
    dec_to_snafu(snafu_to_dec_sum(input))
}

#[cfg(test)]
mod day_25_tests {
    use super::*;
    static INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_snafu_to_dec_sum() {
        assert_eq!(snafu_to_dec_sum(INPUT), 4890);
        assert_eq!(snafu_to_dec_sum(INPUT_TXT), 29_600_609_295_066);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), "2=-1=0".to_string());
        assert_eq!(part_1(INPUT_TXT), "2=--00--0220-0-21==1".to_string());
    }
}
