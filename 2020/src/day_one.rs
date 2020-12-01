use std::io::Error;

fn parse_expense(raw_expense: String) -> Result<Vec<i64>, Error> {
    let expense = raw_expense
        .split_whitespace()
        .map(|line| line.trim().parse::<i64>().expect("parse error"))
        .collect();
    Ok(expense)
}

fn find_2020_multiple(expenses: &Vec<i64>, depth: i64) -> Result<i64, Error> {
    let index: Vec<usize> = vec![];
    find_2020_multiple_recurse(expenses, depth, index, 0, 1)
}

fn find_2020_multiple_recurse(
    expenses: &Vec<i64>,
    depth: i64,
    index_ignore: Vec<usize>,
    current_value: i64,
    current_multiple: i64,
) -> Result<i64, Error> {
    let goal = 2020;
    if depth == 0 {
        return Ok(0);
    }
    for (index, value) in expenses.iter().enumerate() {
        if index_ignore.contains(&index) {
            continue;
        }
        if index_ignore.len() != depth as usize {
            let mut new_index_ignore = index_ignore.to_vec();
            new_index_ignore.push(index);
            let new_value = current_value + value;
            if new_value >= goal {
                continue;
            }
            let res = find_2020_multiple_recurse(
                &expenses,
                depth,
                new_index_ignore,
                new_value,
                current_multiple * value,
            )?;
            if res != 0 {
                return Ok(res);
            }
        }
        if value + current_value == goal {
            return Ok(value * current_multiple);
        }
    }
    Ok(0)
}

pub fn run(input: String, depth: i64) -> Result<i64, Error> {
    let expenses = parse_expense(input)?;
    let multiple = find_2020_multiple(&expenses, depth)?;
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
        let result = find_2020_multiple(&input, 1).unwrap();
        println!("{}", result);
        assert!(514579 == result);
    }

    #[test]
    fn test_find_2020_multple_depth() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_2020_multiple(&input, 2).unwrap();
        assert!(241861950 == result);
    }
}
