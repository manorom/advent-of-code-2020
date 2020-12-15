use std::collections::HashMap;
use std::iter::Iterator;

fn parse_input(input: &str) -> impl Iterator<Item=(usize, u64)>  + '_ {
    input
        .split(|c| c == ',' || c == '\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<u64>().unwrap())
        .enumerate()
        .map(|(idx, i)| (idx + 1, i))
}


fn main() {
    let input = include_str!("input.txt");
    let init = parse_input(input);
    let mut track = HashMap::new();

    let mut start_round = 0;
    for (idx, i) in init {
        start_round = idx;
        track.insert(i, idx);
    }

    let mut spoken_number = 0;

    for round in start_round + 2..=30000000 {
        let next_spoken_number;
        if let Some(prev_spoken) = track.get(&spoken_number) {
            next_spoken_number = (round - 1 - prev_spoken) as u64;
        } else {
            next_spoken_number = 0;
        }
        track.insert(spoken_number, round - 1);
        spoken_number = next_spoken_number;
    }

    println!("2020th number is {}", spoken_number);
}
