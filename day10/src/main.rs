use std::iter::Iterator;
use std::slice::Iter;

fn part2(mut iter: Iter<i64>, len: usize) -> i64 {
    let mut dynprog = vec![0; len];
    dynprog.push(0);
    dynprog[0] = 1;

    for &i in iter {
        let i = i as usize;
        if i <= 1 {
            dynprog[i] = dynprog[i -1];
        } else if i <= 2 {
            dynprog[i] = dynprog[i-1] + dynprog[i-2];
        } else {
            dynprog[i] = dynprog[i-1] + dynprog[i-2] + dynprog[i-3];
        }
    }

    return *dynprog.last().unwrap();
}

fn main() {
    let mut input = include_str!("input.txt").split('\n').filter(|l| !l.is_empty()).map(|l| l.parse::<i64>().unwrap()).collect::<Vec<_>>();

    input.sort_unstable();
    let max_joltage = input.last().unwrap() + 3;
    
    let diffs = input.windows(2).fold((1,1), |accum, v| {
        if (v[1] - v[0]).abs() == 1 {
            (accum.0 + 1, accum.1)
        }
        else if (v[1] - v[0]).abs() == 3 {
            (accum.0, accum.1 + 1)
        } else {
            accum
        }
    });
    
    for i in input.windows(2) {
        if (i[0] - i[1]).abs() == 3 {
            println!("Pair {},{} is inseperable", i[0], i[1]);
        } else {
            println!("Pair {},{} is seperable", i[0], i[1]);
        }
    }

    input.push(max_joltage);
    println!("Product of number of differences: {}", diffs.0 * diffs.1);
    println!("Number of viable combinations: {}", part2(input.iter(), max_joltage as usize));

}
