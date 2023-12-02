static INPUT_TXT: &str = include_str!("../../input/02.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

enum Colour {
    Blue,
    Green,
    Red,
}

struct Block {
    num: i32,
    colour: Colour,
}

fn parse_line(line: &str) -> (i32, Vec<Vec<Block>>) {
    let (game, blocks_str) = line.split_once(':').unwrap();
    let id = game.strip_prefix("Game ").unwrap().parse::<i32>().unwrap();
    let blocks = blocks_str
        .split(';')
        .map(|selection| {
            selection
                .trim()
                .split(',')
                .map(|block| {
                    let (num, c) = block.trim().split_once(' ').unwrap();
                    let num = num.parse::<i32>().unwrap();
                    let colour = match c.trim() {
                        "blue" => Colour::Blue,
                        "green" => Colour::Green,
                        "red" => Colour::Red,
                        _ => panic!("Unknown colour {c}"),
                    };
                    Block { num, colour }
                })
                .collect()
        })
        .collect();
    (id, blocks)
}

fn part_1(input: &str) -> i32 {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let (id, blocks) = parse_line(line);
            let valid = blocks.iter().all(|selection| {
                let (blue, green, red) =
                    selection
                        .iter()
                        .fold((0, 0, 0), |(blue, green, red), block| match block.colour {
                            Colour::Blue => (blue + block.num, green, red),
                            Colour::Green => (blue, green + block.num, red),
                            Colour::Red => (blue, green, red + block.num),
                        });
                blue <= 14 && green <= 13 && red <= 12
            });
            (id, valid)
        })
        .filter(|(_, valid)| *valid)
        .map(|(id, _)| id)
        .sum()
}

fn part_2(input: &str) -> i32 {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let (_, blocks) = parse_line(line);
            let (blue, green, red) =
                blocks
                    .iter()
                    .flatten()
                    .fold((0, 0, 0), |(blue, green, red), block| match block.colour {
                        Colour::Blue => (blue.max(block.num), green, red),
                        Colour::Green => (blue, green.max(block.num), red),
                        Colour::Red => (blue, green, red.max(block.num)),
                    });
            blue * green * red
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 8);
        assert_eq!(part_1(INPUT_TXT), 2600);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 2286);
        assert_eq!(part_2(INPUT_TXT), 86036);
    }
}
