use std::slice::Iter;
use std::iter::Iterator;

fn part1(earliest_depart: i128, busses: &[i128]) -> i128 {
    let bus0_earliest_depart = (busses[0] + 1) * (earliest_depart / busses[0]);
    let earliest_bus = busses
        .iter()
        .filter(|v| **v != 0)
        .map(|v| (v, v * ((earliest_depart / v) + 1)))
        .fold((busses[0], bus0_earliest_depart), |accum, (id, depart)| {
            if depart < accum.1 {
                (*id, depart)
            } else {
                accum
            }
        });

    earliest_bus.0 * (earliest_bus.1 - earliest_depart)

}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(residues: &[i128], modulii: &[i128]) -> Option<i128> {
    let prod = modulii.iter().product::<i128>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}


fn part2(busses: &[i128]) {
    let residues = busses
        .iter()
        .cloned()
        .enumerate()
        .filter(|(idx, v)| *v != 0)
        .map(|(idx, v)| (v - ((idx as i128) % v)))
        .collect::<Vec<i128>>();
    let modulii = busses
        .iter()
        .cloned()
        .filter(|v| *v != 0)
        .collect::<Vec<i128>>();

    dbg!(&modulii);
    dbg!(chinese_remainder(&residues, &modulii));
    
}

fn parse_input(input: &str) -> (i128, impl Iterator<Item=i128> + '_) {
    let mut lines = input.split("\n").filter(|l| !l.is_empty());
    let earliest_depart = lines.next().unwrap().parse::<i128>().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|v| {
            if v == "x" {
                "0"
            } else {
                v
            }
        })
        .map(|v| v.parse::<i128>().unwrap());
    
    (earliest_depart, busses)
}
    
fn main() {
    let mut input = parse_input(include_str!("input.txt"));
    let earliest_depart = input.0;
    let busses = input.1.collect::<Vec<i128>>();
    //println!("Part 1: {}",  part1(earliest_depart, &busses));
    
   part2(&busses); 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = parse_input(include_str!("test.txt"));
        let result = part1(input.0, &input.1.collect::<Vec<i128>>());
        assert_eq!(result, 295); 
    }
}
