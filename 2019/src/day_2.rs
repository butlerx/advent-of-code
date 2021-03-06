fn execute(memory: Vec<usize>, noun: usize, verb: usize) -> i64 {
    let mut nums = memory.clone();
    let mut pointer = 0;
    nums[1] = noun;
    nums[2] = verb;
    loop {
        match nums[pointer] {
            1 => {
                let (arg_1, arg_2, pos) = (nums[pointer + 1], nums[pointer + 2], nums[pointer + 3]);
                nums[pos] = nums[arg_1] + nums[arg_2];
            }
            2 => {
                let (arg_1, arg_2, pos) = (nums[pointer + 1], nums[pointer + 2], nums[pointer + 3]);
                nums[pos] = nums[arg_1] * nums[arg_2];
            }
            99 => return nums[0] as i64,
            _ => unreachable!(),
        }
        pointer += 4;
    }
}
pub fn run(input: &str, part_two: bool) -> i64 {
    if part_two {
        let mem: Vec<usize> = input
            .split(",")
            .map(|num| num.trim().parse().unwrap())
            .collect();
        let goal = 19690720;
        let (mut noun, mut verb) = (0, 0);
        for n in 0..99 {
            for v in 0..99 {
                let test = execute(mem.clone(), n, v);
                if test == goal {
                    noun = n;
                    verb = v;
                    break;
                }
            }
        }
        (100 * noun + verb) as i64
    } else {
        execute(
            input
                .split(",")
                .map(|num| num.trim().parse().unwrap())
                .collect(),
            12,
            2,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "1,9,10,3,2,3,11,0,99,30,40,50,30";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 1600);
        let results = run(include_str!("../input/day_2.txt"), false);
        println!("{}", results);
        assert!(results == 3850704);
    }

    #[test]
    fn test_part_2() {
        let results = run(include_str!("../input/day_2.txt"), true);
        println!("{}", results);
        assert!(results == 6718);
    }
}
