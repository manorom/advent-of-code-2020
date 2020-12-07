use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::iter::{from_fn, Iterator};

fn parse_bag_can_contain(input: &str) -> impl Iterator<Item = (String, u64, String)> + '_ {
    let idx = input.find("bags contain").unwrap();
    let color = input[..idx].trim().to_owned();
    let input = &input[idx + 13..];
    let contains_none = input.trim() == "no other bags.";
    let mut contained_bags = input.split(',');
    from_fn(move || {
        if contains_none {
            None
        } else {
            contained_bags.next().map(|s| {
                let mut pieces = s.trim().splitn(2, ' ');
                (
                    color.to_string(),
                    pieces.next().unwrap().parse::<u64>().unwrap(),
                    pieces.next()
                        .unwrap()
                        .trim()
                        .trim_end_matches('.')
                        .trim_end_matches("bags")
                        .trim_end_matches("bag")
                        .trim()
                        .to_string(),
                )
            })
        }
    })
}

fn find_bag_costs(map: &HashMap<String, HashMap<String, u64>>, color: &String) -> u64 {
    let mut cost = 1;
    if let Some(m) = map.get(color){
        for (k, v) in m.iter() {
            cost += v * find_bag_costs(map, k);
        }
    }

    cost
}

fn main() {
    let input = include_str!("input.txt");

    let mut contained_contains_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut contains_contained_map: HashMap<String, HashMap<String, u64>> = HashMap::new();

    for rules in input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| parse_bag_can_contain(l))
    {
        for (contains, num, contained) in rules {
            contained_contains_map
                .entry(contained.clone())
                .or_insert(HashSet::new())
                .insert(contains.clone());
            contains_contained_map
                .entry(contains)
                .or_insert(HashMap::new())
                .insert(contained, num);
        }
    }

    let mut reachable_bags = HashSet::new();
    let mut working_stack = vec!["shiny gold".to_owned()];
    while let Some(color) = working_stack.pop() {
        if let Some(containing) = contained_contains_map.get(&color) {
            for c in containing {
                reachable_bags.insert(c.clone());
                working_stack.push(c.clone());
            }
        }
    }

    println!("You can put your shiny gold bag in {} other bags", reachable_bags.len());

    dbg!(&contains_contained_map);
    println!("You need at least {} other bags", find_bag_costs(&contains_contained_map, &"shiny gold".to_owned()));
}
