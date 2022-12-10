static INPUT_TXT: &str = include_str!("../../input/10.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: \n{}", part_2(INPUT_TXT));
}

fn part_1(input: &str) -> i64 {
    let cycles = input
        .trim()
        .lines()
        .fold(vec![1], |mut cycles, line| match line {
            "noop" => {
                cycles.push(cycles[cycles.len() - 1]);
                cycles
            }
            _ => {
                cycles.push(cycles[cycles.len() - 1]);
                let num = line.split(' ').nth(1).unwrap().parse::<i64>().unwrap();
                cycles.push(cycles[cycles.len() - 1] + num);
                cycles
            }
        });

    vec![20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|cycle| cycle * cycles[(cycle as usize) - 1])
        .sum()
}

fn part_2(input: &str) -> String {
    input
        .trim()
        .lines()
        .fold(
            (1, 1, String::new()),
            |(cycle, register, mut display), line| match line {
                "noop" => {
                    output_to_display(&mut display, register, cycle);
                    (cycle + 1, register, display)
                }
                _ => {
                    output_to_display(&mut display, register, cycle);
                    output_to_display(&mut display, register, cycle + 1);
                    let num = line.split(' ').nth(1).unwrap().parse::<i64>().unwrap();
                    (cycle + 2, register + num, display)
                }
            },
        )
        .2
}

fn output_to_display(display: &mut String, register: i64, cycle: i64) {
    let cycle_wrapped = (cycle - 1) % 40;
    let display_char = if (register - cycle_wrapped).abs() <= 1 {
        '#'
    } else {
        ' '
    };
    display.push(display_char);
    if cycle_wrapped == 39 {
        display.push('\n');
    }
}

#[cfg(test)]
mod day_10_tests {

    use super::*;
    static INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 13140);
        assert_eq!(part_1(INPUT_TXT), 16060);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(INPUT),
            "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     
"
        );
        assert_eq!(
            part_2(INPUT_TXT),
            "###   ##   ##  #### #  # #    #  # #### 
#  # #  # #  # #    # #  #    #  # #    
###  #  # #    ###  ##   #    #### ###  
#  # #### #    #    # #  #    #  # #    
#  # #  # #  # #    # #  #    #  # #    
###  #  #  ##  #### #  # #### #  # #    
"
        );
    }
}
