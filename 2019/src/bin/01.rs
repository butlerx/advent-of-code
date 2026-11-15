fn main() {
    let input = include_str!("../../input/01.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn calculate_fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

fn run(input: &str, part_two: bool) -> i64 {
    input
        .lines()
        .map(|line| {
            let mass = line.trim().parse::<i64>().unwrap();
            if part_two {
                cumalitive_fuel(mass)
            } else {
                calculate_fuel(mass)
            }
        })
        .sum()
}

fn cumalitive_fuel(mass: i64) -> i64 {
    let mut total_fuel = 0;
    let mut fuel = calculate_fuel(mass);
    while fuel > 0 {
        total_fuel += fuel;
        fuel = calculate_fuel(fuel);
    }
    total_fuel
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "12
14
1969
100756";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, false), 34241);
        assert_eq!(run(include_str!("../../input/01.txt"), false), 3_268_951);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 51316);
        assert_eq!(run(include_str!("../../input/01.txt"), true), 4_900_568);
    }
}
