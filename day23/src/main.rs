use std::iter::successors;

fn parse(input: &str) -> Vec<i64> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64 - 1)
        .collect::<Vec<i64>>()
}

fn play(input: Vec<i64>, rounds: usize, num_cups: usize) -> Vec<i64> {
    let mut cups: Vec<i64> = vec![0; num_cups];
    for i in 0..num_cups {
        cups[i] = (i + 1) as i64;
    }
    let mut cur_cup = (cups.len() - 1) as i64;
    for i in input.iter() {
        cups[cur_cup as usize] = *i as i64;
        cur_cup = *i as i64;
    }

    if input.len() < num_cups {
        cups[cur_cup as usize] = input.len() as i64;
    } else {
        cups[cur_cup as usize] = input[0];
    }

    cur_cup = input[0];

    for _ in 0..rounds {
        let picked_cups = successors(Some(cur_cup), |c| Some(cups[*c as usize]))
            .skip(1)
            .take(3)
            .collect::<Vec<i64>>();
        let mut dest_cup = (cur_cup - 1).rem_euclid(num_cups as i64);
        while picked_cups.iter().any(|c| *c == dest_cup) {
            dest_cup = (dest_cup - 1).rem_euclid(num_cups as i64);
        }

        let picked_cups_next = cups[picked_cups[2] as usize];
        // move the selected cups after the destination
        let dest_cup_next = cups[dest_cup as usize];
        cups[dest_cup as usize] = cups[cur_cup as usize];
        // move everything after the destination cup behind the picked cups
        cups[picked_cups[2] as usize] = dest_cup_next;
        cups[cur_cup as usize] = picked_cups_next;
        cur_cup = cups[cur_cup as usize];
    }

    cups
}

fn main() {
    let input = parse("598162734");

    let final_cups_part1 = play(input.clone(), 100, input.len());
    println!(
        "Final pick of 10 cups after 100 rounds: {}",
        successors(Some(final_cups_part1[0]), |c| Some(
            final_cups_part1[*c as usize]
        ))
        .map(|i| format!("{}", i + 1))
        .take(input.len() - 1)
        .collect::<Vec<_>>()
        .join(" ")
    );
    let final_cups_part2 = play(input, 10_000_000, 1_000_000);
    println!(
        "Multiple of labels next to 1 after 10_000_000 rounds: {}",
        successors(Some(final_cups_part2[0]), |c| Some(
            final_cups_part2[*c as usize]
        ))
        .take(2)
        .map(|c| c + 1)
        .product::<i64>()
    );
}
