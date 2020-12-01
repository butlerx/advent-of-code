use std::io::Error;

fn parse_expense(raw_expense: String) -> Result<Vec<i64>, Error> {
    let expense = raw_expense
        .split_whitespace()
        .map(|line| line.trim().parse::<i64>().expect("parse error"))
        .collect();
    Ok(expense)
}

fn find_2020_multiple(expenses: Vec<i64>) -> Result<i64, Error> {
    let goal = 2020;
    for (index_1, value_1) in expenses.iter().enumerate() {
        for (index_2, value_2) in expenses.iter().enumerate() {
            if index_1 == index_2 {
                continue;
            }
            if value_1 + value_2 == goal {
                return Ok(value_1 * value_2);
            }
        }
    }
    Ok(0)
}

pub fn run(input: String) -> Result<i64, Error> {
    let expenses = parse_expense(input)?;
    let multiple = find_2020_multiple(expenses)?;
    Ok(multiple)
}

#[cfg(test)]
mod tests {
    use super::{find_2020_multiple, parse_expense};

    #[test]
    fn test_parse_expense() {
        let results = vec![1721, 979, 366, 299, 675, 1456];
        assert!(parse_expense("1721\n979\n366\n299\n675\n1456".to_string()).unwrap() == results);
    }

    #[test]
    fn test_find_2020_multiple() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_2020_multiple(input).unwrap();
        assert!(514579 == result);
    }
}
