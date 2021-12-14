use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/13.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn find_next_bus(current_time: i64, bus_ids: Vec<i64>) -> i64 {
    (current_time..)
        .find_map(|i| bus_ids.iter().find(|&b| i % b == 0).map(|b| (i, b)))
        .map(|(i, b)| b * (i - current_time))
        .unwrap()
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn run(input: &str, part_two: bool) -> i64 {
    let inputs: (&str, &str) = input.lines().collect_tuple().unwrap();
    let ids: Vec<(i64, i64)> = inputs
        .1
        .split(',')
        .enumerate()
        .filter(|(_, n)| n != &"x")
        .map(|(i, n)| (i as i64, n.trim().parse::<i64>().unwrap()))
        .collect();
    let goal = inputs.0.trim().parse::<i64>().unwrap();
    let busses = ids.iter().map(|&(_, b)| b).collect::<Vec<_>>();
    if part_two {
        let res = ids.iter().map(|&(i, b)| (b - i)).collect::<Vec<_>>();
        chinese_remainder(&res, &busses).unwrap()
    } else {
        find_next_bus(goal, busses)
    }
}

#[cfg(test)]
mod day_13_tests {
    use super::*;
    static INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 295);
        let results = run(include_str!("../../input/13.txt"), false);
        println!("{}", results);
        assert!(results == 2382);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 1068781);
        let results = run(include_str!("../../input/13.txt"), true);
        println!("{}", results);
        assert!(results == 906332393333683);
    }
}
