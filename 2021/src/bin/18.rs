use itertools::Itertools;
use std::ops::Add;

fn main() {
    let input = include_str!("../../input/18.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

type Exploded = Option<(Option<i64>, Option<i64>)>;

#[derive(Clone)]
enum Num {
    Nested(Box<Num>, Box<Num>),
    Value(i64),
}

impl Add for Num {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Nested(Box::new(self), Box::new(other)).reduce()
    }
}

fn part_1(input: &str) -> i64 {
    input
        .lines()
        .map(Num::new)
        .reduce(|l, r| l + r)
        .unwrap()
        .magnitude()
}

fn part_2(input: &str) -> i64 {
    input
        .lines()
        .map(Num::new)
        .permutations(2)
        .map(|permutation| {
            permutation
                .into_iter()
                .reduce(|l, r| l + r)
                .unwrap()
                .magnitude()
        })
        .max()
        .unwrap()
}

impl Num {
    fn new(input: &str) -> Num {
        if let Ok(a) = input.parse::<i64>() {
            Num::Value(a)
        } else {
            let (position, _) =
                input
                    .chars()
                    .enumerate()
                    .fold((0, 0), |(position, open), (pos, c)| match c {
                        '[' => (position, open + 1),
                        ']' => (position, open - 1),
                        ',' if open == 1 => (pos, open),
                        _ => (position, open),
                    });
            let pos_1 = position + 1;
            let len = input.chars().count() - 1;
            Num::new(&input[1..position]) + Num::new(&input[pos_1..len])
        }
    }

    fn split(self) -> (Self, bool) {
        match self {
            Num::Nested(a, b) => {
                let (a_split, a_was_split) = a.split();
                if !a_was_split {
                    let (b_split, b_was_split) = b.split();
                    (
                        Self::Nested(Box::new(a_split), Box::new(b_split)),
                        b_was_split,
                    )
                } else {
                    (Self::Nested(Box::new(a_split), b), true)
                }
            }
            Num::Value(a) if a >= 10 => (
                Num::Nested(
                    Box::new(Num::Value(a / 2)),
                    Box::new(Num::Value(a / 2 + a % 2)),
                ),
                true,
            ),
            Num::Value(_) => (self, false),
        }
    }

    fn add_to(self, right: bool, inc: i64) -> Self {
        match self {
            Num::Nested(a, b) => {
                if right {
                    Num::Nested(a, Box::new(b.add_to(right, inc)))
                } else {
                    Num::Nested(Box::new(a.add_to(right, inc)), b)
                }
            }
            Num::Value(a) => Num::Value(a + inc),
        }
    }

    fn explode(self, depth: usize) -> (Self, Exploded) {
        match self {
            Self::Value(_) => (self, None),
            Self::Nested(l, r) => match (*l, *r) {
                (Self::Value(nl), Self::Value(nr)) if depth >= 4 => {
                    (Self::Value(0), Some((Some(nl), Some(nr))))
                }
                (l, r) => match l.explode(depth + 1) {
                    (l_reduced, Some((explode_left, explode_right))) => (
                        Self::Nested(
                            Box::new(l_reduced),
                            Box::new(if let Some(explode_right) = explode_right {
                                r.add_to(false, explode_right)
                            } else {
                                r
                            }),
                        ),
                        Some((explode_left, None)),
                    ),
                    (l_reduced, None) => match r.explode(depth + 1) {
                        (r_reduced, Some((explode_left, explode_right))) => (
                            Self::Nested(
                                Box::new(if let Some(explode_left) = explode_left {
                                    l_reduced.add_to(true, explode_left)
                                } else {
                                    l_reduced
                                }),
                                Box::new(r_reduced),
                            ),
                            Some((None, explode_right)),
                        ),
                        (r_reduced, None) => {
                            (Self::Nested(Box::new(l_reduced), Box::new(r_reduced)), None)
                        }
                    },
                },
            },
        }
    }

    fn reduce(self) -> Self {
        let mut number = self;
        loop {
            match number.explode(0) {
                (n, None) => match n.split() {
                    (n, false) => break n,
                    (n, _) => number = n,
                },
                (n, _) => number = n,
            }
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            Num::Nested(a, b) => (3 * a.magnitude()) + (2 * b.magnitude()),
            Num::Value(a) => *a,
        }
    }
}

#[cfg(test)]
mod day_18_tests {
    use super::*;
    static INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 4140);
        assert_eq!(part_1(include_str!("../../input/18.txt")), 4008);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 3993);
        assert_eq!(part_2(include_str!("../../input/18.txt")), 4667);
    }
}
