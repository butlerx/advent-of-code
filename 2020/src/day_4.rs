use regex::Regex;

fn validate_year(year_str: &str, low: i64, high: i64) -> bool {
    let year = year_str.parse().unwrap();
    (low <= year) && (year <= high)
}

fn validate_key_value(kv: &str) -> bool {
    let key_value: Vec<&str> = kv.split(":").collect();
    match key_value[0] {
        "byr" => validate_year(key_value[1], 1920, 2002),
        "iyr" => validate_year(key_value[1], 2010, 2020),
        "eyr" => validate_year(key_value[1], 2020, 2030),
        "hgt" => {
            if key_value[1].contains("cm") {
                let height = key_value[1].replace("cm", "").parse().unwrap();
                (150 <= height) && (height <= 193)
            } else if key_value[1].contains("in") {
                let height = key_value[1].replace("in", "").parse().unwrap();
                (59 <= height) && (height <= 76)
            } else {
                false
            }
        }
        "hcl" => Regex::new(r"^#[0-9a-f]{6}$")
            .unwrap()
            .is_match(key_value[1]),
        "ecl" => vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&key_value[1]),
        "pid" => Regex::new(r"^\d{9}$").unwrap().is_match(key_value[1]),
        "cid" => true,
        _ => true,
    }
}

fn valid_passport(passport: &str) -> bool {
    if !check_required_fields(passport) {
        false
    } else {
        passport
            .replace('\n', " ")
            .split(" ")
            .map(validate_key_value)
            .all(|item| item)
    }
}

fn check_required_fields(passport: &str) -> bool {
    for field in vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
        if !passport.contains(field) {
            return false;
        }
    }
    true
}

pub fn run(input: &str, part_2: bool) -> i64 {
    input
        .split("\n\n")
        .filter(|line| {
            if !part_2 {
                check_required_fields(line.trim())
            } else {
                valid_passport(line.trim())
            }
        })
        .collect::<Vec<_>>()
        .len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    static VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    static INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 2);
        assert!(run(include_str!("../input/day_4.txt"), false) == 230);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 2);
        assert!(run(VALID, true) == 4);
        assert!(run(INVALID, true) == 0);
        assert!(run(include_str!("../input/day_4.txt"), true) == 156);
    }
}
