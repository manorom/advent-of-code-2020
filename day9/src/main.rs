use std::collections::HashSet;

fn check_window(window: &[u64], preamble_size: usize) -> Option<u64> {

    if !window
        .iter()
        .take(preamble_size)
        .map(|i| window.iter().take(preamble_size).map(move |j| i + j))
        .flatten()
        .any(|i| i == window[preamble_size]) {
        Some(window[preamble_size])
    } else {
        None
    }
}

fn find_block(input: &Vec<u64>, wrong_number: u64) -> u64 {
    for window_size in 2..(input.len()) {
        for win in input.windows(window_size) {
            if win.iter().sum::<u64>() == wrong_number {
                return win.iter().min().unwrap() + win.iter().max().unwrap();
            }
        }
    }
    0
}

fn main() {
    let input = include_str!("input.txt");
    let code = input.split('\n').filter(|l| !l.is_empty()).map(|l| l.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let first_wrong_number = code.windows(26).map(|s| check_window(s, 25)).filter_map(|i| i).next().unwrap();
    println!("first wrong number: {}", first_wrong_number);
    println!("Block sum: {}", find_block(&code, first_wrong_number));
}
