fn main() {
    let input = include_str!("../../input/08.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn parse_digit(letters: &str) -> i64 {
    letters.chars().fold(0, |mut output, letter| {
        if letter.is_ascii_lowercase() {
            output |= 1 << (letter as usize - 'a' as usize);
        }
        output
    })
}

fn run(input: &str, part_two: bool) -> usize {
    input
        .lines()
        .map(|line| {
            let (s, o) = line.split_once(" | ").unwrap();
            let output = o.split(' ').map(parse_digit);
            if part_two {
                let signal: Vec<i64> = s.split(' ').map(parse_digit).collect();
                let digits = decode_to_order(&signal).unwrap();
                output.fold(0, |res, d| {
                    (res * 10)
                        + digits
                            .iter()
                            .enumerate()
                            .filter(|(_, segs)| **segs == d)
                            .map(|(n, _)| n)
                            .next()
                            .unwrap()
                })
            } else {
                output
                    .filter(|o| matches!(o.count_ones(), 2 | 3 | 4 | 7))
                    .count()
            }
        })
        .sum()
}

fn decode_to_order(signal: &[i64]) -> Option<[i64; 10]> {
    let mut digit = [0; 10];
    for l in signal.iter().copied() {
        match l.count_ones() {
            2 => digit[1] = l,
            3 => digit[7] = l,
            4 => digit[4] = l,
            7 => digit[8] = l,
            5 | 6 => (),
            _ => return None,
        }
    }

    for l in signal.iter().copied() {
        match l.count_ones() {
            5 => {
                if (l & digit[1]).count_ones() == 2 {
                    digit[3] = l;
                } else if (l & digit[4]).count_ones() == 2 {
                    digit[2] = l;
                } else {
                    digit[5] = l;
                }
            }
            6 => {
                if (l & digit[4]).count_ones() == 4 {
                    digit[9] = l;
                } else if (l & digit[1]).count_ones() == 2 {
                    digit[0] = l;
                } else {
                    digit[6] = l;
                }
            }
            _ => (),
        }
    }
    Some(digit)
}

#[cfg(test)]
mod day_8_tests {
    use super::*;
    static INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, false), 26);
        assert_eq!(run(include_str!("../../input/08.txt"), false), 237);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 61229);
        assert_eq!(run(include_str!("../../input/08.txt"), true), 1_009_098);
    }
}
