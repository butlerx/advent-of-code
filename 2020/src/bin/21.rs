use std::collections::{hash_map::Entry, HashMap, HashSet};

type Allergens = HashMap<String, HashSet<String>>;
type Ingredients = (Allergens, HashMap<String, usize>);

fn main() {
    let (allergens, occurrences) = parse_input(include_str!("../../input/21.txt"));
    println!("Part 1: {}", part_1(occurrences));
    println!(
        "Part 2: {}",
        find_danderous_ingrediants(allergens).join(",")
    );
}

fn parse_input(input: &str) -> Ingredients {
    let mut allergen_map = HashMap::<String, HashSet<String>>::new();
    let mut occurrences = HashMap::<String, usize>::new();
    input.lines().for_each(|line| {
        let contains = line.find(" (contains ").unwrap();
        let ingredients = line[..contains]
            .split(' ')
            .map(String::from)
            .collect::<HashSet<_>>();
        let allergens = line[(contains + 11)..(line.len() - 1)]
            .split(", ")
            .map(String::from)
            .collect::<Vec<_>>();
        for allergen in &allergens {
            match allergen_map.entry(allergen.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(ingredients.clone());
                }
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() = entry.get().intersection(&ingredients).cloned().collect();
                }
            }
        }

        for ingredient in &ingredients {
            *occurrences.entry(ingredient.clone()).or_insert(0) += 1;
        }
    });
    for ingredients in allergen_map.values() {
        for ingredient in ingredients {
            occurrences.remove(ingredient);
        }
    }

    (allergen_map, occurrences)
}

fn find_danderous_ingrediants(mut allergens: Allergens) -> Vec<String> {
    let mut danger_list = Vec::<(String, String)>::new();
    loop {
        if allergens.is_empty() {
            danger_list.sort_by_key(|(allergen, _)| allergen.clone());
            break danger_list
                .iter()
                .map(|(_, ingredient)| ingredient.clone())
                .collect();
        }
        for (allergen, ingredient_set) in &allergens {
            if ingredient_set.len() == 1 {
                let ingredient = ingredient_set.iter().next().unwrap();
                danger_list.push((allergen.clone(), ingredient.clone()));
            }
        }

        for (allergen, _) in &danger_list {
            allergens.remove(allergen);
        }

        for ingredient_set in allergens.values_mut() {
            for (_, ingredient) in &danger_list {
                ingredient_set.remove(ingredient);
            }
        }
    }
}

fn part_1(occurrences: HashMap<String, usize>) -> i64 {
    occurrences
        .values()
        .fold(0, |sum, occurrence| sum + *occurrence as i64)
}

#[cfg(test)]
mod day_21_tests {
    use super::*;
    static INPUT: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_part_1() {
        let (_, o) = parse_input(INPUT);
        assert_eq!(part_1(o), 5);
        let (_, occurrences) = parse_input(include_str!("../../input/21.txt"));
        assert_eq!(part_1(occurrences), 2779);
    }

    #[test]
    fn test_part_2() {
        let (allergens, _) = parse_input(INPUT);
        assert_eq!(
            find_danderous_ingrediants(allergens).join(","),
            "mxmxvkd,sqjhc,fvjkl"
        );
        let (allergens_2, _) = parse_input(include_str!("../../input/21.txt"));
        assert_eq!(
            find_danderous_ingrediants(allergens_2).join(","),
            "lkv,lfcppl,jhsrjlj,jrhvk,zkls,qjltjd,xslr,rfpbpn"
        );
    }
}
