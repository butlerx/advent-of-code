fn convert_to_layers(input: &str, pixels_wide: usize, pixels_tall: usize) -> Vec<Vec<String>> {
    input
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .chunks(pixels_wide * pixels_tall)
        .map(|chunk| {
            chunk
                .chunks(pixels_wide)
                .map(|sub_chunk| {
                    sub_chunk
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect()
        })
        .collect::<Vec<Vec<String>>>()
        .clone()
}

fn layers_to_image(layers: Vec<Vec<String>>) -> Vec<String> {
    let init = vec![vec!["0";layers[0][0].len()].join("");layers[0].len()];
    layers.fold(init, |final, layer| {
        layer
    })
}

pub fn run(input: &str, part_two: bool) -> i64 {
    if part_two {
        0
    } else {
        let layer = convert_to_layers(input, 25, 6)
            .iter()
            .map(|layer| layer.join(""))
            .min_by(|x, y| {
                let zeros = &"0".to_string();
                x.matches(zeros).count().cmp(&y.matches(zeros).count())
            })
            .unwrap();
        (layer.matches(&"1".to_string()).count() * layer.matches(&"2".to_string()).count()) as i64
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
        assert_eq!(layers[0][0], "123");
        assert_eq!(layers[0][1], "456");
        assert_eq!(layers[1][0], "789");
        assert_eq!(layers[1][1], "012");
    }

    #[test]
    fn test_layers_to_image() {
        let results = layers_to_image(convert_to_layers(INPUT_2, 2, 2));
        assert!(results[0] == "01");
        assert!(results[1] == "10");
    }

    #[test]
    fn test_part_1() {
        let results = run(include_str!("../input/day_8.txt"), false);
        println!("{}", results);
        assert!(results == 1920);
    }

    #[test]
    fn test_part_2() {
        let results = run(include_str!("../input/day_8.txt"), true);
        println!("{}", results);
        assert!(results == 0);
    }
}
