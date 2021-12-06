fn main() {
    let input = include_str!("../../input/01.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn calculate_fuel(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}

pub fn run(input: &str, part_two: bool) -> i64 {
    input
        .lines()
        .map(|line| {
            if part_two {
                cumalitive_fuel(line.trim().parse::<f64>().unwrap())
            } else {
                calculate_fuel(line.trim().parse::<f64>().unwrap())
            }
        })
        .sum::<f64>() as i64
}

pub fn cumalitive_fuel(mass: f64) -> f64 {
    let mut total_fuel = 0.0;
    let mut fuel = calculate_fuel(mass);
    while fuel > 0.0 {
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
        assert_eq!(run(include_str!("../../input/01.txt"), false), 3268951);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 51316);
        assert_eq!(run(include_str!("../../input/01.txt"), true), 4900568);
    }
}
