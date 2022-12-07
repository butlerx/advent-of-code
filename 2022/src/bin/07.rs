use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
static INPUT_TXT: &str = include_str!("../../input/07.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> HashMap<PathBuf, i64> {
    let mut pwd = PathBuf::new();
    let fs: HashMap<PathBuf, HashSet<(i64, &str)>> =
        input
            .trim()
            .split('$')
            .skip(1)
            .fold(HashMap::new(), |mut fs, command| {
                match command.trim().lines().next().unwrap() {
                    "cd .." => {
                        pwd.pop();
                    }
                    "ls" => {
                        let entries = command.lines().skip(1).map(|output| {
                            let (size, file) = output.split_once(' ').unwrap();
                            (size.parse::<i64>().unwrap_or(-1), file)
                        });
                        fs.entry(pwd.clone()).or_default().extend(entries);
                    }
                    cd_dir => {
                        pwd.push(cd_dir.split_once(' ').unwrap().1);
                    }
                }
                fs
            });

    fs.keys().fold(HashMap::new(), |mut dir_sizes, dir| {
        dir_size(&fs, &mut dir_sizes, dir);
        dir_sizes
    })
}

fn dir_size(
    fs: &HashMap<PathBuf, HashSet<(i64, &str)>>,
    dir_sizes: &mut HashMap<PathBuf, i64>,
    dir: &PathBuf,
) {
    let size = fs
        .get(dir)
        .unwrap()
        .iter()
        .map(|&(s, d)| match s {
            -1 => {
                let dir = dir.join(d);
                dir_size(fs, dir_sizes, &dir);
                *dir_sizes.get(&dir).unwrap()
            }
            s => s,
        })
        .sum();
    dir_sizes.insert(dir.clone(), size);
}

fn part_1(input: &str) -> i64 {
    parse_input(input).values().filter(|&&s| s <= 100000).sum()
}

fn part_2(input: &str) -> i64 {
    let sizes = parse_input(input);
    let total_size = sizes.get(&PathBuf::from("/")).unwrap();
    *sizes
        .values()
        .filter(|&s| 40000000 + s >= *total_size)
        .min()
        .unwrap()
}

#[cfg(test)]
mod day_7_tests {
    use super::*;
    static INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 95437);
        assert_eq!(part_1(INPUT_TXT), 1390824);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 24933642);
        assert_eq!(part_2(INPUT_TXT), 7490863);
    }
}
