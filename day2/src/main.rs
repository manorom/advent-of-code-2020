use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_input_line(line: &str) -> (u64, u64, char, String) {
    let mut r = line.split(|c| c == '-' || c == ' ');
    let from = r.next().unwrap().parse::<u64>().unwrap();
    let to =   r.next().unwrap().parse::<u64>().unwrap();
    let c = r.next().unwrap().chars().next().unwrap();
    let password = r.next().unwrap().to_string();
    (from, to, c, password)
}

fn check_password_part1(from: u64, to: u64, c: char, password: &str) -> bool {
    password.chars().fold((from == 0, 0), |(pred, num), v| {
        if v == c {
            (num+1 >= from && num+1 <= to, num+1)
        } else {
            (pred, num)
        }
    }).0
}

fn check_password_part2(pos1: u64, pos2: u64, c: char, password: &str) -> bool {
    let idx1 = (pos1 - 1) as usize;
    let idx2 = (pos2 - 1) as usize - idx1 - 1;
    let mut password_chars = password.chars();
   (password_chars.nth(idx1).unwrap() == c) ^ (password_chars.nth(idx2).unwrap() == c)
}

fn main() {
    let input_file = File::open("input.txt").expect("Could not open file");
    let passwords = BufReader::new(input_file)
        .lines()
        .map(|r| r.unwrap())
        .map(|l| parse_input_line(&l)).collect::<Vec<_>>();
    
    let num_wrong_passwords_p1 = passwords
        .iter()
        .map(|(from, to, c, password)| check_password_part1(*from, *to, *c, &password))
        .filter(|b| *b)
        .count();
    println!("Number of wrong passwords according to policy 1: {}", num_wrong_passwords_p1);

    let num_wrong_passwords_p2 = passwords
        .iter()
        .map(|(pos1, pos2, c, password)| check_password_part2(*pos1, *pos2, *c, &password))
        .filter(|b| *b)
        .count(); 
    println!("Number of wrong passwords according to policy 2: {}", num_wrong_passwords_p2);
}
