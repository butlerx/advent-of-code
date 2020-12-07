fn calculate_fuel(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}

pub fn run(input: &str, part_two: bool) -> i64 {
    input
        .lines()
        .map(|line| {
            if part_two {
                cumalitive_fule(line.trim().parse::<f64>().unwrap())
            } else {
                calculate_fuel(line.trim().parse::<f64>().unwrap())
            }
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "12
14
1969
100756";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 34241);
        let results = run(include_str!("../input/day_1.txt"), false);
        println!("{}", results);
        assert!(results == 3268951);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 51316);
        let results = run(include_str!("../input/day_1.txt"), true);
        println!("{}", results);
        assert!(results == 4900568);
    }
}
