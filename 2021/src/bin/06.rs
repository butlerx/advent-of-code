static COMMA: &str = ",";

fn main() {
    let input = parse_input(include_str!("../../input/06.txt"));
    println!("Part 1: {}", calculate(&mut input.clone(), 80));
    println!("Part 2: {}", calculate(&mut input.clone(), 256));
}

fn parse_input(input: &str) -> [usize; 9] {
    let nums: Vec<usize> = input
        .split(COMMA)
        .map(|num| num.trim().parse::<usize>().unwrap())
        .collect();
    let mut counts: [usize; 9] = [0; 9];
    for age in nums {
        counts[age] += 1;
    }
    counts
}

fn calculate(counts: &mut [usize; 9], num_days: usize) -> usize {
    for _day in 0..num_days {
        counts[7usize] += counts[0usize];
        counts.rotate_left(1);
    }
    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_small_input() {
        let input = parse_input(INPUT);
        assert!(calculate(&mut input.clone(), 80) == 5934);
        assert!(calculate(&mut input.clone(), 256) == 26984457539);
    }

    #[test]
    fn test_large_input() {
        let input = parse_input(include_str!("../../input/06.txt"));
        assert!(calculate(&mut input.clone(), 80) == 361169);
        assert!(calculate(&mut input.clone(), 256) == 1634946868992);
    }
}
