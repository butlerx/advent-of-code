fn calculate_fuel(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}

pub fn part_1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| calculate_fuel(line.trim().parse::<f64>().unwrap()))
        .sum::<f64>() as i64
}

pub fn cumalitive_fule(mass: f64) -> f64 {
    let mut total_fuel = 0.0;
    let mut fuel = calculate_fuel(mass);
    while fuel > 0.0 {
        total_fuel += fuel;
        fuel = calculate_fuel(fuel);
    }
    total_fuel
}

pub fn part_2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| cumalitive_fule(line.trim().parse::<f64>().unwrap()))
        .sum::<f64>() as i64
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
        assert!(part_1(INPUT) == 34241);
        let results = part_1(include_str!("../input/day_1.txt"));
        println!("{}", results);
        assert!(results == 3268951);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2(INPUT) == 51316);
        let results = part_2(include_str!("../input/day_1.txt"));
        println!("{}", results);
        assert!(results == 4900568);
    }
}
