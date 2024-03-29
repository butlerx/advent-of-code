fn main() {
    let input = parse_input(include_str!("../../input/06.txt"));
    println!("Part 1: {}", calculate(&mut input.clone(), 80));
    println!("Part 2: {}", calculate(&mut input.clone(), 256));
}

fn parse_input(input: &str) -> [usize; 9] {
    let mut counts: [usize; 9] = [0; 9];
    for age in input.split(',') {
        counts[age.trim().parse::<usize>().unwrap()] += 1;
    }
    counts
}

fn calculate(counts: &mut [usize; 9], num_days: usize) -> usize {
    for _ in 0..num_days {
        counts[7] += counts[0];
        counts.rotate_left(1);
    }
    counts.iter().sum()
}

#[cfg(test)]
mod day_6_tests {
    use super::*;
    static INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_small_input() {
        let input = parse_input(INPUT);
        assert_eq!(calculate(&mut input.clone(), 80), 5934);
        assert_eq!(calculate(&mut input.clone(), 256), 26_984_457_539);
    }

    #[test]
    fn test_large_input() {
        let input = parse_input(include_str!("../../input/06.txt"));
        assert_eq!(calculate(&mut input.clone(), 80), 361_169);
        assert_eq!(calculate(&mut input.clone(), 256), 1_634_946_868_992);
    }
}
