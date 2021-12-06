const PIXELS: [&str; 3] = ["  ", "██", "  "];

fn main() {
    let input = include_str!("../../input/08.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn convert_to_layers(input: &str, pixels_wide: usize, pixels_tall: usize) -> Vec<String> {
    input
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .chunks(pixels_wide * pixels_tall)
        .map(|chunk| {
            chunk
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
}

fn layers_to_image(layers: Vec<String>) -> String {
    let mut image = vec!['2'; layers[0].len()];
    for layer in layers {
        for (i, c) in layer.chars().enumerate() {
            if image[i] == '2' {
                image[i] = c;
            }
        }
    }
    image
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
}
fn print_image(image: &str, pixels_wide: usize) {
    for line in image
        .chars()
        .map(|c| match c {
            '0' => PIXELS[0],
            '1' => PIXELS[1],
            '2' => PIXELS[2],
            _ => unreachable!(),
        })
        .collect::<Vec<&str>>()
        .chunks(pixels_wide)
    {
        println!("{}", line.join(""));
    }
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let image = convert_to_layers(input, 25, 6);
    if part_two {
        let layer = layers_to_image(image);
        print_image(&layer, 25);
        layer.parse().unwrap_or(0)
    } else {
        let layer = image
            .iter()
            .min_by(|x, y| {
                let zeros = &"0".to_string();
                x.matches(zeros).count().cmp(&y.matches(zeros).count())
            })
            .unwrap();
        (layer.matches(&"1").count() * layer.matches(&"2").count()) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "123456789012";
    static INPUT_2: &str = "0222112222120000";

    #[test]
    fn test_convert_to_layers() {
        let layers = convert_to_layers(INPUT, 3, 2);
        assert_eq!(layers[0], "123456");
        assert_eq!(layers[1], "789012");
    }

    #[test]
    fn test_layers_to_image() {
        assert_eq!(layers_to_image(convert_to_layers(INPUT_2, 2, 2)), "0110");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(run(include_str!("../../input/08.txt"), false), 1920);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(include_str!("../../input/08.txt"), true), 0);
    }
}
