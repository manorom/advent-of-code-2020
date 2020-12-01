use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::{Iterator, Peekable, from_fn};
use std::slice;

struct CartProdIter<'a, T> {
    base_it: slice::Iter<'a, T>,
    first_it: Peekable<slice::Iter<'a, T>>,
    second_it: Peekable<slice::Iter<'a, T>>,
    third_it: slice::Iter<'a, T>
}

impl<'a, T: Copy> CartProdIter<'a, T> {
    fn new(t: slice::Iter<'a, T>) -> Self {
        CartProdIter {
            base_it: t.clone(),
            first_it: t.clone().peekable(),
            second_it: t.clone().peekable(),
            third_it: t.clone()
        }
    }
}


impl<'a, T: Copy> Iterator for CartProdIter<'a, T> {
    type Item = (T,T,T);

    fn next(&mut self) -> Option<(T,T,T)> {
        let third = if let Some(t) = self.third_it.next() {
            t
        } else {
            self.second_it.next();
            self.third_it = self.base_it.clone();
            return self.next();
        };

        let second = if let Some(s) = self.second_it.peek() {
            s
        } else {
            self.first_it.next();
            self.second_it = self.base_it.clone().peekable();
            self.third_it = self.base_it.clone();
            return self.next();
        };

        let first = if let Some(f) = self.first_it.peek() {
            f
        } else {
            return None;
        };

        Some((**first, **second, *third))
    }
}


fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let input = BufReader::new(file)
        .lines()
        .map(|r| r.map(|line| line.parse::<i64>().unwrap()))
        .map(|rv| rv.unwrap())
        .collect::<Vec<_>>();

    let r = CartProdIter::new(input.iter()).find(|(f,s, t)| f + s == 2020);

    if let Some((f, s, _)) = r {
        println!("{} + {} = 2020\n{} * {} = {}\n", f, s, f, s, f * s)
    }

    let r2 = CartProdIter::new(input.iter()).find(|(f,s, t)| f + s + t == 2020);
    if let Some((f, s, t)) = r2 {
        println!("{} + {} + {} = 2020\n{} * {} * {} = {}", f, s, t, f, s, t, f * s * t);
    }
}
