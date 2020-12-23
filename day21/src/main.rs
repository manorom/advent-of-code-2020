use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

fn parse_line(line: &str) -> (HashSet<String>, HashSet<String>) {
    let mut pieces = line
        .split(|c| c == ' ' || c == ',' || c == ')' || c == '(')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim());
    let ingredients = pieces
        .by_ref()
        .take_while(|s| *s != "contains")
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();
    let allergens = pieces.map(|s| s.to_string()).collect::<HashSet<_>>();

    (ingredients, allergens)
}

fn parse_input(input: &str) -> impl Iterator<Item = (HashSet<String>, HashSet<String>)> + '_ {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| parse_line(l))
}

fn main() {
    let mut ingredients_allergens = parse_input(include_str!("input.txt")).collect::<Vec<_>>();

    let all_allergens = ingredients_allergens
        .iter()
        .map(|(_, a)| a.iter())
        .flatten()
        .map(|s| s.to_owned())
        .collect::<HashSet<_>>();
    let all_ingredients = ingredients_allergens
        .iter()
        .map(|(i, _)| i.iter())
        .flatten()
        .map(|s| s.to_owned())
        .collect::<HashSet<_>>();

    let mut ingredients_contain_allergens = HashSet::new();
    for ingredient in all_ingredients.iter() {
        for allergen in all_allergens.iter() {
            if ingredients_allergens
                .iter()
                .filter(|(_iset, aset)| aset.contains(allergen))
                .map(|(iset, _aset)| iset.contains(ingredient))
                .all(|b| b)
            {
                ingredients_contain_allergens.insert(ingredient.to_owned());
            }
        }
    }

    let ingredients_without_allergens = all_ingredients
        .difference(&ingredients_contain_allergens)
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    let num_use: usize = ingredients_without_allergens
        .iter()
        .map(|i| {
            ingredients_allergens
                .iter()
                .map(|(iset, _)| iset.contains(i))
                .filter(|b| *b)
                .count()
        })
        .sum();
    println!(
        "Number of products that contain ingredients without allergens {}",
        num_use
    );

    for (iset, aset) in ingredients_allergens.iter_mut() {
        for i in ingredients_without_allergens.iter() {
            iset.remove(i);
        }
    }

    let mut possibly_allergenic: HashMap<String, HashSet<String>> = HashMap::new();

    for allergen in all_allergens.iter() {
        for (iset, aset) in ingredients_allergens.iter() {
            if aset.contains(allergen) {
                if possibly_allergenic.contains_key(allergen) {
                    possibly_allergenic.insert(
                        allergen.to_owned(),
                        possibly_allergenic[allergen]
                            .intersection(iset)
                            .cloned()
                            .collect::<HashSet<_>>(),
                    );
                } else {
                    possibly_allergenic.insert(allergen.to_owned(), iset.clone());
                }
            }
        }
    }

    let mut identified_allergenic: HashMap<String, String> = possibly_allergenic
        .iter()
        .filter(|(a, iset)| iset.len() == 1)
        .map(|(a, iset)| (a.clone(), iset.iter().next().unwrap().clone()))
        .collect::<HashMap<_, _>>();

    while possibly_allergenic
        .iter()
        .filter(|(_, iset)| iset.len() > 1)
        .count()
        != 0
    {
        for (_, identified) in identified_allergenic.iter() {
            for (a, iset) in possibly_allergenic.iter_mut() {
                iset.remove(identified);
            }
        }

        for (a, iset) in possibly_allergenic.iter() {
            if iset.len() == 1 {
                identified_allergenic.insert(a.clone(), iset.iter().next().unwrap().clone());
            }
        }
        for a in identified_allergenic.keys() {
            possibly_allergenic.remove(a);
        }
    }

    let mut dangerous_ingredients = identified_allergenic.iter().collect::<Vec<_>>();
    dangerous_ingredients.sort_by_key(|(a, i)| *a);
    let canocial_dangerous_ingredients = dangerous_ingredients
        .into_iter()
        .map(|(a, i)| i)
        .cloned()
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "Canonical dangerous ingredient list: {}",
        canocial_dangerous_ingredients
    );
}
